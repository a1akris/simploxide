use simploxide_api_types::{
    AChatItem, ChatItem, ComposedMessage, MsgContent, NewUser,
    client_api::{ClientApi, ClientApiError as _},
    commands::ApiSendMessages,
    responses::{ActiveUserResponse, NewChatItemsResponse},
};

use std::{pin::Pin, sync::Arc, time::Duration};

use crate::id::ChatId;

pub type NewUserResponse<C> = Result<Arc<ActiveUserResponse>, <C as ClientApi>::Error>;

pub trait ClientApiExt: ClientApi {
    /// Like [ClientApi::create_active_user] but ensures that user is created even if the name
    /// contains invalid UTF-8 characters. The user struct gets cloned when performing the original
    /// request
    fn new_user(&self, user: NewUser) -> impl Future<Output = NewUserResponse<Self>>;

    fn send_message<M: MessageLike>(&self, chat_id: ChatId, msg: M) -> MessageBuilder<'_, Self>;
}

impl<C> ClientApiExt for C
where
    C: ClientApi,
{
    async fn new_user(&self, mut user: NewUser) -> NewUserResponse<Self> {
        match self.create_active_user(user.clone()).await {
            Ok(response) => Ok(response),
            Err(e) => match e.bad_response().and_then(|e| {
                e.chat_error()
                    .and_then(|e| e.error().and_then(|e| e.invalid_display_name()))
            }) {
                Some(err) => {
                    user.profile.as_mut().unwrap().display_name = err.valid_name.clone();
                    self.create_active_user(user).await
                }
                None => Err(e),
            },
        }
    }

    fn send_message<M: MessageLike>(&self, chat_id: ChatId, msg: M) -> MessageBuilder<'_, Self> {
        MessageBuilder {
            client: self,
            chat_id,
            live: false,
            ttl: None,
            msg: msg.into_composed_message(),
        }
    }
}

pub trait FilterChatItems {
    fn filter_messages(&self) -> impl Iterator<Item = (ChatId, &ChatItem, &MsgContent)>;
}

impl FilterChatItems for Vec<AChatItem> {
    fn filter_messages(&self) -> impl Iterator<Item = (ChatId, &ChatItem, &MsgContent)> {
        self.iter().filter_map(|item| {
            ChatId::from_chat_info(&item.chat_info).and_then(|cid| {
                item.chat_item
                    .content
                    .rcv_msg_content()
                    .map(|msg| (cid, &item.chat_item, msg))
            })
        })
    }
}

pub trait MessageLike {
    fn into_composed_message(self) -> ComposedMessage;
}

impl MessageLike for ComposedMessage {
    fn into_composed_message(self) -> ComposedMessage {
        self
    }
}

impl MessageLike for String {
    fn into_composed_message(self) -> ComposedMessage {
        ComposedMessage {
            file_source: None,
            quoted_item_id: None,
            msg_content: MsgContent::make_text(self),
            mentions: Default::default(),
            undocumented: Default::default(),
        }
    }
}

impl MessageLike for &str {
    fn into_composed_message(self) -> ComposedMessage {
        String::into_composed_message(self.to_owned())
    }
}

pub struct MessageBuilder<'a, C: ?Sized> {
    client: &'a C,
    chat_id: ChatId,
    live: bool,
    ttl: Option<Duration>,
    msg: ComposedMessage,
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

    pub fn reply_to(mut self, item_id: i64) -> Self {
        self.msg.quoted_item_id = Some(item_id);
        self
    }
}

impl<'a, C: 'static + ClientApi> IntoFuture for MessageBuilder<'a, C> {
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
