use simploxide_api_types::{
    AChatItem, ChatItem, MsgContent, NewUser,
    client_api::{ClientApi, ClientApiError as _},
    responses::ActiveUserResponse,
};

use std::sync::Arc;

use crate::id::ChatId;

pub type NewUserResponse<C> = Result<Arc<ActiveUserResponse>, <C as ClientApi>::Error>;

pub trait ClientApiExt: ClientApi {
    /// Like [ClientApi::create_active_user] but ensures that user is created even if the name
    /// contains invalid UTF-8 characters. The user struct gets cloned when performing the original
    /// request
    fn new_user(&self, user: NewUser) -> impl Future<Output = NewUserResponse<Self>>;
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
