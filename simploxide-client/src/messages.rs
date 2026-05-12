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

/// Simple text messages
pub struct TextKind;
/// Text with attachments or special message types(e.g. Report)
pub struct RichKind;
/// Raw ComposedMessage
pub struct RawKind;

/// Messages requiring generating preview images
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

#[cfg(all(not(feature = "multimedia"), not(feature = "native_crypto")))]
fn make_image_preview(_: &CryptoFile) -> ImagePreview {
    ImagePreview::default()
}

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

pub struct Custom {
    pub tag: String,
    pub text: String,
    pub json: JsonObject,
}

impl Custom {
    pub fn new(tag: impl Into<String>, object: impl Serialize) -> Self {
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
