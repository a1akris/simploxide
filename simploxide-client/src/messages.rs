use serde::Serialize;
use simploxide_api_types::{
    ComposedMessage, CryptoFile, CryptoFileArgs, JsonObject, LinkOwnerSig, MsgChatLink, MsgContent,
    ReportReason, client_api::ClientApi, commands::ApiSendMessages,
    responses::NewChatItemsResponse,
};

use std::{pin::Pin, sync::Arc, time::Duration};

use crate::id::{ChatId, MessageId};

pub trait MessageLike {
    fn into_composed_message(self) -> ComposedMessage;
}

impl MessageLike for ComposedMessage {
    fn into_composed_message(self) -> ComposedMessage {
        self
    }
}

impl MessageLike for MsgContent {
    fn into_composed_message(self) -> ComposedMessage {
        ComposedMessage {
            file_source: None,
            quoted_item_id: None,
            msg_content: self,
            mentions: Default::default(),
            undocumented: Default::default(),
        }
    }
}

impl MessageLike for String {
    fn into_composed_message(self) -> ComposedMessage {
        MsgContent::make_text(self).into_composed_message()
    }
}

impl MessageLike for &str {
    fn into_composed_message(self) -> ComposedMessage {
        String::into_composed_message(self.to_owned())
    }
}

impl MessageLike for CryptoFile {
    fn into_composed_message(self) -> ComposedMessage {
        ComposedMessage {
            file_source: Some(self),
            msg_content: MsgContent::make_file(String::default()),
            quoted_item_id: None,
            mentions: Default::default(),
            undocumented: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub text: String,
    pub crypto_args: Option<CryptoFileArgs>,
}

impl File {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().display().to_string(),
            text: String::new(),
            crypto_args: None,
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.text = caption.into();
        self
    }

    pub fn with_crypto_args(mut self, args: CryptoFileArgs) -> Self {
        self.crypto_args = Some(args);
        self
    }
}

impl MessageLike for File {
    fn into_composed_message(self) -> ComposedMessage {
        let file_source = CryptoFile {
            file_path: self.path,
            crypto_args: self.crypto_args,
            undocumented: Default::default(),
        };

        ComposedMessage {
            file_source: Some(file_source),
            msg_content: MsgContent::make_file(self.text),
            quoted_item_id: None,
            mentions: Default::default(),
            undocumented: Default::default(),
        }
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
    fn into_composed_message(self) -> ComposedMessage {
        MsgContent::make_report(self.text, self.reason).into_composed_message()
    }
}

impl MessageLike for ReportReason {
    fn into_composed_message(self) -> ComposedMessage {
        Report {
            text: String::new(),
            reason: self,
        }
        .into_composed_message()
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
    fn into_composed_message(self) -> ComposedMessage {
        MsgContent::make_chat(self.text, self.link, self.owner_sig).into_composed_message()
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
}

impl Custom {
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl MessageLike for Custom {
    fn into_composed_message(self) -> ComposedMessage {
        MsgContent::make_unknown(self.tag, self.text, self.json).into_composed_message()
    }
}

pub struct MessageBuilder<'a, C: 'a + ?Sized> {
    pub(crate) client: &'a C,
    pub(crate) chat_id: ChatId,
    pub(crate) live: bool,
    pub(crate) ttl: Option<Duration>,
    pub(crate) msg: ComposedMessage,
}

impl<'a, C> MessageBuilder<'a, C> {
    pub fn live_message(mut self) -> Self {
        self.live = true;
        self
    }

    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn reply_to(mut self, msg_id: impl Into<MessageId>) -> Self {
        self.msg.quoted_item_id = Some(msg_id.into().0);
        self
    }

    /// Replaces [MsgContent] with [MsgContent::Report] preserving the text
    pub fn report(mut self, reason: ReportReason) -> Self {
        let text = self.take_text();
        self.msg.msg_content = MsgContent::make_report(text, reason);
        self
    }

    /// Replaces [MsgContent] with [MsgContent::Chat]
    ///
    /// If [MsgContent::Chat] text is not set reuses the current message text if any
    pub fn link_chat(mut self, chat: Chat) -> Self {
        let text = if chat.text.is_empty() {
            self.take_text()
        } else {
            chat.text
        };

        self.msg.msg_content = MsgContent::make_chat(text, chat.link, chat.owner_sig);
        self
    }

    /// Sets message text preserving current [MsgContent]
    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        if let Some(s) = self.msg.msg_content.text_mut() {
            *s = text.into();
        }

        self
    }

    /// Overrides the file source without changing the [MsgContent]
    pub fn set_file_source(mut self, source: CryptoFile) -> Self {
        self.msg.file_source = Some(source);
        self
    }

    fn take_text(&mut self) -> String {
        self.msg
            .msg_content
            .text_mut()
            .map(std::mem::take)
            .unwrap_or_default()
    }
}

impl<'a, C> IntoFuture for MessageBuilder<'a, C>
where
    C: 'static + ClientApi,
    C::Error: 'static + Send,
{
    type Output = Result<Arc<NewChatItemsResponse>, C::Error>;
    type IntoFuture = Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        let command = ApiSendMessages {
            send_ref: self.chat_id.into_chat_ref(),
            live_message: self.live,
            ttl: self.ttl.map(|ttl| {
                std::cmp::min(ttl, crate::preferences::timed_messages::TTL_MAX).as_secs() as i32
            }),
            composed_messages: vec![self.msg],
        };

        Box::pin(self.client.api_send_messages(command))
    }
}

pub struct MulticastBuilder<'a, I, C: 'a + ?Sized> {
    pub(crate) client: &'a C,
    pub(crate) chat_ids: I,
    pub(crate) ttl: Option<Duration>,
    pub(crate) msg: ComposedMessage,
}

impl<'a, I, C> MulticastBuilder<'a, I, C> {
    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

impl<'a, I, C> IntoFuture for MulticastBuilder<'a, I, C>
where
    I: IntoIterator<Item = ChatId>,
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
            msg,
        } = self;

        let iter = chat_ids.into_iter().map(move |id| {
            let msg = msg.clone();
            async move {
                let command = ApiSendMessages {
                    send_ref: id.into_chat_ref(),
                    live_message: false,
                    ttl: ttl.map(|ttl| {
                        std::cmp::min(ttl, crate::preferences::timed_messages::TTL_MAX).as_secs()
                            as i32
                    }),
                    composed_messages: vec![msg],
                };

                client.api_send_messages(command).await
            }
        });

        Box::pin(futures::future::join_all(iter))
    }
}

pub trait MsgContentExt {
    fn text(&self) -> Option<&str>;

    fn text_mut(&mut self) -> Option<&mut String>;
}

impl MsgContentExt for MsgContent {
    fn text(&self) -> Option<&str> {
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
            MsgContent::Undocumented(_) => None,
            _ => None,
        }
    }

    fn text_mut(&mut self) -> Option<&mut String> {
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
            MsgContent::Undocumented(_) => None,
            _ => None,
        }
    }
}
