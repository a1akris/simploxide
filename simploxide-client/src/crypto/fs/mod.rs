use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rand::Rng as _;
use simploxide_api_types::{CryptoFile as SxcCryptoFile, CryptoFileArgs as SxcCryptoFileArgs};
use zeroize::{Zeroize as _, ZeroizeOnDrop, Zeroizing};

use crate::crypto::InvalidCryptoArgs;

use super::{InvalidAuthTag, Poly1305Tag, SimplexSecretBox, XSalsa20Key, XSalsa20Nonce};

pub mod std;
pub mod tokio;

#[derive(ZeroizeOnDrop)]
pub struct FileCryptoArgs {
    key: XSalsa20Key,
    nonce: XSalsa20Nonce,
}

impl FileCryptoArgs {
    fn new(key: &XSalsa20Key, nonce: &XSalsa20Nonce) -> Self {
        Self {
            key: *key,
            nonce: *nonce,
        }
    }

    pub fn try_from_base64(mut key: String, mut nonce: String) -> Result<Self, InvalidCryptoArgs> {
        fn try_decode(key_str: &str, nonce_str: &str) -> Result<FileCryptoArgs, InvalidCryptoArgs> {
            let mut key = Zeroizing::new([0u8; ::std::mem::size_of::<XSalsa20Key>()]);
            let mut nonce = Zeroizing::new([0u8; ::std::mem::size_of::<XSalsa20Nonce>()]);

            decode_base64_arg(key_str, key.as_mut())?;
            decode_base64_arg(nonce_str, nonce.as_mut())?;

            Ok(FileCryptoArgs::new(&key, &nonce))
        }

        let result = try_decode(&key, &nonce);

        key.zeroize();
        nonce.zeroize();

        result
    }

    pub fn expose(&self) -> SxcCryptoFileArgs {
        SxcCryptoFileArgs {
            file_key: URL_SAFE.encode(self.key),
            file_nonce: URL_SAFE.encode(self.nonce),
            undocumented: Default::default(),
        }
    }
}

impl TryFrom<SxcCryptoFileArgs> for FileCryptoArgs {
    type Error = InvalidCryptoArgs;

    fn try_from(args: SxcCryptoFileArgs) -> Result<Self, Self::Error> {
        Self::try_from_base64(args.file_key, args.file_nonce)
    }
}

pub trait PlainFileOps: Sized {
    fn open<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<Self>;

    fn create<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<Self>;
}

pub trait AsyncPlainFileOps: Sized {
    fn open<P: AsRef<::std::path::Path>>(path: P) -> impl Future<Output = ::std::io::Result<Self>>;

    fn create<P: AsRef<::std::path::Path>>(
        path: P,
    ) -> impl Future<Output = ::std::io::Result<Self>>;
}

pub trait EncryptedFileOps: Sized {
    fn open<P: AsRef<::std::path::Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> ::std::io::Result<Self>;

    /// Creates file with random crypto args
    fn create<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<Self>;

    fn create_with_args<P: AsRef<::std::path::Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> ::std::io::Result<Self>;

    fn crypto_args(&self) -> &FileCryptoArgs;
}

pub trait AsyncEncryptedFileOps: Sized {
    fn open<P: AsRef<::std::path::Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> impl Future<Output = ::std::io::Result<Self>>;

    /// Creates file with random crypto args
    fn create<P: AsRef<::std::path::Path>>(
        path: P,
    ) -> impl Future<Output = ::std::io::Result<Self>>;

    fn create_with_args<P: AsRef<::std::path::Path>>(
        path: P,
        crypto_args: FileCryptoArgs,
    ) -> impl Future<Output = ::std::io::Result<Self>>;

    fn crypto_args(&self) -> &FileCryptoArgs;
}

pub enum MaybeCryptoFile<P, E> {
    Plain(P),
    Encrypted(E),
}

impl<P: PlainFileOps, E: EncryptedFileOps> MaybeCryptoFile<P, E> {
    pub fn open<PA: AsRef<::std::path::Path>>(
        path: PA,
        crypto_args: Option<FileCryptoArgs>,
    ) -> ::std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(E::open(path, args)?)),
            None => Ok(Self::Plain(P::open(path)?)),
        }
    }

    pub fn create<PA: AsRef<::std::path::Path>>(
        path: PA,
        crypto_args: Option<FileCryptoArgs>,
    ) -> ::std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(E::create_with_args(path, args)?)),
            None => Ok(Self::Plain(P::create(path)?)),
        }
    }

    pub fn reader(crypto_file: SxcCryptoFile) -> ::std::io::Result<Self> {
        match crypto_file.crypto_args {
            Some(args) => {
                let crypto_args = FileCryptoArgs::try_from(args)?;
                Self::open(&crypto_file.file_path, Some(crypto_args))
            }
            None => Self::open(&crypto_file.file_path, None),
        }
    }
}

impl<P: AsyncPlainFileOps, E: AsyncEncryptedFileOps> MaybeCryptoFile<P, E> {
    pub async fn open_async<PA: AsRef<::std::path::Path>>(
        path: PA,
        crypto_args: Option<FileCryptoArgs>,
    ) -> ::std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(E::open(path, args).await?)),
            None => Ok(Self::Plain(P::open(path).await?)),
        }
    }

    pub async fn create_async<PA: AsRef<::std::path::Path>>(
        path: PA,
        crypto_args: Option<FileCryptoArgs>,
    ) -> ::std::io::Result<Self> {
        match crypto_args {
            Some(args) => Ok(Self::Encrypted(E::create_with_args(path, args).await?)),
            None => Ok(Self::Plain(P::create(path).await?)),
        }
    }

    pub async fn reader_async(crypto_file: SxcCryptoFile) -> ::std::io::Result<Self> {
        match crypto_file.crypto_args {
            Some(args) => {
                let crypto_args = FileCryptoArgs::try_from(args)?;
                Self::open_async(&crypto_file.file_path, Some(crypto_args)).await
            }
            None => Self::open_async(&crypto_file.file_path, None).await,
        }
    }
}

impl<P: ::std::io::Read, E: ::std::io::Read> ::std::io::Read for MaybeCryptoFile<P, E> {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        match self {
            Self::Plain(p) => p.read(buf),
            Self::Encrypted(e) => e.read(buf),
        }
    }
}

impl<P: ::std::io::Write, E: ::std::io::Write> ::std::io::Write for MaybeCryptoFile<P, E> {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        match self {
            Self::Plain(p) => p.write(buf),
            Self::Encrypted(e) => e.write(buf),
        }
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        match self {
            Self::Plain(p) => p.flush(),
            Self::Encrypted(e) => e.flush(),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
        match self {
            Self::Plain(p) => p.write_all(buf),
            Self::Encrypted(e) => e.write_all(buf),
        }
    }
}

impl<P: Unpin + ::tokio::io::AsyncRead, E: Unpin + ::tokio::io::AsyncRead> ::tokio::io::AsyncRead
    for MaybeCryptoFile<P, E>
{
    fn poll_read(
        self: ::std::pin::Pin<&mut Self>,
        cx: &mut ::std::task::Context<'_>,
        buf: &mut ::tokio::io::ReadBuf<'_>,
    ) -> ::std::task::Poll<::std::io::Result<()>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => ::std::pin::Pin::new(f).poll_read(cx, buf),
            Self::Encrypted(e) => ::std::pin::Pin::new(e).poll_read(cx, buf),
        }
    }
}

impl<P: Unpin + ::tokio::io::AsyncWrite, E: Unpin + ::tokio::io::AsyncWrite> ::tokio::io::AsyncWrite
    for MaybeCryptoFile<P, E>
{
    fn poll_write(
        self: ::std::pin::Pin<&mut Self>,
        cx: &mut ::std::task::Context<'_>,
        buf: &[u8],
    ) -> ::std::task::Poll<::std::io::Result<usize>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => ::std::pin::Pin::new(f).poll_write(cx, buf),
            Self::Encrypted(e) => ::std::pin::Pin::new(e).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: ::std::pin::Pin<&mut Self>,
        cx: &mut ::std::task::Context<'_>,
    ) -> ::std::task::Poll<::std::io::Result<()>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => ::std::pin::Pin::new(f).poll_flush(cx),
            Self::Encrypted(e) => ::std::pin::Pin::new(e).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: ::std::pin::Pin<&mut Self>,
        cx: &mut ::std::task::Context<'_>,
    ) -> ::std::task::Poll<::std::io::Result<()>> {
        let this = self.get_mut();
        match this {
            Self::Plain(f) => ::std::pin::Pin::new(f).poll_shutdown(cx),
            Self::Encrypted(e) => ::std::pin::Pin::new(e).poll_shutdown(cx),
        }
    }
}

struct EncryptedFileState<S> {
    crypto_args: FileCryptoArgs,
    secret_box: S,
    buf: Zeroizing<Vec<u8>>,
    mode: Mode,
    remaining_data_len: usize,
}

impl<S> EncryptedFileState<S> {
    const DEFAULT_BUFSIZE: usize = 65536;
}

impl<S: SimplexSecretBox> EncryptedFileState<S> {
    fn new() -> Self {
        let mut rng = rand::rng();

        let mut key = [0u8; ::std::mem::size_of::<XSalsa20Key>()];
        let mut nonce = [0u8; ::std::mem::size_of::<XSalsa20Nonce>()];

        rng.fill_bytes(&mut key);
        rng.fill_bytes(&mut nonce);

        let crypto_args = FileCryptoArgs::new(&key, &nonce);
        let secret_box = SimplexSecretBox::init(&key, &nonce);

        key.zeroize();
        nonce.zeroize();

        Self {
            crypto_args,
            secret_box,
            buf: Zeroizing::new(Vec::new()),
            mode: Mode::Write,
            remaining_data_len: 0,
        }
    }

    fn from_args(crypto_args: FileCryptoArgs) -> Self {
        let secret_box = SimplexSecretBox::init(&crypto_args.key, &crypto_args.nonce);

        Self {
            crypto_args,
            secret_box,
            buf: Zeroizing::new(Vec::new()),
            mode: Mode::Write,
            remaining_data_len: 0,
        }
    }

    fn from_size_and_args(file_size: u64, crypto_args: FileCryptoArgs) -> ::std::io::Result<Self> {
        let file_size: usize = file_size
            .try_into()
            .map_err(|e| ::std::io::Error::new(::std::io::ErrorKind::FileTooLarge, e))?;

        let mut state = Self::from_args(crypto_args);
        if file_size < ::std::mem::size_of::<Poly1305Tag>() {
            return Err(InvalidAuthTag::io_error());
        } else if file_size == ::std::mem::size_of::<Poly1305Tag>() {
            state.switch_to_auth_mode();
        } else {
            state.remaining_data_len = file_size - ::std::mem::size_of::<Poly1305Tag>();
            state.mode = Mode::Read;
        }

        Ok(state)
    }

    fn crypto_args(&self) -> &FileCryptoArgs {
        &self.crypto_args
    }

    fn reset(&mut self) {
        let mut rng = rand::rng();
        let mut key = [0u8; ::std::mem::size_of::<XSalsa20Key>()];
        let mut nonce = [0u8; ::std::mem::size_of::<XSalsa20Nonce>()];

        rng.fill_bytes(&mut key);
        rng.fill_bytes(&mut nonce);

        self.crypto_args = FileCryptoArgs::new(&key, &nonce);
        self.secret_box = SimplexSecretBox::init(&key, &nonce);
        self.remaining_data_len = 0;

        key.zeroize();
        nonce.zeroize();
    }

    fn encrypt_chunk(&mut self, chunk: &[u8]) -> &[u8] {
        self.buf
            .reserve_exact(::std::cmp::max(Self::DEFAULT_BUFSIZE, chunk.len()));
        self.buf.resize(chunk.len(), 0);

        self.secret_box.encrypt_chunk(chunk, &mut self.buf);
        self.remaining_data_len += chunk.len();
        &self.buf
    }

    fn encrypted_buf(&self) -> &[u8] {
        let offset = self
            .buf
            .len()
            .checked_sub(self.remaining_data_len)
            .expect("encrypted_buf: no overflows");

        &self.buf[offset..]
    }

    fn consume_encrypted_bytes(&mut self, bytes: usize) {
        self.remaining_data_len = self
            .remaining_data_len
            .checked_sub(bytes)
            .expect("consume_encrypted_bytes: no overflows");
    }

    fn is_encrypted_buf_consumed(&self) -> bool {
        self.is_all_data_read()
    }

    fn prep_read_buf(&mut self, bytes: usize) -> &mut [u8] {
        let corrected_bytes = ::std::cmp::min(bytes, self.remaining_data_len);
        let buf_size = ::std::cmp::max(self.optimal_buf_size(), corrected_bytes);
        self.buf.reserve_exact(buf_size);

        self.buf.resize(corrected_bytes, 0);
        &mut self.buf
    }

    fn decrypt_read_buf(&mut self, bytes_read: usize, out_chunk: &mut [u8]) {
        self.remaining_data_len = self
            .remaining_data_len
            .checked_sub(bytes_read)
            .expect("decrypt_read_buf: no overflows");

        self.secret_box
            .decrypt_chunk(&mut self.buf[..bytes_read], out_chunk);
    }

    fn is_all_data_read(&self) -> bool {
        self.remaining_data_len == 0
    }

    fn optimal_buf_size(&self) -> usize {
        if self.mode == Mode::Auth {
            ::std::mem::size_of::<Poly1305Tag>()
        } else if self.mode == Mode::Read && self.remaining_data_len < Self::DEFAULT_BUFSIZE {
            self.remaining_data_len
        } else {
            Self::DEFAULT_BUFSIZE
        }
    }

    fn plaintext_size_hint(&self) -> usize {
        match self.mode {
            Mode::Read => self.remaining_data_len,
            Mode::Write => EncryptedFileState::<S>::DEFAULT_BUFSIZE,
            Mode::Auth | Mode::Shutdown | Mode::AuthFailure => 0,
        }
    }

    fn assert_writable(&self) -> ::std::io::Result<()> {
        if self.mode == Mode::Write {
            Ok(())
        } else {
            Err(::std::io::Error::other("Trying to write non-writable file"))
        }
    }

    fn assert_readable(&self) -> ::std::io::Result<()> {
        if self.mode == Mode::Read {
            Ok(())
        } else {
            Err(::std::io::Error::other("Trying to read non-readable file"))
        }
    }

    fn switch_to_auth_mode(&mut self) {
        self.mode = Mode::Auth;
        self.buf.resize(::std::mem::size_of::<Poly1305Tag>(), 0);
        self.remaining_data_len = ::std::mem::size_of::<Poly1305Tag>();
    }

    fn write_auth_tag_in_buf(&mut self) {
        self.switch_to_auth_mode();
        let file_tag = self.secret_box.auth_tag();
        self.buf.copy_from_slice(&file_tag);
    }

    fn auth_tag_buf(&mut self) -> &mut [u8] {
        let offset = self
            .buf
            .len()
            .checked_sub(self.remaining_data_len)
            .expect("auth_tag_buf: no overflows");

        self.buf[offset..].as_mut()
    }

    fn consume_auth_tag_bytes(&mut self, bytes: usize) {
        self.consume_encrypted_bytes(bytes);
    }

    fn authenticate(&mut self) -> ::std::io::Result<()> {
        let file_tag: &Poly1305Tag = self
            .buf
            .as_slice()
            .try_into()
            .map_err(|_| InvalidAuthTag::io_error())?;

        let result = if self.secret_box.verify_tag(file_tag) {
            Ok(())
        } else {
            self.mode = Mode::AuthFailure;
            Err(InvalidAuthTag::io_error())
        };

        self.buf.truncate(0);
        self.remaining_data_len = 0;

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Read,
    Write,
    Auth,
    Shutdown,
    AuthFailure,
}

fn decode_base64_arg(b64str: &str, buf: &mut [u8]) -> Result<(), InvalidCryptoArgs> {
    let len = URL_SAFE
        .decode_slice(b64str, buf)
        .map_err(|_| InvalidCryptoArgs)?;

    if len == buf.len() {
        Ok(())
    } else {
        Err(InvalidCryptoArgs)
    }
}
