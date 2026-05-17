use simploxide_api_types::CryptoFile as SxcCryptoFile;
use tokio::io::{AsyncRead, AsyncSeekExt as _, AsyncWrite, AsyncWriteExt as _};

use std::{io::SeekFrom, path::Path, pin::Pin, task::Poll};

use super::{EncryptedFileState, FileCryptoArgs, InvalidAuthTag, Mode, SimplexSecretBox};

/// Async wrapper over a file with SimpleX-SecretBox encryption.
///
/// # Security
///
/// - All bytes returned from `read()` are unauthenticated until the file is fully read. The caller
///   must never act on streamed content until `read()` has returned `Ok(0)`. If reading a file
///   returns Err() all previously read data cannot be trusted and must be discarded.
///
/// - The caller is responsible to call [`Self::put_auth_tag`] manually. The `AsyncWrite` implementation
///   does its best to write the authentication tag but it can silently fail leaving the file
///   unauthenticated.
pub struct EncryptedFile<S: SimplexSecretBox> {
    file: ::tokio::fs::File,
    state: Box<EncryptedFileState<S>>,
}

impl<S: SimplexSecretBox> EncryptedFile<S> {
    pub async fn create<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = tokio::fs::File::create(path).await?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::new()),
        })
    }

    pub async fn create_with_args<P: AsRef<Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> std::io::Result<Self> {
        let file = tokio::fs::File::create(path).await?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::from_args(crypto_args)),
        })
    }

    /// Note: this call requires write permissions on the file system for
    /// [`Self::prepare_for_overwrite`] to work. Use [`Self::open_read_only`] when write access is
    /// not needed or not available.
    pub async fn open<P: AsRef<Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> std::io::Result<Self> {
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(false)
            .open(path)
            .await?;

        let size = size_hint(&mut file).await?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::from_size_and_args(size, crypto_args)?),
        })
    }

    /// Opens file in a read-only mode. [`Self::prepare_for_overwrite`] will return an IO error.
    pub async fn open_read_only<P: AsRef<Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> std::io::Result<Self> {
        let mut file = tokio::fs::OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path)
            .await?;

        let size = size_hint(&mut file).await?;

        Ok(Self {
            file,
            state: Box::new(EncryptedFileState::from_size_and_args(size, crypto_args)?),
        })
    }

    pub async fn prepare_for_overwrite(&mut self) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.set_len(0).await?;
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
    pub async fn put_auth_tag(mut self) -> std::io::Result<()> {
        if self.state.mode == Mode::Read {
            return self.state.assert_writable();
        } else if self.state.mode == Mode::Write {
            self.state.mode = Mode::Auth;
            let tag = self.state.secret_box.auth_tag();
            self.file.write_all(&tag).await?;
        } else if self.state.mode == Mode::AuthFailure {
            return Err(InvalidAuthTag::io_error());
        }

        Ok(())
    }
}

macro_rules! poll_throw {
    ($e:expr) => {
        match $e {
            Ok(res) => res,
            Err(err) => {
                return ::std::task::Poll::Ready(Err(err));
            }
        }
    };
}

impl<S: SimplexSecretBox> AsyncWrite for EncryptedFile<S> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let mut file = Pin::new(&mut this.file);

        poll_throw!(this.state.assert_writable());

        if this.state.is_encrypted_buf_consumed() {
            this.state.encrypt_chunk(buf);
        }

        while !this.state.is_encrypted_buf_consumed() {
            let encrypted_buf = this.state.encrypted_buf();

            match file.as_mut().poll_write(cx, encrypted_buf) {
                Poll::Ready(res) => {
                    let bytes_written = poll_throw!(res);

                    if bytes_written == 0 {
                        return Poll::Ready(Err(std::io::Error::new(
                            std::io::ErrorKind::WriteZero,
                            "underlying writer accepted 0 bytes",
                        )));
                    }

                    this.state.consume_encrypted_bytes(bytes_written);
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let file = Pin::new(&mut this.file);

        file.poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let mut file = Pin::new(&mut this.file);

        if this.state.mode == Mode::AuthFailure {
            return Poll::Ready(Err(InvalidAuthTag::io_error()));
        }

        if this.state.mode == Mode::Read {
            return Poll::Ready(this.state.assert_writable());
        }

        if this.state.mode == Mode::Write {
            this.state.write_auth_tag_in_buf();
        }

        if this.state.mode == Mode::Auth {
            while !this.state.is_encrypted_buf_consumed() {
                let encrypted_buf = this.state.encrypted_buf();

                match file.as_mut().poll_write(cx, encrypted_buf) {
                    Poll::Ready(res) => {
                        let bytes_written = poll_throw!(res);

                        if bytes_written == 0 {
                            return Poll::Ready(Err(std::io::Error::new(
                                std::io::ErrorKind::WriteZero,
                                "underlying writer accepted 0 bytes",
                            )));
                        }

                        this.state.consume_encrypted_bytes(bytes_written);
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            this.state.mode = Mode::Shutdown;
        }

        if this.state.mode == Mode::Shutdown {
            return file.poll_shutdown(cx);
        }

        unreachable!()
    }
}

impl<S: SimplexSecretBox> AsyncRead for EncryptedFile<S> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let mut file = Pin::new(&mut this.file);

        if this.state.mode == Mode::AuthFailure {
            return Poll::Ready(Err(InvalidAuthTag::io_error()));
        }

        if this.state.mode == Mode::Auth {
            if this.state.is_all_data_read() {
                return Poll::Ready(Ok(()));
            } else {
                while !this.state.is_all_data_read() {
                    let mut auth_buf = tokio::io::ReadBuf::new(this.state.auth_tag_buf());

                    match file.as_mut().poll_read(cx, &mut auth_buf) {
                        Poll::Ready(res) => {
                            poll_throw!(res.map_err(|_| InvalidAuthTag::io_error()));

                            let bytes_read = auth_buf.filled().len();
                            if bytes_read == 0 {
                                return Poll::Ready(Err(InvalidAuthTag::io_error()));
                            }

                            this.state.consume_auth_tag_bytes(bytes_read);
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }

                poll_throw!(this.state.authenticate());
                return Poll::Ready(Ok(()));
            }
        }

        poll_throw!(this.state.assert_readable());

        if buf.remaining() == 0 {
            return Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "reader got exhausted before EOF: the data cannot be authenticated",
            )));
        }

        let mut read_buf = tokio::io::ReadBuf::new(this.state.prep_read_buf(buf.remaining()));
        match file.poll_read(cx, &mut read_buf) {
            Poll::Ready(res) => {
                poll_throw!(res);
                let bytes_read = read_buf.filled().len();

                let out = buf.initialize_unfilled_to(bytes_read);
                this.state.decrypt_read_buf(bytes_read, out);
                buf.advance(bytes_read);

                if this.state.is_all_data_read() {
                    this.state.switch_to_auth_mode();
                } else if bytes_read == 0 {
                    return Poll::Ready(Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "file truncated before ciphertext end",
                    )));
                }

                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<S: SimplexSecretBox> Drop for EncryptedFile<S> {
    fn drop(&mut self) {
        if self.state.mode == Mode::Write {
            log::error!("The file was not authenticated after write");
        }
    }
}

/// An async file that is either plaintext or SimpleX-SecretBox encrypted.
pub enum TokioMaybeCryptoFile<S: SimplexSecretBox> {
    Plain(::tokio::fs::File),
    Encrypted(EncryptedFile<S>),
}

impl<S: SimplexSecretBox> TokioMaybeCryptoFile<S> {
    /// Opens the file read+write so that [`Self::prepare_for_overwrite`] works.
    /// Use [`Self::open_read_only`] when write access is not needed or not available.
    pub async fn open<P: AsRef<Path>>(
        path: P,
        crypto_args: Option<FileCryptoArgs>,
    ) -> std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(EncryptedFile::open(path, args).await?)),
            None => Ok(Self::Plain(
                tokio::fs::OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(false)
                    .open(path)
                    .await?,
            )),
        }
    }

    pub async fn open_read_only<P: AsRef<Path>>(
        path: P,
        crypto_args: Option<FileCryptoArgs>,
    ) -> std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(
                EncryptedFile::open_read_only(path, args).await?,
            )),
            None => Ok(Self::Plain(
                tokio::fs::OpenOptions::new()
                    .write(false)
                    .read(true)
                    .create(false)
                    .open(path)
                    .await?,
            )),
        }
    }

    pub async fn create<P: AsRef<Path>>(
        path: P,
        crypto_args: Option<FileCryptoArgs>,
    ) -> std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(
                EncryptedFile::create_with_args(path, args).await?,
            )),
            None => Ok(Self::Plain(
                tokio::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .await?,
            )),
        }
    }

    pub async fn from_crypto_file(crypto_file: SxcCryptoFile) -> std::io::Result<Self> {
        match crypto_file.crypto_args {
            Some(args) => {
                let crypto_args = FileCryptoArgs::try_from(args)?;
                Self::open(&crypto_file.file_path, Some(crypto_args)).await
            }
            None => Self::open(&crypto_file.file_path, None).await,
        }
    }

    pub async fn from_crypto_file_read_only(crypto_file: SxcCryptoFile) -> std::io::Result<Self> {
        match crypto_file.crypto_args {
            Some(args) => {
                let crypto_args = FileCryptoArgs::try_from(args)?;
                Self::open_read_only(&crypto_file.file_path, Some(crypto_args)).await
            }
            None => Self::open_read_only(&crypto_file.file_path, None).await,
        }
    }

    pub async fn size_hint(&mut self) -> std::io::Result<usize> {
        match self {
            Self::Plain(f) => size_hint(f).await,
            Self::Encrypted(f) => Ok(f.plaintext_size_hint()),
        }
    }

    pub fn crypto_args(&self) -> Option<&FileCryptoArgs> {
        match self {
            Self::Plain(_) => None,
            Self::Encrypted(f) => Some(f.crypto_args()),
        }
    }

    pub async fn prepare_for_overwrite(&mut self) -> std::io::Result<()> {
        match self {
            Self::Plain(f) => {
                f.seek(SeekFrom::Start(0)).await?;
                f.set_len(0).await?;
                Ok(())
            }
            Self::Encrypted(f) => f.prepare_for_overwrite().await,
        }
    }

    /// Writes the AEAD auth tag for encrypted files; no-op for plain files.
    pub async fn put_auth_tag(self) -> std::io::Result<()> {
        match self {
            Self::Plain(_) => Ok(()),
            Self::Encrypted(f) => f.put_auth_tag().await,
        }
    }
}

impl<S: SimplexSecretBox> AsyncRead for TokioMaybeCryptoFile<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => Pin::new(f).poll_read(cx, buf),
            Self::Encrypted(e) => Pin::new(e).poll_read(cx, buf),
        }
    }
}

impl<S: SimplexSecretBox> AsyncWrite for TokioMaybeCryptoFile<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => Pin::new(f).poll_write(cx, buf),
            Self::Encrypted(e) => Pin::new(e).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => Pin::new(f).poll_flush(cx),
            Self::Encrypted(e) => Pin::new(e).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => Pin::new(f).poll_shutdown(cx),
            Self::Encrypted(e) => Pin::new(e).poll_shutdown(cx),
        }
    }
}

async fn size_hint(file: &mut ::tokio::fs::File) -> ::std::io::Result<usize> {
    let size = file.seek(SeekFrom::End(0)).await?;
    file.seek(SeekFrom::Start(0)).await?;

    crate::util::cast_file_size(size)
}
