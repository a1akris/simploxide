use simploxide_api_types::{
    AChatItem, ChatDeleteMode, ChatItem, ComposedMessage, Contact, GroupInfo, MsgContent, NewUser,
    UserInfo,
    client_api::{ClientApi, ClientApiError as _},
    commands::{ApiListGroups, ApiSendMessages, Connect},
    responses::{ActiveUserResponse, ApiDeleteChatResponse, ConnectResponse, NewChatItemsResponse},
};

use std::{pin::Pin, sync::Arc, time::Duration};

use crate::id::{ChatId, UserId};

pub type UsersResponse<C> = Result<Vec<UserInfo>, <C as ClientApi>::Error>;
pub type ContactsResponse<C> = Result<Vec<Contact>, <C as ClientApi>::Error>;
pub type GroupsResponse<C> = Result<Vec<GroupInfo>, <C as ClientApi>::Error>;
pub type NewUserResponse<C> = Result<Arc<ActiveUserResponse>, <C as ClientApi>::Error>;
pub type InitiateConnectionResponse<C> = Result<ConnectResponse, <C as ClientApi>::Error>;
pub type DeleteChatResponse<C> = Result<ApiDeleteChatResponse, <C as ClientApi>::Error>;

pub trait ClientApiExt: ClientApi {
    fn users(&self) -> impl Future<Output = UsersResponse<Self>>;

    fn contacts(&self, user_id: UserId) -> impl Future<Output = ContactsResponse<Self>>;

    fn groups(&self, user_id: UserId) -> impl Future<Output = GroupsResponse<Self>>;

    /// Like [ClientApi::create_active_user] but ensures that user is created even if the name
    /// contains invalid UTF-8 characters. The user struct gets cloned when performing the original
    /// request
    fn new_user(&self, user: NewUser) -> impl Future<Output = NewUserResponse<Self>>;

    /// [ChatId] can be created from various types. See [ChatId] docs for all `From` impls
    fn send_message<CID: Into<ChatId>, M: MessageLike>(
        &self,
        chat_id: CID,
        msg: M,
    ) -> MessageBuilder<'_, Self>;

    fn multicast_message<I, M>(&self, chat_ids: I, msg: M) -> MulticastBuilder<'_, I, Self>
    where
        I: IntoIterator<Item = ChatId>,
        M: MessageLike;

    fn initiate_connection(
        &self,
        link: impl Into<String>,
    ) -> impl Future<Output = InitiateConnectionResponse<Self>>;

    fn delete_chat<CID: Into<ChatId>>(
        &self,
        chat_id: CID,
        mode: DeleteMode,
    ) -> impl Future<Output = DeleteChatResponse<Self>>;
}

impl<C> ClientApiExt for C
where
    C: ClientApi,
{
    async fn users(&self) -> UsersResponse<Self> {
        let mut response = self.list_users().await?;
        let response = Arc::get_mut(&mut response).unwrap();

        Ok(std::mem::take(&mut response.users))
    }

    async fn contacts(&self, user_id: UserId) -> ContactsResponse<Self> {
        let mut response = self.api_list_contacts(user_id.0).await?;
        let response = Arc::get_mut(&mut response).unwrap();

        Ok(std::mem::take(&mut response.contacts))
    }

    async fn groups(&self, user_id: UserId) -> GroupsResponse<Self> {
        let mut response = self.api_list_groups(ApiListGroups::new(user_id.0)).await?;
        let response = Arc::get_mut(&mut response).unwrap();

        Ok(std::mem::take(&mut response.groups))
    }

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

    fn send_message<CID: Into<ChatId>, M: MessageLike>(
        &self,
        cid: CID,
        msg: M,
    ) -> MessageBuilder<'_, Self> {
        MessageBuilder {
            client: self,
            chat_id: cid.into(),
            live: false,
            ttl: None,
            msg: msg.into_composed_message(),
        }
    }

    fn multicast_message<I, M>(&self, chat_ids: I, msg: M) -> MulticastBuilder<'_, I, Self>
    where
        I: IntoIterator<Item = ChatId>,
        M: MessageLike,
    {
        MulticastBuilder {
            client: self,
            chat_ids,
            ttl: None,
            msg: msg.into_composed_message(),
        }
    }

    async fn initiate_connection(
        &self,
        link: impl Into<String>,
    ) -> InitiateConnectionResponse<Self> {
        self.connect(Connect {
            incognito: false,
            conn_link: Some(link.into()),
        })
        .await
    }

    async fn delete_chat<CID: Into<ChatId>>(
        &self,
        chat_id: CID,
        mode: DeleteMode,
    ) -> DeleteChatResponse<Self> {
        self.api_delete_chat(chat_id.into().into_chat_ref(), mode.into())
            .await
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

#[derive(Debug, Clone, Copy)]
pub enum DeleteMode {
    Full { notify: bool },
    Entity { notify: bool },
    Messages,
}

impl Default for DeleteMode {
    fn default() -> Self {
        Self::Full { notify: true }
    }
}

impl From<DeleteMode> for ChatDeleteMode {
    fn from(mode: DeleteMode) -> Self {
        match mode {
            DeleteMode::Full { notify } => ChatDeleteMode::Full {
                notify,
                undocumented: Default::default(),
            },
            DeleteMode::Entity { notify } => ChatDeleteMode::Entity {
                notify,
                undocumented: Default::default(),
            },
            DeleteMode::Messages => ChatDeleteMode::Messages,
        }
    }
}

// This impl mainly exist to catch breaking changes
impl TryFrom<ChatDeleteMode> for DeleteMode {
    type Error = ChatDeleteMode;

    fn try_from(mode: ChatDeleteMode) -> Result<Self, Self::Error> {
        match mode {
            ChatDeleteMode::Full {
                notify,
                undocumented: _,
            } => Ok(Self::Full { notify }),
            ChatDeleteMode::Entity {
                notify,
                undocumented: _,
            } => Ok(Self::Entity { notify }),
            ChatDeleteMode::Messages => Ok(Self::Messages),
            ChatDeleteMode::Undocumented(_) => Err(mode),
            _ => Err(mode),
        }
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

pub struct MessageBuilder<'a, C: 'a + ?Sized> {
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
    client: &'a C,
    chat_ids: I,
    ttl: Option<Duration>,
    msg: ComposedMessage,
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
