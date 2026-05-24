//! Sync version of SimpleX encrypted files

use simploxide_api_types::CryptoFile as SxcCryptoFile;

use std::{
    io::{Read, Seek as _, SeekFrom, Write},
    path::Path,
};

use super::{EncryptedFileState, FileCryptoArgs, InvalidAuthTag, Mode, SimplexSecretBox};

/// Sync wrapper over a file with SimpleX-SecretBox encryption.
///
/// # Security
///
/// - All bytes returned from `read()` are unauthenticated until the file is fully read. The caller
///   must never act on streamed content until `read()` has returned `Ok(0)`. If reading a file
///   returns Err() all previously read data cannot be trusted and must be discarded.
///
/// - The caller is responsible to call [`Self::put_auth_tag`] manually. The `Drop` implementation does
///   its best to write the authentication tag but it can silently fail leaving the file
///   unauthenticated.
pub struct EncryptedFile<S: SimplexSecretBox> {
    file: ::std::fs::File,
    state: Box<EncryptedFileState<S>>,
}

impl<S: SimplexSecretBox> EncryptedFile<S> {
    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        Ok(Self {
            file: std::fs::File::create(path)?,
            state: Box::new(EncryptedFileState::new()),
        })
    }

    pub fn create_with_args<P: AsRef<Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> std::io::Result<Self> {
        Ok(Self {
            file: std::fs::File::create(path)?,
            state: Box::new(EncryptedFileState::from_args(crypto_args)),
        })
    }

    /// Note: this call requires write permissions on the file system for
    /// [`Self::prepare_for_overwrite`] to work. Use [`Self::open_read_only`] when write access is
    /// not needed or not available.
    pub fn open<P: AsRef<Path>>(path: P, crypto_args: FileCryptoArgs) -> std::io::Result<Self> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(false)
            .open(path)?;

        let size = size_hint(&mut file)?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::from_size_and_args(size, crypto_args)?),
        })
    }

    /// Opens file in a read-only mode. [`Self::prepare_for_overwrite`] will return an IO error.
    pub fn open_read_only<P: AsRef<Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> std::io::Result<Self> {
        let mut file = std::fs::OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path)?;

        let size = size_hint(&mut file)?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::from_size_and_args(size, crypto_args)?),
        })
    }

    pub fn prepare_for_overwrite(&mut self) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.set_len(0)?;
        self.state.reset();
        self.state.mode = Mode::Write;

        Ok(())
    }

    pub fn crypto_args(&self) -> &FileCryptoArgs {
        self.state.crypto_args()
    }

    pub fn optimal_buf_size(&self) -> usize {
        self.state.optimal_buf_size()
    }

    pub fn plaintext_size_hint(&self) -> usize {
        self.state.plaintext_size_hint()
    }

    /// Does nothing if auth tag was already written
    pub fn put_auth_tag(mut self) -> std::io::Result<()> {
        if self.state.mode == Mode::Read {
            return self.state.assert_writable();
        } else if self.state.mode == Mode::Write {
            self.state.mode = Mode::Auth;
            let tag = self.state.secret_box.auth_tag();
            self.file.write_all(&tag)?;
        } else if self.state.mode == Mode::AuthFailure {
            return Err(InvalidAuthTag::io_error());
        }

        Ok(())
    }
}

impl<S: SimplexSecretBox> Write for EncryptedFile<S> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.state.assert_writable()?;
        let encrypted = self.state.encrypt_chunk(buf);
        self.file.write_all(encrypted)?;
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.write(buf).map(drop)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl<S: SimplexSecretBox> Read for EncryptedFile<S> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.state.mode == Mode::AuthFailure {
            return Err(InvalidAuthTag::io_error());
        }

        if self.state.mode == Mode::Auth {
            if self.state.is_all_data_read() {
                return Ok(0);
            } else {
                self.file
                    .read_exact(self.state.auth_tag_buf())
                    .map_err(|_| InvalidAuthTag::io_error())?;
                self.state.authenticate()?;
                return Ok(0);
            }
        }

        self.state.assert_readable()?;

        if buf.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "reader got exhausted before EOF: the data cannot be authenticated",
            ));
        }

        let read_buf = self.state.prep_read_buf(buf.len());
        let bytes_read = self.file.read(read_buf)?;

        self.state.decrypt_read_buf(bytes_read, buf);

        if self.state.is_all_data_read() {
            self.state.switch_to_auth_mode();
        } else if bytes_read == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "file truncated before ciphertext end",
            ));
        }

        Ok(bytes_read)
    }
}

impl<S: SimplexSecretBox> Drop for EncryptedFile<S> {
    fn drop(&mut self) {
        if self.state.mode == Mode::Write {
            let tag = self.state.secret_box.auth_tag();
            if let Err(e) = self.file.write_all(&tag) {
                log::error!("Failed to authenticate a file: {e}");
            }
        }
    }
}

/// A sync file that is either plaintext or SimpleX-SecretBox encrypted.
pub enum StdMaybeCryptoFile<S: SimplexSecretBox> {
    Plain(::std::fs::File),
    Encrypted(EncryptedFile<S>),
}

impl<S: SimplexSecretBox> StdMaybeCryptoFile<S> {
    /// Opens the file read+write so that [`Self::prepare_for_overwrite`] works.
    /// Use [`Self::open_read_only`] when write access is not needed or not available.
    pub fn open<P: AsRef<Path>>(
        path: P,
        crypto_args: Option<FileCryptoArgs>,
    ) -> std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(EncryptedFile::open(path, args)?)),
            None => Ok(Self::Plain(
                std::fs::OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(false)
                    .open(path)?,
            )),
        }
    }

    pub fn open_read_only<P: AsRef<Path>>(
        path: P,
        crypto_args: Option<FileCryptoArgs>,
    ) -> std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(EncryptedFile::open_read_only(path, args)?)),
            None => Ok(Self::Plain(
                std::fs::OpenOptions::new()
                    .write(false)
                    .read(true)
                    .create(false)
                    .open(path)?,
            )),
        }
    }

    pub fn create<P: AsRef<Path>>(
        path: P,
        crypto_args: Option<FileCryptoArgs>,
    ) -> std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(EncryptedFile::create_with_args(
                path, args,
            )?)),
            None => Ok(Self::Plain(
                std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?,
            )),
        }
    }

    pub fn from_crypto_file(crypto_file: SxcCryptoFile) -> std::io::Result<Self> {
        match crypto_file.crypto_args {
            Some(args) => {
                let crypto_args = FileCryptoArgs::try_from(args)?;
                Self::open(&crypto_file.file_path, Some(crypto_args))
            }
            None => Self::open(&crypto_file.file_path, None),
        }
    }

    pub fn from_crypto_file_read_only(crypto_file: SxcCryptoFile) -> std::io::Result<Self> {
        match crypto_file.crypto_args {
            Some(args) => {
                let crypto_args = FileCryptoArgs::try_from(args)?;
                Self::open_read_only(&crypto_file.file_path, Some(crypto_args))
            }
            None => Self::open_read_only(&crypto_file.file_path, None),
        }
    }

    pub fn size_hint(&mut self) -> std::io::Result<usize> {
        match self {
            Self::Plain(f) => size_hint(f),
            Self::Encrypted(f) => Ok(f.plaintext_size_hint()),
        }
    }

    pub fn crypto_args(&self) -> Option<&FileCryptoArgs> {
        match self {
            Self::Plain(_) => None,
            Self::Encrypted(f) => Some(f.crypto_args()),
        }
    }

    pub fn prepare_for_overwrite(&mut self) -> std::io::Result<()> {
        match self {
            Self::Plain(f) => {
                f.seek(SeekFrom::Start(0))?;
                f.set_len(0)?;
                Ok(())
            }
            Self::Encrypted(f) => f.prepare_for_overwrite(),
        }
    }

    /// Writes the AEAD auth tag for encrypted files; no-op for plain files.
    pub fn put_auth_tag(self) -> std::io::Result<()> {
        match self {
            Self::Plain(_) => Ok(()),
            Self::Encrypted(f) => f.put_auth_tag(),
        }
    }
}

impl<S: SimplexSecretBox> Read for StdMaybeCryptoFile<S> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Plain(f) => f.read(buf),
            Self::Encrypted(e) => e.read(buf),
        }
    }
}

impl<S: SimplexSecretBox> Write for StdMaybeCryptoFile<S> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Plain(f) => f.write(buf),
            Self::Encrypted(e) => e.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Plain(f) => f.flush(),
            Self::Encrypted(e) => e.flush(),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Self::Plain(f) => f.write_all(buf),
            Self::Encrypted(e) => e.write_all(buf),
        }
    }
}

fn size_hint(file: &mut ::std::fs::File) -> ::std::io::Result<usize> {
    let size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    crate::util::cast_file_size(size)
}
