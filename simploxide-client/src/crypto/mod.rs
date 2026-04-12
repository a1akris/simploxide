pub mod fs;

#[cfg(feature = "native_crypto")]
pub mod native;

pub type XSalsa20Key = [u8; 32];
pub type XSalsa20Nonce = [u8; 24];
pub type Poly1305Tag = [u8; 16];

pub trait SimplexSecretBox {
    /// Return a properly initialized SimpleX `secretbox`.
    ///
    /// Beware that SimpleX uses a non-standard initialization like this:
    ///
    /// intermediate = hsalsa20(xsalsa20_key, [0u8; 16]);
    /// xsalsa20 = xsalsa20_init(intermediate, xsalsa20_nonce);
    /// poly1305_key = (first 32 bytes of xsalsa20 cipherstream);
    fn init(key: &XSalsa20Key, nonce: &XSalsa20Nonce) -> Self;

    /// Write a ciphertext into a `buf`. Update poly1305 but do not authenticate the chunk, the
    /// auth tag must be put only at the end of the whole message.
    fn encrypt_chunk(&mut self, chunk: impl AsRef<[u8]>, buf: impl AsMut<[u8]>);

    /// Write a plaintext into a `buf`. `chunk` is always pure ciphertext, `simploxide` utilities
    /// guarantee that `auth_tag` won't appear in the input chunk.
    fn decrypt_chunk(&mut self, chunk: impl AsRef<[u8]>, buf: impl AsMut<[u8]>);

    fn auth_tag(&mut self) -> Poly1305Tag;

    fn verify_tag(&mut self, tag_to_verify: &Poly1305Tag) -> bool;
}

#[cfg(feature = "native_crypto")]
pub type StdEncryptedFile = fs::std::EncryptedFile<native::SecretBox>;

#[cfg(feature = "native_crypto")]
pub type TokioEncryptedFile = fs::tokio::EncryptedFile<native::SecretBox>;

#[cfg(feature = "native_crypto")]
pub type StdMaybeCryptoFile = fs::MaybeCryptoFile<std::fs::File, StdEncryptedFile>;

#[cfg(feature = "native_crypto")]
pub type TokioMaybeCryptoFile = fs::MaybeCryptoFile<tokio::fs::File, TokioEncryptedFile>;

#[derive(Debug, Clone, Copy)]
pub struct InvalidAuthTag;

impl InvalidAuthTag {
    pub fn io_error() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, InvalidAuthTag)
    }
}

impl From<InvalidAuthTag> for ::std::io::Error {
    fn from(_: InvalidAuthTag) -> Self {
        InvalidAuthTag::io_error()
    }
}

impl std::fmt::Display for InvalidAuthTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid poly1305 auth tag")
    }
}

impl std::error::Error for InvalidAuthTag {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidCryptoArgs;

impl InvalidCryptoArgs {
    pub fn io_error() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, InvalidCryptoArgs)
    }
}

impl From<InvalidCryptoArgs> for ::std::io::Error {
    fn from(_: InvalidCryptoArgs) -> Self {
        InvalidCryptoArgs::io_error()
    }
}

impl std::fmt::Display for InvalidCryptoArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid file crypto args")
    }
}

impl std::error::Error for InvalidCryptoArgs {}
