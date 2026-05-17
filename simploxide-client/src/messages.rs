//! Message builders.
//!
//! Any [`MessageLike`] value can be passed to send message methods and then modified by a
//! plenty of builder options as shown in the usage examples below
//!
//!
//! ### Simple text
//!
//! ```ignore
//! // Regular text message
//! bot.send_msg(chat, "Hello").await?;
//!
//! // Regular text reply with TTL
//! bot.send_msg(chat, "Hello")
//!     .reply_to(msg)
//!     .with_ttl(Duration::from_secs(3600))
//!     .await?;
//!
//! // Formatted text
//! bot.send_msg(chat, Text::yellow("Warning: operation is cancelledd")).await?;
//!
//! // Heavily-formatted text
//! bot.send_msg(
//!     chat,
//!     format!("{}\n\nThe operation {} {}",
//!         "Attention".bold(),
//!         op.italic(),
//!         "is not permitted".red()
//!     )
//! ).await?;
//! ```
//!
//! ### Simple files
//!
//! ```ignore
//! // Plain file with caption
//! bot.send_msg(
//!     chat,
//!     File::new("document.pdf")
//!         .with_caption("Here's the doc")
//! ).await?;
//!
//! // Same as above but with a message builder method
//! bot.send_msg(chat, File::new("document.pdf"))
//!    .set_text("Here's the doc")
//!    .await?;
//!
//! // Attach a CryptoFile to a text message
//! bot.send_msg(chat, "See attached")
//!    .attach(crypto_file)
//!    .await?;
//! ```
//!
//! ### Images
//!
//! ```ignore
//! // With multimedia: source file is automatically transcoded into a thumbnail
//! // Without multimedia: sends with the default placeholder as a preview
//! bot.send_msg(chat, Image::new("img.jpg")).await?;
//!
//! // Override transcoder settings(requires `multimedia` feature)
//! bot.send_msg(chat, Image::new("img.jpg"))
//!     .with_transcoder(
//!         Transcoder::default()
//!             .with_size(200, 200)
//!             .with_quality(80)
//!             .with_blur(1.5)
//!     ).await?;
//!
//! // Get thumbnail from in memory bytes. With `multimedia` feature the bytes will be transcoded
//! // to JPG so with_transcoder(Transcoder::disabled()) is used to opt out, without multimedia the bytes
//! // are used as is
//! bot.send_msg(chat, Image::new("img.jpg"))
//!     .with_preview(
//!         ImagePreview::from_bytes(thumb_bytes)
//!             .with_transcoder(Transcoder::disabled())
//!     ).await?;
//!
//! // Thumbnail from a separate file(read asyncronously at send time)
//! bot.send_msg(chat, Image::new("img.jpg"))
//!     .with_preview(ImagePreview::from_file("thumb.jpg"))
//!     .await?;
//!
//! // Encrypted source and thumbnail(requires feature `native_crypto`)
//! bot.send_msg(chat, Image::from(image_crypto_file))
//!     .with_preview(ImagePreview::from_crypto_file(thumb_crypto_file))
//!     .await?;
//!
//! // Text transitioning to image so "Here is the photo" becomes the caption
//! bot.send_msg(chat, "Here is the photo")
//!     .with_image(Image::new("img.jpg"))
//!     .await?;
//! ```
//!
//! ### Video
//!
//! Automatic preview generation from video files is currently unsupported. A custom preview can be
//! provided, or the message sends with the default placeholder preview.
//!
//! ```ignore
//! // Default placeholder preview
//! bot.send_msg(chat, Video::new("vid.mp4", Duration::from_secs(30))).await?;
//!
//! // Custom thumbnail
//! bot.send_msg(chat, Video::new("vid.mp4", Duration::from_secs(30)))
//!     .with_preview(ImagePreview::from_bytes(thumb_bytes))
//!     .await?;
//!
//! // Custom thumbnail from a file, resized at send time(requires `multimedia`)
//! bot.send_msg(chat, Video::new("vid.mp4", Duration::from_secs(30)))
//!     .with_preview(
//!         ImagePreview::from_file("thumb.jpg")
//!             .with_transcoder(Transcoder::default().with_size(255, 255))
//!     )
//!     .await?;
//! ```
//!
//! ### Link
//!
//! ```ignore
//! // Minimal: no preview image, no metadata
//! bot.send_msg(chat, Link::new("https://example.com")).await?;
//!
//! // Full Open Graph preview
//! let og_bytes: Vec<u8> = fetch_og_image("https://example.com").await?;
//! bot.send_msg(chat,
//!     Link::new("https://example.com")
//!         .with_title("Example Domain")
//!         .with_description("Domain description")
//!         .with_content(LinkContent::make_page())
//! )
//! .with_preview(ImagePreview::from_bytes(og_bytes))
//! .await?;
//!
//! // Text transitioning to link
//! bot.send_msg(chat, "Check this out")
//!     .with_link(Link::new("https://example.com").with_title("Example"))
//!     .await?;
//! ```
//!
//! ### Special messages like reports and chat links
//!
//! ```ignore
//! // Report
//! bot.send_msg(chat, Report::spam("Unsolicited advertisement")).await?;
//!
//! // Report via text transition so the text becomes the report body
//! bot.send_msg(chat, "Unsolicited advertisement").report(ReportReason::Spam).await?;
//!
//! // Chat invitation
//! bot.send_msg(chat, Chat::new(chat_link).with_text("Join our group")).await?;
//! ```
//!
//! ### Custom and Raw messages
//!
//! Custom messages are useful for implementing interbot protocols
//!
//! ```ignore
//! bot.send_msg(chat, Custom::new("app.ping", &PingPayload { id: 42 })).await?;
//! ```
//!
//! [`ComposedMessage`] is for dynamic construction scenarios where the message content, media
//! type, or delivery options are determined by program logic rather than known at compile time.
//! Because [`ComposedMessage`] is sent verbatim, preview resolution is the caller's
//! responsibility.
//!
//! ```ignore
//! // resolve() always returns a valid preview string, falling back to the default on any error
//! let preview = ImagePreview::from_file("thumb.jpg").resolve().await;
//!
//! // try_resolve() surfaces the error so the caller can dechate what to do
//! let preview = match ImagePreview::from_file("thumb.jpg").try_resolve().await {
//!     Ok(s) => s,
//!     Err(e) => {
//!         log::error!("Preview failed: {e}");
//!         return Err(e.into());
//!     }
//! };
//!
//! let mut msg = ComposedMessage {
//!     file_source: None,
//!     msg_content: MsgContent::make_text(String::new()),
//!     quoted_item_id: None,
//!     mentions: Default::default(),
//!     undocumented: Default::default(),
//! };
//!
//! if let Some(image_file) = attachment {
//!     msg.file_source = Some(image_file);
//!     msg.msg_content = MsgContent::make_image(caption, preview);
//! }
//!
//! if let Some(id) = reply_to_id {
//!     msg.quoted_item_id = Some(id);
//! }
//!
//! bot.send_msg(chat, msg).await?;
//! ```
//!
//! ### Broadcasts & Multicasts
//!
//! `prepare_broadcast` fetches the recipient list asynchronously, then returns a
//! `MulticastBuilder`. Preview is resolved **only once** and the result is cloned for every
//! recipient.
//!
//! ```ignore
//! // All known chats
//! bot.prepare_broadcast("Hello everyone")
//!     .await?
//!     .send()
//!     .await;
//!
//! // Filtered to direct chats only
//! bot.prepare_broadcast_with("Hello", |id| id.is_direct())
//!     .await?
//!     .send()
//!     .await;
//!
//! // Image preview is transcoded/resolved once, result broadcast to all groups
//! bot.prepare_broadcast_with(Image::new("img.jpg"), |id| id.is_group())
//!     .await?
//!     .send()
//!     .await;
//!
//! // Image with in-memory thumbnail
//! bot.prepare_broadcast(Image::new("img.jpg"))
//!     .await?
//!     .with_preview(ImagePreview::from_bytes(thumb_bytes))
//!     .send()
//!     .await;
//!
//! // Text transitioning to link inside the broadcast builder
//! bot.prepare_broadcast("Check this out")
//!     .await?
//!     .with_link(Link::new("https://example.com").with_title("Example"))
//!     .with_preview(ImagePreview::from_bytes(og_bytes))
//!     .with_ttl(Duration::from_secs(86400))
//!     .send()
//!     .await;
//!
//! // Explicit set of chat IDs
//! bot.multicast_message(chat_ids, Image::new("/tmp/photo.jpg"))
//!     .with_preview(ImagePreview::from_bytes(thumb_bytes))
//!     .await;
//! ```

use serde::Serialize;
use simploxide_api_types::{
    ComposedMessage, CryptoFile, CryptoFileArgs, JsonObject, LinkContent, LinkOwnerSig,
    LinkPreview, MsgChatLink, MsgContent, ReportReason, client_api::ClientApi,
    commands::ApiSendMessages, responses::NewChatItemsResponse,
};

#[cfg(feature = "multimedia")]
use crate::preview;
use crate::{
    id::{ChatId, MessageId},
    preferences,
    preview::{ImagePreview, PreviewKind},
};

use std::{path::Path, pin::Pin, sync::Arc, time::Duration};

/// A kind for simple text messsages
pub struct TextKind;

/// A kind for complex messages(simple attachments, reports, etc) that don't require any
/// pre-processing to be sent
pub struct RichKind;

/// Builder kind for [`ComposedMessage`]. Content is sent verbatim so no builder methods are
/// available for this kind.
pub struct RawKind;

/// Builder kind for messages requiring preview processing. Exposes `with_preview` to override the
/// thumbnail. With the `multimedia` feature, also exposes `with_transcoder` to control JPEG
/// re-encoding at send time.
pub struct PreviewableKind(ImagePreview);

pub trait MessageLike {
    type Kind;
    fn into_builder_parts(self) -> (ComposedMessage, Self::Kind);
}

impl MessageLike for ComposedMessage {
    type Kind = RawKind;
    fn into_builder_parts(self) -> (ComposedMessage, RawKind) {
        (self, RawKind)
    }
}

impl MessageLike for MsgContent {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        (wrap_content(self), RichKind)
    }
}

impl MessageLike for String {
    type Kind = TextKind;
    fn into_builder_parts(self) -> (ComposedMessage, TextKind) {
        (wrap_content(MsgContent::make_text(self)), TextKind)
    }
}

impl MessageLike for &str {
    type Kind = TextKind;
    fn into_builder_parts(self) -> (ComposedMessage, TextKind) {
        self.to_owned().into_builder_parts()
    }
}

/// Represents a styled text(applies SimpleX-Chat markdown syntax to the given substr)
#[derive(Debug, Clone)]
pub enum Text<'a> {
    Bold(&'a str),
    Italic(&'a str),
    Strike(&'a str),
    Monospace(&'a str),
    Secret(&'a str),
    Red(&'a str),
    Green(&'a str),
    Blue(&'a str),
    Yellow(&'a str),
    Cyan(&'a str),
    Magenta(&'a str),
}

impl std::fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start, text, end) = match self {
            Self::Bold(s) => ("*", s, "*"),
            Self::Italic(s) => ("_", s, "_"),
            Self::Strike(s) => ("~", s, "~"),
            Self::Monospace(s) => ("`", s, "`"),
            Self::Secret(s) => ("#", s, "#"),
            Self::Red(s) => ("!1 ", s, "!"),
            Self::Green(s) => ("!2 ", s, "!"),
            Self::Blue(s) => ("!3, ", s, "!"),
            Self::Yellow(s) => ("!4 ", s, "!"),
            Self::Cyan(s) => ("!5 ", s, "!"),
            Self::Magenta(s) => ("!6 ", s, "!"),
        };

        for line in text.lines() {
            if line.trim().is_empty() {
                writeln!(f, "{line}")?;
            } else {
                writeln!(f, "{start}{}{end}", line.trim())?;
            }
        }

        Ok(())
    }
}

/// An extension trait supposed to construct [`Text`] types from string like types, e.g.
///
/// ```ignore
/// format!("Hello, {}", user_name.bold())
/// ```
pub trait TextExt {
    fn bold(&self) -> Text<'_>;
    fn italic(&self) -> Text<'_>;
    fn strike(&self) -> Text<'_>;
    fn monospace(&self) -> Text<'_>;
    fn secret(&self) -> Text<'_>;
    fn red(&self) -> Text<'_>;
    fn green(&self) -> Text<'_>;
    fn blue(&self) -> Text<'_>;
    fn yellow(&self) -> Text<'_>;
    fn cyan(&self) -> Text<'_>;
    fn magenta(&self) -> Text<'_>;
}

impl<S> TextExt for S
where
    S: std::ops::Deref<Target = str>,
{
    fn bold(&self) -> Text<'_> {
        Text::Bold(self)
    }

    fn italic(&self) -> Text<'_> {
        Text::Italic(self)
    }

    fn strike(&self) -> Text<'_> {
        Text::Strike(self)
    }

    fn monospace(&self) -> Text<'_> {
        Text::Monospace(self)
    }

    fn secret(&self) -> Text<'_> {
        Text::Secret(self)
    }

    fn red(&self) -> Text<'_> {
        Text::Red(self)
    }

    fn green(&self) -> Text<'_> {
        Text::Green(self)
    }

    fn blue(&self) -> Text<'_> {
        Text::Blue(self)
    }

    fn yellow(&self) -> Text<'_> {
        Text::Yellow(self)
    }

    fn cyan(&self) -> Text<'_> {
        Text::Cyan(self)
    }

    fn magenta(&self) -> Text<'_> {
        Text::Magenta(self)
    }
}

impl MessageLike for Text<'_> {
    type Kind = TextKind;

    fn into_builder_parts(self) -> (ComposedMessage, Self::Kind) {
        self.to_string().into_builder_parts()
    }
}

impl MessageLike for CryptoFile {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        (
            ComposedMessage {
                file_source: Some(self),
                msg_content: MsgContent::make_file(String::new()),
                quoted_item_id: None,
                mentions: Default::default(),
                undocumented: Default::default(),
            },
            RichKind,
        )
    }
}

/// Image message type. With the `multimedia` feature, auto-transcodes the source file into a
/// thumbnail on resolve when no explicit preview is set. Without it, the gray placeholder is used.
/// With `native_crypto` feature can auto-transcode thumbnails even from the encrypted source files
#[derive(Debug, Clone)]
pub struct Image {
    source: CryptoFile,
    custom_preview: ImagePreview,
    text: String,
}

impl Image {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            source: CryptoFile {
                file_path: path.as_ref().display().to_string(),
                crypto_args: None,
                undocumented: Default::default(),
            },
            custom_preview: ImagePreview::default(),
            text: String::new(),
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.text = caption.into();
        self
    }

    pub fn with_preview(mut self, preview: ImagePreview) -> Self {
        self.custom_preview = preview;
        self
    }

    pub fn with_crypto_args(mut self, args: CryptoFileArgs) -> Self {
        self.source.crypto_args = Some(args);
        self
    }
}

impl From<CryptoFile> for Image {
    fn from(source: CryptoFile) -> Self {
        Self {
            source,
            custom_preview: ImagePreview::default(),
            text: String::new(),
        }
    }
}

impl MessageLike for Image {
    type Kind = PreviewableKind;
    fn into_builder_parts(self) -> (ComposedMessage, PreviewableKind) {
        let preview = if self.custom_preview.kind() != PreviewKind::Default {
            self.custom_preview
        } else {
            make_image_preview(&self.source)
        };

        (
            ComposedMessage {
                file_source: Some(self.source),
                msg_content: MsgContent::make_image(self.text, String::new()),
                quoted_item_id: None,
                mentions: Default::default(),
                undocumented: Default::default(),
            },
            PreviewableKind(preview),
        )
    }
}

#[cfg(all(feature = "multimedia", feature = "native_crypto"))]
fn make_image_preview(file: &CryptoFile) -> ImagePreview {
    ImagePreview::from_crypto_file(file.clone())
}

#[cfg(all(feature = "multimedia", not(feature = "native_crypto")))]
fn make_image_preview(file: &CryptoFile) -> ImagePreview {
    if file.crypto_args.is_none() {
        ImagePreview::from_file(&file.file_path)
    } else {
        ImagePreview::default()
    }
}

#[cfg(not(feature = "multimedia"))]
fn make_image_preview(_: &CryptoFile) -> ImagePreview {
    ImagePreview::default()
}

/// Video message type. Automatic preview generation from video files is unsupported; set a preview
/// explicitly or the default placeholder is used. Your app can generate video previews by calling
/// the external `ffmpeg` process or similar.
#[derive(Debug, Clone)]
pub struct Video {
    source: CryptoFile,
    preview: ImagePreview,
    text: String,
    duration: Duration,
}

impl Video {
    pub fn new(path: impl AsRef<Path>, duration: Duration) -> Self {
        Self {
            source: CryptoFile {
                file_path: path.as_ref().display().to_string(),
                crypto_args: None,
                undocumented: Default::default(),
            },
            preview: ImagePreview::default(),
            text: String::new(),
            duration,
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.text = caption.into();
        self
    }

    pub fn with_preview(mut self, preview: ImagePreview) -> Self {
        self.preview = preview;
        self
    }

    pub fn with_crypto_args(mut self, args: CryptoFileArgs) -> Self {
        self.source.crypto_args = Some(args);
        self
    }
}

impl From<CryptoFile> for Video {
    fn from(source: CryptoFile) -> Self {
        Self {
            source,
            preview: ImagePreview::default(),
            text: String::new(),
            duration: Duration::ZERO,
        }
    }
}

impl MessageLike for Video {
    type Kind = PreviewableKind;
    fn into_builder_parts(self) -> (ComposedMessage, PreviewableKind) {
        (
            ComposedMessage {
                file_source: Some(self.source),
                msg_content: MsgContent::make_video(
                    self.text,
                    String::default(),
                    self.duration.as_secs().try_into().unwrap_or(i32::MAX),
                ),
                quoted_item_id: None,
                mentions: Default::default(),
                undocumented: Default::default(),
            },
            PreviewableKind(self.preview),
        )
    }
}

/// Link preview message. Use `with_title`, `with_description`, and `with_image` to populate
/// the Open Graph-style card shown to the recipient.
#[derive(Debug, Clone)]
pub struct Link {
    uri: String,
    title: String,
    description: String,
    image: ImagePreview,
    content: Option<LinkContent>,
    text: String,
}

impl Link {
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            title: String::new(),
            description: String::new(),
            image: ImagePreview::default(),
            content: None,
            text: String::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_image(mut self, image: ImagePreview) -> Self {
        self.image = image;
        self
    }

    pub fn with_content(mut self, content: LinkContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl MessageLike for Link {
    type Kind = PreviewableKind;
    fn into_builder_parts(self) -> (ComposedMessage, PreviewableKind) {
        (
            ComposedMessage {
                file_source: None,
                msg_content: MsgContent::make_link(
                    self.text,
                    LinkPreview {
                        uri: self.uri,
                        title: self.title,
                        description: self.description,
                        image: String::new(),
                        content: self.content,
                        undocumented: Default::default(),
                    },
                ),
                quoted_item_id: None,
                mentions: Default::default(),
                undocumented: Default::default(),
            },
            PreviewableKind(self.image),
        )
    }
}

/// Simple file attachment
#[derive(Debug, Clone)]
pub struct File {
    pub text: String,
    pub file: CryptoFile,
}

impl File {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            file: CryptoFile {
                file_path: path.as_ref().display().to_string(),
                crypto_args: None,
                undocumented: Default::default(),
            },
            text: String::new(),
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.text = caption.into();
        self
    }

    pub fn with_crypto_args(mut self, args: CryptoFileArgs) -> Self {
        self.file.crypto_args = Some(args);
        self
    }
}

impl MessageLike for File {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        (
            ComposedMessage {
                file_source: Some(self.file),
                msg_content: MsgContent::make_file(self.text),
                quoted_item_id: None,
                mentions: Default::default(),
                undocumented: Default::default(),
            },
            RichKind,
        )
    }
}

/// A message sent to groups to report other users
#[derive(Debug, Clone)]
pub struct Report {
    pub text: String,
    pub reason: ReportReason,
}

impl Report {
    pub fn spam<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            reason: ReportReason::Spam,
        }
    }

    pub fn content<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            reason: ReportReason::Content,
        }
    }

    pub fn community<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            reason: ReportReason::Community,
        }
    }

    pub fn profile<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            reason: ReportReason::Profile,
        }
    }

    pub fn other<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            reason: ReportReason::Other,
        }
    }
}

impl MessageLike for Report {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        (
            wrap_content(MsgContent::make_report(self.text, self.reason)),
            RichKind,
        )
    }
}

impl MessageLike for ReportReason {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        Report {
            text: String::new(),
            reason: self,
        }
        .into_builder_parts()
    }
}

/// Chat invitation message containing a link to a group or direct contact.
#[derive(Debug, Clone)]
pub struct Chat {
    pub text: String,
    pub link: MsgChatLink,
    pub owner_sig: Option<LinkOwnerSig>,
}

impl Chat {
    pub fn new(link: MsgChatLink) -> Self {
        Self {
            text: String::new(),
            link,
            owner_sig: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_owner_sig(mut self, sig: LinkOwnerSig) -> Self {
        self.owner_sig = Some(sig);
        self
    }
}

impl MessageLike for Chat {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        (
            wrap_content(MsgContent::make_chat(self.text, self.link, self.owner_sig)),
            RichKind,
        )
    }
}

/// Application defined message with a string tag and arbitrary JSON payload.
#[derive(Debug, Clone)]
pub struct Custom {
    pub tag: String,
    pub text: String,
    pub json: JsonObject,
}

impl Custom {
    pub fn new(tag: impl Into<String>, object: impl Serialize) -> Self {
        // TODO: handle serialize error
        Self::from_raw(tag.into(), serde_json::to_value(object).unwrap())
    }

    pub fn from_raw(tag: String, json: JsonObject) -> Self {
        Self {
            tag,
            text: String::new(),
            json,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl MessageLike for Custom {
    type Kind = RichKind;
    fn into_builder_parts(self) -> (ComposedMessage, RichKind) {
        (
            wrap_content(MsgContent::make_unknown(self.tag, self.text, self.json)),
            RichKind,
        )
    }
}

/// An awaitable message builder(await sends the message)
pub struct MessageBuilder<'a, C: 'a + ?Sized, M = TextKind> {
    pub(crate) client: &'a C,
    pub(crate) chat_id: ChatId,
    pub(crate) live: bool,
    pub(crate) ttl: Option<Duration>,
    pub(crate) msg: ComposedMessage,
    pub(crate) kind: M,
}

impl<'a, C, M> MessageBuilder<'a, C, M> {
    pub fn live_message(mut self) -> Self {
        self.live = true;
        self
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn reply_to(mut self, msg_id: impl Into<MessageId>) -> Self {
        self.msg.quoted_item_id = Some(msg_id.into().0);
        self
    }

    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        self.msg.msg_content.set_text_part(text);
        self
    }

    /// A syntactic sugar to avoid double awaits(`.await.await` -> `.await.send().await`) in
    /// certain use-cases
    pub fn send(self) -> <Self as IntoFuture>::IntoFuture
    where
        Self: IntoFuture,
    {
        self.into_future()
    }
}

impl<'a, C> MessageBuilder<'a, C, TextKind> {
    pub fn with_image(self, img: Image) -> MessageBuilder<'a, C, PreviewableKind> {
        let (msg, kind) = fuse_messages(self.msg, img);

        MessageBuilder {
            client: self.client,
            chat_id: self.chat_id,
            live: self.live,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn with_video(self, vid: Video) -> MessageBuilder<'a, C, PreviewableKind> {
        let (msg, kind) = fuse_messages(self.msg, vid);

        MessageBuilder {
            client: self.client,
            chat_id: self.chat_id,
            live: self.live,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn with_link(self, link: Link) -> MessageBuilder<'a, C, PreviewableKind> {
        let (msg, kind) = fuse_messages(self.msg, link);

        MessageBuilder {
            client: self.client,
            chat_id: self.chat_id,
            live: self.live,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn attach(self, img: CryptoFile) -> MessageBuilder<'a, C, RichKind> {
        let (msg, kind) = fuse_messages(self.msg, img);

        MessageBuilder {
            client: self.client,
            chat_id: self.chat_id,
            live: self.live,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn report(self, reason: ReportReason) -> MessageBuilder<'a, C, RichKind> {
        let (msg, kind) = fuse_messages(self.msg, reason);

        MessageBuilder {
            client: self.client,
            chat_id: self.chat_id,
            live: self.live,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn link_chat(self, chat: Chat) -> MessageBuilder<'a, C, RichKind> {
        let (msg, kind) = fuse_messages(self.msg, chat);

        MessageBuilder {
            client: self.client,
            chat_id: self.chat_id,
            live: self.live,
            ttl: self.ttl,
            msg,
            kind,
        }
    }
}

impl<'a, C> MessageBuilder<'a, C, RichKind> {
    pub fn set_file_source(mut self, source: CryptoFile) -> Self {
        self.msg.file_source = Some(source);
        self
    }
}

impl<'a, C> MessageBuilder<'a, C, PreviewableKind> {
    /// Override the current image preview with a custom one
    pub fn with_preview(mut self, preview: ImagePreview) -> Self {
        self.kind.0 = preview;
        self
    }

    #[cfg(feature = "multimedia")]
    /// Alter the default preview transcoder
    pub fn with_transcoder(mut self, transcoder: preview::Transcoder) -> Self {
        self.kind.0.set_transcoder(transcoder);
        self
    }
}

mod sealed {
    pub trait SimplySendable {}
    impl SimplySendable for super::TextKind {}
    impl SimplySendable for super::RichKind {}
    impl SimplySendable for super::RawKind {}
}

impl<'a, C, M> IntoFuture for MessageBuilder<'a, C, M>
where
    C: 'static + ClientApi,
    C::Error: 'static + Send,
    M: sealed::SimplySendable,
{
    type Output = Result<Arc<NewChatItemsResponse>, C::Error>;
    type IntoFuture = Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.client.api_send_messages(ApiSendMessages {
            send_ref: self.chat_id.into_chat_ref(),
            live_message: self.live,
            ttl: self.ttl.map(preferences::timed_messages::ttl_to_secs),
            composed_messages: vec![self.msg],
        }))
    }
}

impl<'a, C> IntoFuture for MessageBuilder<'a, C, PreviewableKind>
where
    C: 'static + ClientApi,
    C::Error: 'static + Send,
{
    type Output = Result<Arc<NewChatItemsResponse>, C::Error>;
    type IntoFuture = Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let preview_data = self.kind.0.resolve().await;
            let mut msg = self.msg;
            msg.msg_content.set_preview(preview_data);

            self.client
                .api_send_messages(ApiSendMessages {
                    send_ref: self.chat_id.into_chat_ref(),
                    live_message: self.live,
                    ttl: self.ttl.map(preferences::timed_messages::ttl_to_secs),
                    composed_messages: vec![msg],
                })
                .await
        })
    }
}

pub struct MulticastBuilder<'a, I, C: 'a + ?Sized, M = TextKind> {
    pub(crate) client: &'a C,
    pub(crate) chat_ids: I,
    pub(crate) ttl: Option<Duration>,
    pub(crate) msg: ComposedMessage,
    pub(crate) kind: M,
}

impl<'a, I, C, M> MulticastBuilder<'a, I, C, M> {
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        self.msg.msg_content.set_text_part(text);
        self
    }

    /// A syntactic sugar to avoid double awaits(`.await.await` -> `.await.send().await`) in
    /// certain use-cases
    pub fn send(self) -> <Self as IntoFuture>::IntoFuture
    where
        Self: IntoFuture,
    {
        self.into_future()
    }
}

impl<'a, I, C> MulticastBuilder<'a, I, C, TextKind> {
    pub fn with_image(self, img: Image) -> MulticastBuilder<'a, I, C, PreviewableKind> {
        let (msg, kind) = fuse_messages(self.msg, img);

        MulticastBuilder {
            client: self.client,
            chat_ids: self.chat_ids,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn with_video(self, vid: Video) -> MulticastBuilder<'a, I, C, PreviewableKind> {
        let (msg, kind) = fuse_messages(self.msg, vid);

        MulticastBuilder {
            client: self.client,
            chat_ids: self.chat_ids,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn with_link(self, link: Link) -> MulticastBuilder<'a, I, C, PreviewableKind> {
        let (msg, kind) = fuse_messages(self.msg, link);

        MulticastBuilder {
            client: self.client,
            chat_ids: self.chat_ids,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn attach(self, img: CryptoFile) -> MulticastBuilder<'a, I, C, RichKind> {
        let (msg, kind) = fuse_messages(self.msg, img);

        MulticastBuilder {
            client: self.client,
            chat_ids: self.chat_ids,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn report(self, reason: ReportReason) -> MulticastBuilder<'a, I, C, RichKind> {
        let (msg, kind) = fuse_messages(self.msg, reason);

        MulticastBuilder {
            client: self.client,
            chat_ids: self.chat_ids,
            ttl: self.ttl,
            msg,
            kind,
        }
    }

    pub fn link_chat(self, chat: Chat) -> MulticastBuilder<'a, I, C, RichKind> {
        let (msg, kind) = fuse_messages(self.msg, chat);

        MulticastBuilder {
            client: self.client,
            chat_ids: self.chat_ids,
            ttl: self.ttl,
            msg,
            kind,
        }
    }
}

impl<'a, I, C> MulticastBuilder<'a, I, C, RichKind> {
    pub fn set_file_source(mut self, source: CryptoFile) -> Self {
        self.msg.file_source = Some(source);
        self
    }
}

impl<'a, I, C> MulticastBuilder<'a, I, C, PreviewableKind> {
    /// Override the current image preview with a custom one
    pub fn with_preview(mut self, preview: ImagePreview) -> Self {
        self.kind.0 = preview;
        self
    }

    #[cfg(feature = "multimedia")]
    pub fn with_transcoder(mut self, transcoder: preview::Transcoder) -> Self {
        self.kind.0.set_transcoder(transcoder);
        self
    }
}

impl<'a, I, C, M> IntoFuture for MulticastBuilder<'a, I, C, M>
where
    I: IntoIterator<Item = ChatId>,
    C: 'static + ClientApi,
    C::Error: 'static + Send,
    M: sealed::SimplySendable,
{
    type Output = Vec<Result<Arc<NewChatItemsResponse>, C::Error>>;
    type IntoFuture = Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            client,
            chat_ids,
            ttl,
            msg,
            kind: _,
        } = self;

        let iter = chat_ids.into_iter().map(move |id| {
            let msg = msg.clone();
            async move {
                let command = ApiSendMessages {
                    send_ref: id.into_chat_ref(),
                    live_message: false,
                    ttl: ttl.map(preferences::timed_messages::ttl_to_secs),
                    composed_messages: vec![msg],
                };

                client.api_send_messages(command).await
            }
        });

        Box::pin(futures::future::join_all(iter))
    }
}

impl<'a, I, C> IntoFuture for MulticastBuilder<'a, I, C, PreviewableKind>
where
    I: 'static + Send + IntoIterator<Item = ChatId>,
    C: 'static + ClientApi,
    C::Error: 'static + Send,
{
    type Output = Vec<Result<Arc<NewChatItemsResponse>, C::Error>>;
    type IntoFuture = Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            client,
            chat_ids,
            ttl,
            mut msg,
            kind,
        } = self;

        Box::pin(async move {
            let preview_data = kind.0.resolve().await;
            msg.msg_content.set_preview(preview_data);

            let iter = chat_ids.into_iter().map(move |id| {
                let msg = msg.clone();
                async move {
                    let command = ApiSendMessages {
                        send_ref: id.into_chat_ref(),
                        live_message: false,
                        ttl: ttl.map(preferences::timed_messages::ttl_to_secs),
                        composed_messages: vec![msg],
                    };

                    client.api_send_messages(command).await
                }
            });

            futures::future::join_all(iter).await
        })
    }
}

fn fuse_messages<M: MessageLike>(old: ComposedMessage, new: M) -> (ComposedMessage, M::Kind) {
    let (mut new, kind) = new.into_builder_parts();
    new.quoted_item_id = old.quoted_item_id;

    if new.msg_content.text_part().unwrap_or_default().is_empty() {
        new.msg_content.set_text_part(
            old.msg_content
                .text_part()
                .map(|s| s.to_owned())
                .unwrap_or_default(),
        );
    }

    (new, kind)
}

fn wrap_content(msg_content: MsgContent) -> ComposedMessage {
    ComposedMessage {
        file_source: None,
        quoted_item_id: None,
        msg_content,
        mentions: Default::default(),
        undocumented: Default::default(),
    }
}

pub trait MsgContentExt {
    fn text_part(&self) -> Option<&str>;

    fn text_part_mut(&mut self) -> Option<&mut String>;

    fn set_text_part(&mut self, new_text: impl Into<String>) {
        if let Some(text) = self.text_part_mut() {
            *text = new_text.into();
        }
    }

    fn preview(&self) -> Option<&str>;

    fn preview_mut(&mut self) -> Option<&mut String>;

    fn set_preview(&mut self, new_preview: String) {
        if let Some(preview) = self.preview_mut() {
            *preview = new_preview;
        }
    }
}

impl MsgContentExt for MsgContent {
    fn text_part(&self) -> Option<&str> {
        match self {
            MsgContent::Text { text, .. }
            | MsgContent::Link { text, .. }
            | MsgContent::Image { text, .. }
            | MsgContent::Video { text, .. }
            | MsgContent::Voice { text, .. }
            | MsgContent::File { text, .. }
            | MsgContent::Report { text, .. }
            | MsgContent::Chat { text, .. }
            | MsgContent::Unknown { text, .. } => Some(text),
            _ => None,
        }
    }

    fn text_part_mut(&mut self) -> Option<&mut String> {
        match self {
            MsgContent::Text { text, .. }
            | MsgContent::Link { text, .. }
            | MsgContent::Image { text, .. }
            | MsgContent::Video { text, .. }
            | MsgContent::Voice { text, .. }
            | MsgContent::File { text, .. }
            | MsgContent::Report { text, .. }
            | MsgContent::Chat { text, .. }
            | MsgContent::Unknown { text, .. } => Some(text),
            _ => None,
        }
    }

    fn preview(&self) -> Option<&str> {
        match self {
            MsgContent::Link {
                preview: LinkPreview { image, .. },
                ..
            }
            | MsgContent::Image { image, .. }
            | MsgContent::Video { image, .. } => Some(image),
            _ => None,
        }
    }

    fn preview_mut(&mut self) -> Option<&mut String> {
        match self {
            MsgContent::Link {
                preview: LinkPreview { image, .. },
                ..
            }
            | MsgContent::Image { image, .. }
            | MsgContent::Video { image, .. } => Some(image),
            _ => None,
        }
    }
}
