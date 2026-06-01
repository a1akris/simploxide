//! Image previews generation

use base64::prelude::*;
#[cfg(feature = "native_crypto")]
use simploxide_api_types::CryptoFile;
use tokio::io::{AsyncReadExt as _, AsyncSeekExt as _};

use crate::util;

use std::{
    io::SeekFrom,
    path::{Path, PathBuf},
};

const DEFAULT_PREVIEW: &str = "data:image/jpg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/\
2wBDABALDA4MChAODQ4SERATGCgaGBYWGDEjJR0oOjM9PDkzODdASFxOQERXRTc4UG1RV19iZ2hnPk1xeXBkeFxlZ2P/\
2wBDARESEhgVGC8aGi9jQjhCY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2NjY2P/wAARCABVAIADASIAAhEBAxEB/\
8QAFgABAQEAAAAAAAAAAAAAAAAAAAEE/8QAFBABAAAAAAAAAAAAAAAAAAAAAP/EABgBAQEBAQEAAAAAAAAAAAAAAAMCAQUE/8QAFhEBAQEAAAAAAAAAAAAAAAAAAAER/\
9oADAMBAAIRAxEAPwDaKF17qgo3UVBRWjqCjdFUFFaOoKN0VQUVo6go3R0FHi13ago3R1BRWjoijdHUFFSiqCjZR1BRUo6go3RVQHh13aAK1FAVuiqCipR1BRUo6go2UV\
QUVKOoKK0dAHg13aCjdHUFFaOoKKlHUFFSiqCipR1BRsoqgoqUdBR4Nd2oKNlHUUFSjoAqUdAFSjoAqUVAFSjoAqUdAHPd2gCoOqAqDoA2DoAuCoAqDoAqCoAqDr//2Q==";

const MAX_PREVIEW_BYTES: usize = 10_000;
#[cfg(feature = "multimedia")]
const MAX_FILE_SIZE: usize = 64 * 1024 * 1024;

/// Thumbnail for [`Image`](crate::messages::Image), [`Video`](crate::messages::Video), and
/// [`Link`](crate::messages::Link) messages. Also used as bot profile pictures. The source is stored
/// lazily and resolved when [`resolve`](Self::resolve) or [`try_resolve`](Self::try_resolve) is
/// called(either manually or automatically by message builders). Any error falls back to a default
/// ~600 bytes in size JPEG placeholder.
#[derive(Clone)]
pub struct ImagePreview {
    source: PreviewSource,
    #[cfg(feature = "multimedia")]
    transcoder: Transcoder,
}

impl Default for ImagePreview {
    fn default() -> Self {
        Self {
            source: PreviewSource::Default,
            #[cfg(feature = "multimedia")]
            transcoder: Transcoder::thumbnail(),
        }
    }
}

impl std::fmt::Debug for ImagePreview {
    #[cfg(not(feature = "multimedia"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImagePreview")
            .field("source", &self.kind())
            .finish()
    }

    #[cfg(feature = "multimedia")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImagePreview")
            .field("source", &self.kind())
            .field("transcoder", &self.transcoder)
            .finish()
    }
}

impl ImagePreview {
    /// Thumbnail from raw JPEG bytes. Fails on resolve if the encoded data URI exceeds 13333 bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            source: PreviewSource::Bytes(bytes.into()),
            #[cfg(feature = "multimedia")]
            transcoder: Transcoder::thumbnail(),
        }
    }

    /// Thumbnail from a pre-assembled `data:image/jpg;base64,{base64_contents} URI string.
    pub fn raw(uri: impl Into<String>) -> Self {
        Self {
            source: PreviewSource::DataUri(uri.into()),
            #[cfg(feature = "multimedia")]
            transcoder: Transcoder::thumbnail(),
        }
    }

    /// Thumbnail loaded from a file; the file is read lazily when resolved.
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        Self {
            source: PreviewSource::File(path.as_ref().to_path_buf()),
            #[cfg(feature = "multimedia")]
            transcoder: Transcoder::thumbnail(),
        }
    }

    pub fn kind(&self) -> PreviewKind {
        match self.source {
            PreviewSource::Default => PreviewKind::Default,
            PreviewSource::Bytes(_) => PreviewKind::Bytes,
            PreviewSource::DataUri(_) => PreviewKind::Raw,
            PreviewSource::File(_) => PreviewKind::File,
            #[cfg(feature = "native_crypto")]
            PreviewSource::CryptoFile(_) => PreviewKind::CryptoFile,
        }
    }

    #[cfg(feature = "native_crypto")]
    /// Thumbnail loaded from an encrypted file; decrypted lazily when resolved.
    pub fn from_crypto_file(file: CryptoFile) -> Self {
        Self {
            source: PreviewSource::CryptoFile(file),
            #[cfg(feature = "multimedia")]
            transcoder: Transcoder::thumbnail(),
        }
    }

    #[cfg(feature = "multimedia")]
    /// Attach a custom [`Transcoder`] to transcode the source as a JPEG thumbnail on resolve.
    /// Transcoder transcodes images of any widespread format to JPEGs.
    ///
    /// Has no effect on `default` and `raw` sources, they always passed as is.
    pub fn with_transcoder(mut self, transcoder: Transcoder) -> Self {
        self.set_transcoder(transcoder);
        self
    }

    #[cfg(feature = "multimedia")]
    pub fn set_transcoder(&mut self, transcoder: Transcoder) {
        self.transcoder = transcoder;
    }

    /// Like [`Self::try_resolve`] but falls back to the default placeholder preview on error.
    pub async fn resolve(self) -> String {
        match self.try_resolve().await {
            Ok(s) => s,
            Err(e) => {
                log::warn!("Falling back to default preview due to an error: {e}");
                default()
            }
        }
    }

    #[cfg(not(feature = "multimedia"))]
    /// Returns the preview as a `data:image/jpg;base64,{base64_contents}` URI. The source is
    /// assumed to be a valid JPEG(encoding is not validated) when multimedia feature is off or is
    /// lazily transcoded to JPEG when multimedia feature is on. Fails if the source cannot be read
    /// or the encoded URI exceeds 13333 bytes.
    pub async fn try_resolve(self) -> Result<String, PreviewError> {
        match self.source {
            PreviewSource::Default => Ok(default()),
            PreviewSource::Bytes(b) => try_encode_jpg_to_uri(&b),
            PreviewSource::DataUri(s) => validate_uri_preview(s),
            PreviewSource::File(path) => {
                let bytes = read_plain_file(&path, MAX_PREVIEW_BYTES).await?;
                try_encode_jpg_to_uri(&bytes)
            }
            #[cfg(feature = "native_crypto")]
            PreviewSource::CryptoFile(file) => {
                let bytes = read_crypto_file(file, MAX_PREVIEW_BYTES).await?;
                try_encode_jpg_to_uri(&bytes)
            }
        }
    }

    #[cfg(feature = "multimedia")]
    /// Returns the preview as a `data:image/jpg;base64,{base64_contents}` URI. The source is
    /// assumed to be a valid JPEG(encoding is not validated) when multimedia feature is off or is
    /// lazily transcoded to JPEG when multimedia feature is on. Fails if the source cannot be read
    /// or the encoded URI exceeds 13333 bytes.
    pub async fn try_resolve(self) -> Result<String, PreviewError> {
        let bytes = match self.source {
            PreviewSource::Default => return Ok(default()),
            PreviewSource::Bytes(b) => b,
            PreviewSource::DataUri(s) => {
                return validate_uri_preview(s);
            }
            PreviewSource::File(path) => read_plain_file(&path, MAX_FILE_SIZE).await?,
            #[cfg(feature = "native_crypto")]
            PreviewSource::CryptoFile(file) => read_crypto_file(file, MAX_FILE_SIZE).await?,
        };

        let jpg_bytes = if self.transcoder.is_enabled() {
            tokio::task::spawn_blocking(move || -> Result<Vec<u8>, PreviewError> {
                self.transcoder.transcode_to_jpg(bytes)
            })
            .await??
        } else {
            bytes
        };

        try_encode_jpg_to_uri(&jpg_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewKind {
    Default,
    Bytes,
    Raw,
    File,
    #[cfg(feature = "native_crypto")]
    CryptoFile,
}

#[cfg(feature = "multimedia")]
pub mod transcoder {
    use image::{ImageReader, codecs::jpeg::JpegEncoder};
    use std::io::Cursor;

    use super::PreviewError;

    /// Transcodes images of any wide-spread types to JPEG thumbnails. Default settings generate
    /// previews similar to SimpleX-Chat previews
    #[derive(Debug, Clone, Copy)]
    pub struct Transcoder {
        enabled: bool,
        size: (u8, u8),
        quality: u8,
        blur: f32,
    }

    impl Default for Transcoder {
        fn default() -> Self {
            Self {
                enabled: true,
                size: (0, 0),
                quality: 100,
                blur: 0.0,
            }
        }
    }

    impl Transcoder {
        /// Disable transcoding entirely. Useful for pre-made and pictures thumbnails.
        pub fn disabled() -> Self {
            Self {
                enabled: false,
                ..Default::default()
            }
        }

        /// Only convert the image to JPEG(the best quality) without applying any other
        /// transformations. Use builder methods to add transformations on top
        pub fn jpeg() -> Self {
            Self::default()
        }

        /// The default transcoder for thumbnails. Modify defaults with builder methods.
        pub fn thumbnail() -> Self {
            Self {
                enabled: true,
                size: (128, 128),
                quality: 60,
                blur: 0.0,
            }
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        /// Bound between 32x32 and 255x255
        pub fn with_size(mut self, x: u8, y: u8) -> Self {
            let x = std::cmp::max(32, x);
            let y = std::cmp::max(32, y);

            self.size = (x, y);

            self
        }

        /// Quality is bound between 1..=100 where 1 is the worst
        pub fn with_quality(mut self, quality: u8) -> Self {
            if quality == 0 {
                self.quality = 1;
            } else if quality > 100 {
                self.quality = 100;
            } else {
                self.quality = quality;
            }

            self
        }

        /// sigma < 1.0 - no blur. sigma = 100.0 - max blur
        pub fn with_blur(mut self, sigma: f32) -> Self {
            if sigma < 1.0 {
                self.blur = 0.0;
            } else if sigma > 100.0 {
                self.blur = 100.0
            } else {
                self.blur = sigma
            };

            self
        }

        /// **WARNING**: this is a relatively expensive blocking operation, ensure that you call
        /// this method outside the tokio executor with `tokio::spawn_blocking` or on a dedicated
        /// thread.
        pub fn transcode_to_jpg(self, mut bytes: Vec<u8>) -> Result<Vec<u8>, PreviewError> {
            if !self.enabled {
                return Ok(bytes);
            }

            let img = ImageReader::new(Cursor::new(&bytes))
                .with_guessed_format()?
                .decode()?;

            let img = if self.size != (0, 0) {
                img.thumbnail(self.size.0.into(), self.size.1.into())
            } else {
                img
            };

            let img = if self.blur >= 1.0 {
                img.fast_blur(self.blur)
            } else {
                img
            };

            bytes.clear();
            let encoder = JpegEncoder::new_with_quality(&mut bytes, self.quality);
            img.write_with_encoder(encoder)?;

            Ok(bytes)
        }
    }
}

#[cfg(feature = "multimedia")]
pub use transcoder::Transcoder;

const URI_HEADER: &str = "data:image/jpg;base64,";

pub fn default() -> String {
    DEFAULT_PREVIEW.to_owned()
}

/// Returns the default preview on [`PreviewError`]
pub fn encode_jpg_to_uri(bytes: &[u8]) -> String {
    match try_encode_jpg_to_uri(bytes) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("{e}");
            default()
        }
    }
}

pub fn try_encode_jpg_to_uri(bytes: &[u8]) -> Result<String, PreviewError> {
    if bytes.len() > MAX_PREVIEW_BYTES {
        return Err(PreviewError::TooLarge);
    }

    let mut encoded = String::with_capacity(bytes.len() * 4 / 3 + URI_HEADER.len() + 3);
    encoded.push_str(URI_HEADER);
    BASE64_STANDARD.encode_string(bytes, &mut encoded);

    Ok(encoded)
}

pub fn try_decode_jpg_from_uri(uri_str: &str) -> Result<Vec<u8>, UriDecodeError> {
    let Some(s) = uri_str.strip_prefix(URI_HEADER) else {
        return Err(UriDecodeError::NotAUri);
    };

    BASE64_STANDARD.decode(s).map_err(UriDecodeError::Base64)
}

#[derive(Debug)]
pub enum PreviewError {
    TooLarge,
    BadUri(UriDecodeError),
    Io(std::io::Error),
    #[cfg(feature = "multimedia")]
    Transcoding(image::ImageError),
    #[cfg(feature = "multimedia")]
    Tokio(tokio::task::JoinError),
}

impl From<std::io::Error> for PreviewError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[cfg(feature = "multimedia")]
impl From<image::ImageError> for PreviewError {
    fn from(err: image::ImageError) -> Self {
        Self::Transcoding(err)
    }
}

#[cfg(feature = "multimedia")]
impl From<tokio::task::JoinError> for PreviewError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Tokio(err)
    }
}

impl std::fmt::Display for PreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLarge => {
                write!(
                    f,
                    "preview size exceeds the max possible size({MAX_PREVIEW_BYTES} bytes)"
                )
            }
            Self::BadUri(e) => write!(f, "{e}"),
            Self::Io(error) => write!(f, "Cannot process preview file: {error}"),
            #[cfg(feature = "multimedia")]
            Self::Transcoding(error) => write!(f, "Cannot transcode preview: {error}"),
            #[cfg(feature = "multimedia")]
            Self::Tokio(error) => write!(f, "Failed to join the transcoding task: {error}"),
        }
    }
}

impl std::error::Error for PreviewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TooLarge => None,
            Self::BadUri(error) => Some(error),
            Self::Io(error) => Some(error),
            #[cfg(feature = "multimedia")]
            Self::Transcoding(error) => Some(error),
            #[cfg(feature = "multimedia")]
            Self::Tokio(error) => Some(error),
        }
    }
}

#[derive(Debug)]
pub enum UriDecodeError {
    NotAUri,
    Base64(base64::DecodeError),
}

impl std::fmt::Display for UriDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAUri => write!(f, "not a URI string"),
            Self::Base64(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for UriDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Base64(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

#[derive(Clone)]
enum PreviewSource {
    Default,
    Bytes(Vec<u8>),
    DataUri(String),
    File(PathBuf),
    #[cfg(feature = "native_crypto")]
    CryptoFile(CryptoFile),
}

async fn read_plain_file(path: &PathBuf, size_limit: usize) -> std::io::Result<Vec<u8>> {
    let mut f = tokio::fs::File::open(&path).await?;
    let size_hint = f.seek(SeekFrom::End(0)).await?;
    f.seek(SeekFrom::Start(0)).await?;
    let size_hint: usize = util::cast_file_size(size_hint)?;

    if size_hint > size_limit {
        return Err(util::file_is_too_large(format!(
            "Size exceeds {size_limit} bytes"
        )));
    }

    let mut buf = Vec::with_capacity(size_hint);
    f.read_to_end(&mut buf).await?;

    Ok(buf)
}

#[cfg(feature = "native_crypto")]
async fn read_crypto_file(file: CryptoFile, size_limit: usize) -> std::io::Result<Vec<u8>> {
    let mut f = crate::crypto::fs::TokioMaybeCryptoFile::from_crypto_file(file).await?;
    let size_hint = f.size_hint().await?;

    if size_hint > size_limit {
        return Err(util::file_is_too_large(format!(
            "Size exceeds {size_limit} bytes"
        )));
    }

    let mut buf = Vec::with_capacity(size_hint);
    f.read_to_end(&mut buf).await?;

    Ok(buf)
}

fn validate_uri_preview(uri: String) -> Result<String, PreviewError> {
    let Some(s) = uri.strip_prefix(URI_HEADER) else {
        return Err(PreviewError::BadUri(UriDecodeError::NotAUri));
    };

    if s.len() > MAX_PREVIEW_BYTES * 4 / 3 {
        return Err(PreviewError::TooLarge);
    }

    Ok(uri)
}
