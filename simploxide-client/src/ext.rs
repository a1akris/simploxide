use futures::FutureExt as _;
use simploxide_api_types::{
    AChatItem, CIDeleteMode, ChatDeleteMode, ChatItem, Contact, GroupInfo, GroupMember,
    GroupMemberRole, GroupProfile, JsonObject, MsgContent, MsgReaction, NewUser, UpdatedMessage,
    UserInfo,
    client_api::{
        AllowUndocumentedResponses as _, ClientApi, ClientApiError as _, UndocumentedResponse,
    },
    commands::{
        ApiBlockMembersForAll, ApiChatItemReaction, ApiListGroups, ApiRemoveMembers,
        ApiSetContactCustomData, ApiSetGroupCustomData, ApiUpdateChatItem, Connect, ReceiveFile,
    },
    responses::{
        AcceptingContactRequestResponse, ActiveUserResponse, ApiDeleteChatResponse,
        ApiUpdateChatItemResponse, CancelFileResponse, ChatItemReactionResponse,
        ChatItemsDeletedResponse, CmdOkResponse, ConnectResponse, ContactRequestRejectedResponse,
        GroupLinkCreatedResponse, GroupLinkDeletedResponse, GroupLinkResponse, GroupRelaysResponse,
        GroupUpdatedResponse,
        LeftMemberUserResponse, MemberAcceptedResponse, MembersBlockedForAllUserResponse,
        MembersRoleUserResponse, ReceiveFileResponse, SentGroupInvitationResponse,
        UserAcceptedGroupSentResponse, UserDeletedMembersResponse,
    },
};

use std::{pin::Pin, sync::Arc};

use crate::{
    id::{ChatId, ContactId, ContactRequestId, FileId, GroupId, MemberId, MessageId, UserId},
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

pub type AddMemberResponse<C> = Result<Arc<SentGroupInvitationResponse>, <C as ClientApi>::Error>;
pub type JoinGroupResponse<C> = Result<Arc<UserAcceptedGroupSentResponse>, <C as ClientApi>::Error>;
pub type AcceptMemberResponse<C> = Result<Arc<MemberAcceptedResponse>, <C as ClientApi>::Error>;
pub type SetMembersRoleResponse<C> = Result<Arc<MembersRoleUserResponse>, <C as ClientApi>::Error>;
pub type BlockMembersResponse<C> =
    Result<Arc<MembersBlockedForAllUserResponse>, <C as ClientApi>::Error>;
pub type RemoveMembersResponse<C> = Result<Arc<UserDeletedMembersResponse>, <C as ClientApi>::Error>;
pub type LeaveGroupResponse<C> = Result<Arc<LeftMemberUserResponse>, <C as ClientApi>::Error>;
pub type ListMembersResponse<C> = Result<Vec<GroupMember>, <C as ClientApi>::Error>;
pub type UpdateGroupProfileResponse<C> = Result<Arc<GroupUpdatedResponse>, <C as ClientApi>::Error>;
pub type SetContactCustomDataResponse<C> = Result<Arc<CmdOkResponse>, <C as ClientApi>::Error>;
pub type SetGroupCustomDataResponse<C> = Result<Arc<CmdOkResponse>, <C as ClientApi>::Error>;
pub type CreateGroupLinkResult<C> = Result<Arc<GroupLinkCreatedResponse>, <C as ClientApi>::Error>;
pub type GroupLinkResult<C> = Result<Arc<GroupLinkResponse>, <C as ClientApi>::Error>;
pub type DeleteGroupLinkResult<C> = Result<Arc<GroupLinkDeletedResponse>, <C as ClientApi>::Error>;
pub type GetGroupRelaysResponse<C> = Result<Arc<GroupRelaysResponse>, <C as ClientApi>::Error>;

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
    ) -> MessageBuilder<'_, Self, M::Kind>;

    /// Deliver the same message to multiple recepients
    fn multicast_message<I, M>(
        &self,
        chat_ids: I,
        msg: M,
    ) -> MulticastBuilder<'_, I, Self, M::Kind>
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

    fn add_member<GID: Into<GroupId>, CID: Into<ContactId>>(
        &self,
        group_id: GID,
        contact_id: CID,
        role: GroupMemberRole,
    ) -> impl Future<Output = AddMemberResponse<Self>>;

    fn join_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = JoinGroupResponse<Self>>;

    fn accept_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> impl Future<Output = AcceptMemberResponse<Self>>;

    fn set_members_role<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
        role: GroupMemberRole,
    ) -> impl Future<Output = SetMembersRoleResponse<Self>>;

    fn set_member_role<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> impl Future<Output = SetMembersRoleResponse<Self>> {
        self.set_members_role(group_id, std::iter::once(member_id.into()), role)
    }

    fn block_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = BlockMembersResponse<Self>>;

    fn unblock_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = BlockMembersResponse<Self>>;

    fn block_member_for_all<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = BlockMembersResponse<Self>> {
        self.block_members_for_all(group_id, std::iter::once(member_id.into()))
    }

    fn unblock_member_for_all<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = BlockMembersResponse<Self>> {
        self.unblock_members_for_all(group_id, std::iter::once(member_id.into()))
    }

    fn remove_members<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = RemoveMembersResponse<Self>>;

    fn remove_members_with_messages<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = RemoveMembersResponse<Self>>;

    fn remove_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = RemoveMembersResponse<Self>> {
        self.remove_members(group_id, std::iter::once(member_id.into()))
    }

    fn remove_member_with_messages<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = RemoveMembersResponse<Self>> {
        self.remove_members_with_messages(group_id, std::iter::once(member_id.into()))
    }

    fn leave_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = LeaveGroupResponse<Self>>;

    fn list_members<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = ListMembersResponse<Self>>;

    fn moderate_messages<GID: Into<GroupId>, I: IntoIterator<Item = MessageId>>(
        &self,
        group_id: GID,
        message_ids: I,
    ) -> impl Future<Output = DeleteMessageResponse<Self>>;

    fn moderate_message<GID: Into<GroupId>, MID: Into<MessageId>>(
        &self,
        group_id: GID,
        message_id: MID,
    ) -> impl Future<Output = DeleteMessageResponse<Self>> {
        self.moderate_messages(group_id, std::iter::once(message_id.into()))
    }

    fn update_group_profile<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        profile: GroupProfile,
    ) -> impl Future<Output = UpdateGroupProfileResponse<Self>>;

    fn set_group_custom_data<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        data: Option<JsonObject>,
    ) -> impl Future<Output = SetGroupCustomDataResponse<Self>>;

    fn set_contact_custom_data<CID: Into<ContactId>>(
        &self,
        contact_id: CID,
        data: Option<JsonObject>,
    ) -> impl Future<Output = SetContactCustomDataResponse<Self>>;

    fn create_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> impl Future<Output = CreateGroupLinkResult<Self>>;

    fn set_group_link_role<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> impl Future<Output = GroupLinkResult<Self>>;

    fn delete_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = DeleteGroupLinkResult<Self>>;

    fn get_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = GroupLinkResult<Self>>;

    fn get_group_relays<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = GetGroupRelaysResponse<Self>>;
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
    ) -> MessageBuilder<'_, Self, M::Kind> {
        let (composed, kind) = msg.into_builder_parts();
        MessageBuilder {
            client: self,
            chat_id: cid.into(),
            live: false,
            ttl: None,
            msg: composed,
            kind,
        }
    }

    fn multicast_message<I, M>(&self, chat_ids: I, msg: M) -> MulticastBuilder<'_, I, Self, M::Kind>
    where
        I: IntoIterator<Item = ChatId>,
        M: MessageLike,
    {
        let (msg, kind) = msg.into_builder_parts();
        MulticastBuilder {
            client: self,
            chat_ids,
            ttl: None,
            msg,
            kind,
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

    fn add_member<GID: Into<GroupId>, CID: Into<ContactId>>(
        &self,
        group_id: GID,
        contact_id: CID,
        role: GroupMemberRole,
    ) -> impl Future<Output = AddMemberResponse<Self>> {
        self.api_add_member(group_id.into().0, contact_id.into().0, role)
    }

    fn join_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = JoinGroupResponse<Self>> {
        self.api_join_group(group_id.into().0)
    }

    fn accept_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> impl Future<Output = AcceptMemberResponse<Self>> {
        self.api_accept_member(group_id.into().0, member_id.into().0, role)
    }

    fn set_members_role<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
        role: GroupMemberRole,
    ) -> impl Future<Output = SetMembersRoleResponse<Self>> {
        self.api_members_role(
            group_id.into().0,
            member_ids.into_iter().map(|id| id.0).collect(),
            role,
        )
    }

    fn block_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = BlockMembersResponse<Self>> {
        self.api_block_members_for_all(ApiBlockMembersForAll {
            group_id: group_id.into().0,
            group_member_ids: member_ids.into_iter().map(|id| id.0).collect(),
            blocked: true,
        })
    }

    fn unblock_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = BlockMembersResponse<Self>> {
        self.api_block_members_for_all(ApiBlockMembersForAll {
            group_id: group_id.into().0,
            group_member_ids: member_ids.into_iter().map(|id| id.0).collect(),
            blocked: false,
        })
    }

    fn remove_members<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = RemoveMembersResponse<Self>> {
        self.api_remove_members(ApiRemoveMembers {
            group_id: group_id.into().0,
            group_member_ids: member_ids.into_iter().map(|id| id.0).collect(),
            with_messages: false,
        })
    }

    fn remove_members_with_messages<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = RemoveMembersResponse<Self>> {
        self.api_remove_members(ApiRemoveMembers {
            group_id: group_id.into().0,
            group_member_ids: member_ids.into_iter().map(|id| id.0).collect(),
            with_messages: true,
        })
    }

    fn leave_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = LeaveGroupResponse<Self>> {
        self.api_leave_group(group_id.into().0)
    }

    async fn list_members<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> ListMembersResponse<Self> {
        let mut response = self.api_list_members(group_id.into().0).await?;
        let response = Arc::get_mut(&mut response).unwrap();
        Ok(std::mem::take(&mut response.group.members))
    }

    fn moderate_messages<GID: Into<GroupId>, I: IntoIterator<Item = MessageId>>(
        &self,
        group_id: GID,
        message_ids: I,
    ) -> impl Future<Output = DeleteMessageResponse<Self>> {
        self.api_delete_member_chat_item(
            group_id.into().0,
            message_ids.into_iter().map(|id| id.0).collect(),
        )
    }

    fn update_group_profile<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        profile: GroupProfile,
    ) -> impl Future<Output = UpdateGroupProfileResponse<Self>> {
        self.api_update_group_profile(group_id.into().0, profile)
    }

    fn set_group_custom_data<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        data: Option<JsonObject>,
    ) -> impl Future<Output = SetGroupCustomDataResponse<Self>> {
        self.api_set_group_custom_data(ApiSetGroupCustomData {
            group_id: group_id.into().0,
            custom_data: data,
        })
    }

    fn set_contact_custom_data<CID: Into<ContactId>>(
        &self,
        contact_id: CID,
        data: Option<JsonObject>,
    ) -> impl Future<Output = SetContactCustomDataResponse<Self>> {
        self.api_set_contact_custom_data(ApiSetContactCustomData {
            contact_id: contact_id.into().0,
            custom_data: data,
        })
    }

    fn create_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> impl Future<Output = CreateGroupLinkResult<Self>> {
        self.api_create_group_link(group_id.into().0, role)
    }

    fn set_group_link_role<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> impl Future<Output = GroupLinkResult<Self>> {
        self.api_group_link_member_role(group_id.into().0, role)
    }

    fn delete_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = DeleteGroupLinkResult<Self>> {
        self.api_delete_group_link(group_id.into().0)
    }

    fn get_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = GroupLinkResult<Self>> {
        self.api_get_group_link(group_id.into().0)
    }

    fn get_group_relays<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = GetGroupRelaysResponse<Self>> {
        self.api_get_group_relays(group_id.into().0)
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
