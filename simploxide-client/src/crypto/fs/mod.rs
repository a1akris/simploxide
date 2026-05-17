use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rand::Rng as _;
use simploxide_api_types::CryptoFileArgs as SxcCryptoFileArgs;
use zeroize::{Zeroize as _, ZeroizeOnDrop, Zeroizing};

use crate::crypto::InvalidCryptoArgs;

use super::{InvalidAuthTag, Poly1305Tag, SimplexSecretBox, XSalsa20Key, XSalsa20Nonce};

pub mod std;
pub mod tokio;

#[cfg(feature = "native_crypto")]
pub type StdEncryptedFile = std::EncryptedFile<super::native::SecretBox>;

#[cfg(feature = "native_crypto")]
pub type TokioEncryptedFile = tokio::EncryptedFile<super::native::SecretBox>;

#[cfg(feature = "native_crypto")]
pub type StdMaybeCryptoFile = std::StdMaybeCryptoFile<super::native::SecretBox>;

#[cfg(feature = "native_crypto")]
pub type TokioMaybeCryptoFile = tokio::TokioMaybeCryptoFile<super::native::SecretBox>;

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

    fn from_size_and_args(
        file_size: usize,
        crypto_args: FileCryptoArgs,
    ) -> ::std::io::Result<Self> {
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
