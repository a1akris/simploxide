use futures::FutureExt as _;
use simploxide_api_types::{
    AChatItem, CIDeleteMode, ChatDeleteMode, ChatItem, Contact, GroupInfo, MsgContent, MsgReaction,
    NewUser, UpdatedMessage, UserInfo,
    client_api::{
        AllowUndocumentedResponses as _, ClientApi, ClientApiError as _, UndocumentedResponse,
    },
    commands::{ApiChatItemReaction, ApiListGroups, ApiUpdateChatItem, Connect, ReceiveFile},
    responses::{
        AcceptingContactRequestResponse, ActiveUserResponse, ApiDeleteChatResponse,
        ApiUpdateChatItemResponse, CancelFileResponse, ChatItemReactionResponse,
        ChatItemsDeletedResponse, ConnectResponse, ContactRequestRejectedResponse,
        ReceiveFileResponse,
    },
};

use std::{pin::Pin, sync::Arc};

use crate::{
    id::{ChatId, ContactRequestId, FileId, MessageId, UserId},
    messages::{MessageBuilder, MessageLike, MulticastBuilder},
};

pub type InitiateConnectionResponse<C> =
    Result<UndocumentedResponse<ConnectResponse>, <C as ClientApi>::Error>;

pub type AcceptContactResponse<C> =
    Result<Arc<AcceptingContactRequestResponse>, <C as ClientApi>::Error>;
pub type RejectContactResponse<C> =
    Result<Arc<ContactRequestRejectedResponse>, <C as ClientApi>::Error>;

pub type RejectFileResponse<C> = Result<CancelFileResponse, <C as ClientApi>::Error>;

pub type ContactsResponse<C> = Result<Vec<Contact>, <C as ClientApi>::Error>;
pub type GroupsResponse<C> = Result<Vec<GroupInfo>, <C as ClientApi>::Error>;

pub type DeleteChatResponse<C> = Result<ApiDeleteChatResponse, <C as ClientApi>::Error>;
pub type DeleteMessageResponse<C> = Result<Arc<ChatItemsDeletedResponse>, <C as ClientApi>::Error>;

pub type UpdateMessageReactionsResponse<C> =
    Vec<Result<Arc<ChatItemReactionResponse>, <C as ClientApi>::Error>>;
pub type UpdateMessageResponse<C> = Result<ApiUpdateChatItemResponse, <C as ClientApi>::Error>;

pub type NewUserResponse<C> = Result<Arc<ActiveUserResponse>, <C as ClientApi>::Error>;
pub type UsersResponse<C> = Result<Vec<UserInfo>, <C as ClientApi>::Error>;

pub trait ClientApiExt: ClientApi {
    fn users(&self) -> impl Future<Output = UsersResponse<Self>>;

    fn contacts<UID: Into<UserId>>(
        &self,
        user_id: UID,
    ) -> impl Future<Output = ContactsResponse<Self>>;

    fn groups<UID: Into<UserId>>(&self, user_id: UID)
    -> impl Future<Output = GroupsResponse<Self>>;

    fn accept_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> impl Future<Output = AcceptContactResponse<Self>>;

    fn reject_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> impl Future<Output = RejectContactResponse<Self>>;

    /// Like [ClientApi::create_active_user] but ensures that user is created even if the name
    /// contains disallowed in SimpleX-Chat UTF-8 characters. The [NewUser] struct gets cloned when
    /// performing the original request
    fn new_user(&self, user: NewUser) -> impl Future<Output = NewUserResponse<Self>>;

    /// Returns a powerful awaitable [MessageBuilder] type. Check its docs to learn how to build
    /// any message kind ergonomically
    fn send_message<CID: Into<ChatId>, M: MessageLike>(
        &self,
        chat_id: CID,
        msg: M,
    ) -> MessageBuilder<'_, Self>;

    /// Deliver the same message to multiple recepients
    fn multicast_message<I, M>(&self, chat_ids: I, msg: M) -> MulticastBuilder<'_, I, Self>
    where
        I: IntoIterator<Item = ChatId>,
        M: MessageLike;

    fn update_message<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        new_content: MsgContent,
    ) -> impl Future<Output = UpdateMessageResponse<Self>>;

    fn batch_delete_messages<CID: Into<ChatId>, I: IntoIterator<Item = MessageId>>(
        &self,
        chat_id: CID,
        message_ids: I,
        mode: CIDeleteMode,
    ) -> impl Future<Output = DeleteMessageResponse<Self>>;

    fn delete_message<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        mode: CIDeleteMode,
    ) -> impl Future<Output = DeleteMessageResponse<Self>> {
        self.batch_delete_messages(chat_id, std::iter::once(message_id.into()), mode)
    }

    fn batch_message_reactions<
        CID: Into<ChatId>,
        MID: Into<MessageId>,
        I: IntoIterator<Item = Reaction>,
    >(
        &self,
        chat_id: CID,
        message_id: MID,
        reactions: I,
    ) -> impl Future<Output = UpdateMessageReactionsResponse<Self>>;

    fn update_message_reaction<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        reaction: Reaction,
    ) -> impl Future<Output = UpdateMessageReactionsResponse<Self>> {
        self.batch_message_reactions(chat_id, message_id, std::iter::once(reaction))
    }

    fn accept_file<FID: Into<FileId>>(&self, file_id: FID) -> AcceptFileBuilder<'_, Self>;

    fn reject_file<FID: Into<FileId>>(
        &self,
        file_id: FID,
    ) -> impl Future<Output = RejectFileResponse<Self>>;

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

    async fn contacts<UID: Into<UserId>>(&self, user_id: UID) -> ContactsResponse<Self> {
        let mut response = self.api_list_contacts(user_id.into().0).await?;
        let response = Arc::get_mut(&mut response).unwrap();

        Ok(std::mem::take(&mut response.contacts))
    }

    async fn groups<UID: Into<UserId>>(&self, user_id: UID) -> GroupsResponse<Self> {
        let mut response = self
            .api_list_groups(ApiListGroups::new(user_id.into().0))
            .await?;
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

    fn accept_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> impl Future<Output = AcceptContactResponse<Self>> {
        self.api_accept_contact(contact_request_id.into().0)
    }

    fn reject_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> impl Future<Output = RejectContactResponse<Self>> {
        self.api_reject_contact(contact_request_id.into().0)
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

    fn update_message<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        new_content: MsgContent,
    ) -> impl Future<Output = UpdateMessageResponse<Self>> {
        self.api_update_chat_item(ApiUpdateChatItem {
            chat_ref: chat_id.into().into_chat_ref(),
            chat_item_id: message_id.into().0,
            live_message: false,
            updated_message: UpdatedMessage {
                msg_content: new_content,
                mentions: Default::default(),
                undocumented: Default::default(),
            },
        })
    }

    fn batch_delete_messages<CID: Into<ChatId>, I: IntoIterator<Item = MessageId>>(
        &self,
        chat_id: CID,
        message_ids: I,
        mode: CIDeleteMode,
    ) -> impl Future<Output = DeleteMessageResponse<Self>> {
        self.api_delete_chat_item(
            chat_id.into().into_chat_ref(),
            message_ids.into_iter().map(|id| id.0).collect(),
            mode,
        )
    }

    fn batch_message_reactions<
        CID: Into<ChatId>,
        MID: Into<MessageId>,
        I: IntoIterator<Item = Reaction>,
    >(
        &self,
        chat_id: CID,
        message_id: MID,
        reactions: I,
    ) -> impl Future<Output = UpdateMessageReactionsResponse<Self>> {
        let chat_id = chat_id.into();
        let message_id = message_id.into();

        futures::future::join_all(reactions.into_iter().map(|r| {
            let (add, emoji) = match r {
                Reaction::Set(e) => (true, e),
                Reaction::Unset(e) => (false, e),
            };

            self.api_chat_item_reaction(ApiChatItemReaction {
                chat_ref: chat_id.into_chat_ref(),
                chat_item_id: message_id.0,
                add,
                reaction: MsgReaction::Emoji {
                    emoji,
                    undocumented: Default::default(),
                },
            })
        }))
    }

    fn accept_file<FID: Into<FileId>>(&self, file_id: FID) -> AcceptFileBuilder<'_, Self> {
        AcceptFileBuilder {
            client: self,
            cmd: ReceiveFile::new(file_id.into().0),
        }
    }

    fn reject_file<FID: Into<FileId>>(
        &self,
        file_id: FID,
    ) -> impl Future<Output = RejectFileResponse<Self>> {
        self.cancel_file(file_id.into().0)
    }

    fn initiate_connection(
        &self,
        link: impl Into<String>,
    ) -> impl Future<Output = InitiateConnectionResponse<Self>> {
        self.connect(Connect {
            incognito: false,
            conn_link: Some(link.into()),
        })
        .map(|res| res.allow_undocumented())
    }

    fn delete_chat<CID: Into<ChatId>>(
        &self,
        chat_id: CID,
        mode: DeleteMode,
    ) -> impl Future<Output = DeleteChatResponse<Self>> {
        self.api_delete_chat(chat_id.into().into_chat_ref(), mode.into())
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

pub struct AcceptFileBuilder<'a, C: 'a + ?Sized> {
    client: &'a C,
    cmd: ReceiveFile,
}

impl<'a, C: 'a + ?Sized> AcceptFileBuilder<'a, C> {
    pub fn via_user_approved_relays(mut self) -> Self {
        self.cmd.user_approved_relays = true;
        self
    }

    pub fn store_encrypted(mut self) -> Self {
        self.cmd.store_encrypted = Some(true);
        self
    }

    pub fn inline(mut self) -> Self {
        self.cmd.file_inline = Some(true);
        self
    }

    pub fn file_path<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.cmd.file_path = Some(path.as_ref().display().to_string());
        self
    }
}

impl<'a, C: 'a + ?Sized + ClientApi> IntoFuture for AcceptFileBuilder<'a, C> {
    type Output = Result<ReceiveFileResponse, C::Error>;
    type IntoFuture = Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.client.receive_file(self.cmd))
    }
}

#[derive(Debug, Clone)]
pub enum Reaction {
    Set(String),
    Unset(String),
}
