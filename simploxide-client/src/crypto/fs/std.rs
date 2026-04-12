use std::{
    io::{Read, Seek as _, SeekFrom, Write},
    path::Path,
};

use super::{
    EncryptedFileOps, EncryptedFileState, FileCryptoArgs, InvalidAuthTag, Mode, PlainFileOps,
    SimplexSecretBox,
};

/// Sync wrapper over file with SimpleX-SecretBox encryption.
///
/// # Security
///
/// - All bytes returned from `read()` are unauthenticated until the file is fully read. The caller
///   must never act on streamed content until `read()` has returned `Ok(0)`. If reading a file
///   returns Err() all previoulsy read data cannot be trusted and must be discarded.
///
/// - The caller is responsible to call [put_auth_tag] manually. The `Drop` implementation does its best
///   to write the authentication tag but it can silently fail leaving the file unauthenticated.
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
    /// [Self::prepare_for_overwrite] to work efficiently. Use [open_read_only] if it is important
    /// for your use-case
    pub fn open<P: AsRef<Path>>(path: P, crypto_args: FileCryptoArgs) -> std::io::Result<Self> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(false)
            .open(path)?;

        let size = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::from_size_and_args(size, crypto_args)?),
        })
    }

    /// Opens file in a read only mode, shouldn't be used with [prepare_for_overwrite] as all
    /// writes will return IO errors.
    pub fn open_read_only<P: AsRef<Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> std::io::Result<Self> {
        let mut file = std::fs::OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path)?;

        let size = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;

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

impl PlainFileOps for ::std::fs::File {
    fn open<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<Self> {
        ::std::fs::OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path)
    }

    fn create<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<Self> {
        ::std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
    }
}

impl<S: SimplexSecretBox> EncryptedFileOps for EncryptedFile<S> {
    fn open<P: AsRef<::std::path::Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> ::std::io::Result<Self> {
        Self::open(path, crypto_args)
    }

    fn create<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<Self> {
        Self::create(path)
    }

    fn create_with_args<P: AsRef<::std::path::Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> ::std::io::Result<Self> {
        Self::create_with_args(path, crypto_args)
    }

    fn crypto_args(&self) -> &FileCryptoArgs {
        self.crypto_args()
    }
}
