use super::{errors::*, *};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiCreateMyAddressResponse {
    /// UserContactLinkCreated: User contact address created.
    #[serde(rename = "userContactLinkCreated")]
    UserContactLinkCreated(Arc<UserContactLinkCreatedResponse>),
}

impl ApiCreateMyAddressResponse {
    pub fn into_inner(self) -> Arc<UserContactLinkCreatedResponse> {
        match self {
            Self::UserContactLinkCreated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteMyAddressResponse {
    /// UserContactLinkDeleted: User contact address deleted.
    #[serde(rename = "userContactLinkDeleted")]
    UserContactLinkDeleted(Arc<UserContactLinkDeletedResponse>),
}

impl ApiDeleteMyAddressResponse {
    pub fn into_inner(self) -> Arc<UserContactLinkDeletedResponse> {
        match self {
            Self::UserContactLinkDeleted(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiShowMyAddressResponse {
    /// UserContactLink: User contact address.
    #[serde(rename = "userContactLink")]
    UserContactLink(Arc<UserContactLinkResponse>),
}

impl ApiShowMyAddressResponse {
    pub fn into_inner(self) -> Arc<UserContactLinkResponse> {
        match self {
            Self::UserContactLink(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetProfileAddressResponse {
    /// UserProfileUpdated: User profile updated.
    #[serde(rename = "userProfileUpdated")]
    UserProfileUpdated(Arc<UserProfileUpdatedResponse>),
}

impl ApiSetProfileAddressResponse {
    pub fn into_inner(self) -> Arc<UserProfileUpdatedResponse> {
        match self {
            Self::UserProfileUpdated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetAddressSettingsResponse {
    /// UserContactLinkUpdated: User contact address updated.
    #[serde(rename = "userContactLinkUpdated")]
    UserContactLinkUpdated(Arc<UserContactLinkUpdatedResponse>),
}

impl ApiSetAddressSettingsResponse {
    pub fn into_inner(self) -> Arc<UserContactLinkUpdatedResponse> {
        match self {
            Self::UserContactLinkUpdated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSendMessagesResponse {
    /// NewChatItems: New messages.
    #[serde(rename = "newChatItems")]
    NewChatItems(Arc<NewChatItemsResponse>),
}

impl ApiSendMessagesResponse {
    pub fn into_inner(self) -> Arc<NewChatItemsResponse> {
        match self {
            Self::NewChatItems(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiUpdateChatItemResponse {
    /// ChatItemUpdated: Message updated.
    #[serde(rename = "chatItemUpdated")]
    ChatItemUpdated(Arc<ChatItemUpdatedResponse>),
    /// ChatItemNotChanged: Message not changed.
    #[serde(rename = "chatItemNotChanged")]
    ChatItemNotChanged(Arc<ChatItemNotChangedResponse>),
}

impl ApiUpdateChatItemResponse {
    pub fn chat_item_updated(&self) -> Option<&ChatItemUpdatedResponse> {
        if let Self::ChatItemUpdated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_item_not_changed(&self) -> Option<&ChatItemNotChangedResponse> {
        if let Self::ChatItemNotChanged(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteChatItemResponse {
    /// ChatItemsDeleted: Messages deleted.
    #[serde(rename = "chatItemsDeleted")]
    ChatItemsDeleted(Arc<ChatItemsDeletedResponse>),
}

impl ApiDeleteChatItemResponse {
    pub fn into_inner(self) -> Arc<ChatItemsDeletedResponse> {
        match self {
            Self::ChatItemsDeleted(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteMemberChatItemResponse {
    /// ChatItemsDeleted: Messages deleted.
    #[serde(rename = "chatItemsDeleted")]
    ChatItemsDeleted(Arc<ChatItemsDeletedResponse>),
}

impl ApiDeleteMemberChatItemResponse {
    pub fn into_inner(self) -> Arc<ChatItemsDeletedResponse> {
        match self {
            Self::ChatItemsDeleted(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiChatItemReactionResponse {
    /// ChatItemReaction: Message reaction.
    #[serde(rename = "chatItemReaction")]
    ChatItemReaction(Arc<ChatItemReactionResponse>),
}

impl ApiChatItemReactionResponse {
    pub fn into_inner(self) -> Arc<ChatItemReactionResponse> {
        match self {
            Self::ChatItemReaction(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReceiveFileResponse {
    /// RcvFileAccepted: File accepted to be received.
    #[serde(rename = "rcvFileAccepted")]
    RcvFileAccepted(Arc<RcvFileAcceptedResponse>),
    /// RcvFileAcceptedSndCancelled: File accepted, but no longer sent.
    #[serde(rename = "rcvFileAcceptedSndCancelled")]
    RcvFileAcceptedSndCancelled(Arc<RcvFileAcceptedSndCancelledResponse>),
}

impl ReceiveFileResponse {
    pub fn rcv_file_accepted(&self) -> Option<&RcvFileAcceptedResponse> {
        if let Self::RcvFileAccepted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn rcv_file_accepted_snd_cancelled(&self) -> Option<&RcvFileAcceptedSndCancelledResponse> {
        if let Self::RcvFileAcceptedSndCancelled(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CancelFileResponse {
    /// SndFileCancelled: Cancelled sending file.
    #[serde(rename = "sndFileCancelled")]
    SndFileCancelled(Arc<SndFileCancelledResponse>),
    /// RcvFileCancelled: Cancelled receiving file.
    #[serde(rename = "rcvFileCancelled")]
    RcvFileCancelled(Arc<RcvFileCancelledResponse>),
}

impl CancelFileResponse {
    pub fn snd_file_cancelled(&self) -> Option<&SndFileCancelledResponse> {
        if let Self::SndFileCancelled(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn rcv_file_cancelled(&self) -> Option<&RcvFileCancelledResponse> {
        if let Self::RcvFileCancelled(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiAddMemberResponse {
    /// SentGroupInvitation: Group invitation sent.
    #[serde(rename = "sentGroupInvitation")]
    SentGroupInvitation(Arc<SentGroupInvitationResponse>),
}

impl ApiAddMemberResponse {
    pub fn into_inner(self) -> Arc<SentGroupInvitationResponse> {
        match self {
            Self::SentGroupInvitation(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiJoinGroupResponse {
    /// UserAcceptedGroupSent: User accepted group invitation.
    #[serde(rename = "userAcceptedGroupSent")]
    UserAcceptedGroupSent(Arc<UserAcceptedGroupSentResponse>),
}

impl ApiJoinGroupResponse {
    pub fn into_inner(self) -> Arc<UserAcceptedGroupSentResponse> {
        match self {
            Self::UserAcceptedGroupSent(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiAcceptMemberResponse {
    /// MemberAccepted: Member accepted to group.
    #[serde(rename = "memberAccepted")]
    MemberAccepted(Arc<MemberAcceptedResponse>),
}

impl ApiAcceptMemberResponse {
    pub fn into_inner(self) -> Arc<MemberAcceptedResponse> {
        match self {
            Self::MemberAccepted(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiMembersRoleResponse {
    /// MembersRoleUser: Members role changed by user.
    #[serde(rename = "membersRoleUser")]
    MembersRoleUser(Arc<MembersRoleUserResponse>),
}

impl ApiMembersRoleResponse {
    pub fn into_inner(self) -> Arc<MembersRoleUserResponse> {
        match self {
            Self::MembersRoleUser(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiBlockMembersForAllResponse {
    /// MembersBlockedForAllUser: Members blocked for all by admin.
    #[serde(rename = "membersBlockedForAllUser")]
    MembersBlockedForAllUser(Arc<MembersBlockedForAllUserResponse>),
}

impl ApiBlockMembersForAllResponse {
    pub fn into_inner(self) -> Arc<MembersBlockedForAllUserResponse> {
        match self {
            Self::MembersBlockedForAllUser(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiRemoveMembersResponse {
    /// UserDeletedMembers: Members deleted.
    #[serde(rename = "userDeletedMembers")]
    UserDeletedMembers(Arc<UserDeletedMembersResponse>),
}

impl ApiRemoveMembersResponse {
    pub fn into_inner(self) -> Arc<UserDeletedMembersResponse> {
        match self {
            Self::UserDeletedMembers(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiLeaveGroupResponse {
    /// LeftMemberUser: User left group.
    #[serde(rename = "leftMemberUser")]
    LeftMemberUser(Arc<LeftMemberUserResponse>),
}

impl ApiLeaveGroupResponse {
    pub fn into_inner(self) -> Arc<LeftMemberUserResponse> {
        match self {
            Self::LeftMemberUser(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiListMembersResponse {
    /// GroupMembers: Group members.
    #[serde(rename = "groupMembers")]
    GroupMembers(Arc<GroupMembersResponse>),
}

impl ApiListMembersResponse {
    pub fn into_inner(self) -> Arc<GroupMembersResponse> {
        match self {
            Self::GroupMembers(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiNewGroupResponse {
    /// GroupCreated: Group created.
    #[serde(rename = "groupCreated")]
    GroupCreated(Arc<GroupCreatedResponse>),
}

impl ApiNewGroupResponse {
    pub fn into_inner(self) -> Arc<GroupCreatedResponse> {
        match self {
            Self::GroupCreated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiUpdateGroupProfileResponse {
    /// GroupUpdated: Group updated.
    #[serde(rename = "groupUpdated")]
    GroupUpdated(Arc<GroupUpdatedResponse>),
}

impl ApiUpdateGroupProfileResponse {
    pub fn into_inner(self) -> Arc<GroupUpdatedResponse> {
        match self {
            Self::GroupUpdated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiCreateGroupLinkResponse {
    /// GroupLinkCreated: Group link created.
    #[serde(rename = "groupLinkCreated")]
    GroupLinkCreated(Arc<GroupLinkCreatedResponse>),
}

impl ApiCreateGroupLinkResponse {
    pub fn into_inner(self) -> Arc<GroupLinkCreatedResponse> {
        match self {
            Self::GroupLinkCreated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiGroupLinkMemberRoleResponse {
    /// GroupLink: Group link.
    #[serde(rename = "groupLink")]
    GroupLink(Arc<GroupLinkResponse>),
}

impl ApiGroupLinkMemberRoleResponse {
    pub fn into_inner(self) -> Arc<GroupLinkResponse> {
        match self {
            Self::GroupLink(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteGroupLinkResponse {
    /// GroupLinkDeleted: Group link deleted.
    #[serde(rename = "groupLinkDeleted")]
    GroupLinkDeleted(Arc<GroupLinkDeletedResponse>),
}

impl ApiDeleteGroupLinkResponse {
    pub fn into_inner(self) -> Arc<GroupLinkDeletedResponse> {
        match self {
            Self::GroupLinkDeleted(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiGetGroupLinkResponse {
    /// GroupLink: Group link.
    #[serde(rename = "groupLink")]
    GroupLink(Arc<GroupLinkResponse>),
}

impl ApiGetGroupLinkResponse {
    pub fn into_inner(self) -> Arc<GroupLinkResponse> {
        match self {
            Self::GroupLink(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiAddContactResponse {
    /// Invitation: One-time invitation.
    #[serde(rename = "invitation")]
    Invitation(Arc<InvitationResponse>),
}

impl ApiAddContactResponse {
    pub fn into_inner(self) -> Arc<InvitationResponse> {
        match self {
            Self::Invitation(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiConnectPlanResponse {
    /// ConnectionPlan: Connection link information.
    #[serde(rename = "connectionPlan")]
    ConnectionPlan(Arc<ConnectionPlanResponse>),
}

impl ApiConnectPlanResponse {
    pub fn into_inner(self) -> Arc<ConnectionPlanResponse> {
        match self {
            Self::ConnectionPlan(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiConnectResponse {
    /// SentConfirmation: Confirmation sent to one-time invitation.
    #[serde(rename = "sentConfirmation")]
    SentConfirmation(Arc<SentConfirmationResponse>),
    /// ContactAlreadyExists: Contact already exists.
    #[serde(rename = "contactAlreadyExists")]
    ContactAlreadyExists(Arc<ContactAlreadyExistsResponse>),
    /// SentInvitation: Invitation sent to contact address.
    #[serde(rename = "sentInvitation")]
    SentInvitation(Arc<SentInvitationResponse>),
}

impl ApiConnectResponse {
    pub fn sent_confirmation(&self) -> Option<&SentConfirmationResponse> {
        if let Self::SentConfirmation(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn contact_already_exists(&self) -> Option<&ContactAlreadyExistsResponse> {
        if let Self::ContactAlreadyExists(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn sent_invitation(&self) -> Option<&SentInvitationResponse> {
        if let Self::SentInvitation(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectResponse {
    /// SentConfirmation: Confirmation sent to one-time invitation.
    #[serde(rename = "sentConfirmation")]
    SentConfirmation(Arc<SentConfirmationResponse>),
    /// ContactAlreadyExists: Contact already exists.
    #[serde(rename = "contactAlreadyExists")]
    ContactAlreadyExists(Arc<ContactAlreadyExistsResponse>),
    /// SentInvitation: Invitation sent to contact address.
    #[serde(rename = "sentInvitation")]
    SentInvitation(Arc<SentInvitationResponse>),
}

impl ConnectResponse {
    pub fn sent_confirmation(&self) -> Option<&SentConfirmationResponse> {
        if let Self::SentConfirmation(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn contact_already_exists(&self) -> Option<&ContactAlreadyExistsResponse> {
        if let Self::ContactAlreadyExists(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn sent_invitation(&self) -> Option<&SentInvitationResponse> {
        if let Self::SentInvitation(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiAcceptContactResponse {
    /// AcceptingContactRequest: Contact request accepted.
    #[serde(rename = "acceptingContactRequest")]
    AcceptingContactRequest(Arc<AcceptingContactRequestResponse>),
}

impl ApiAcceptContactResponse {
    pub fn into_inner(self) -> Arc<AcceptingContactRequestResponse> {
        match self {
            Self::AcceptingContactRequest(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiRejectContactResponse {
    /// ContactRequestRejected: Contact request rejected.
    #[serde(rename = "contactRequestRejected")]
    ContactRequestRejected(Arc<ContactRequestRejectedResponse>),
}

impl ApiRejectContactResponse {
    pub fn into_inner(self) -> Arc<ContactRequestRejectedResponse> {
        match self {
            Self::ContactRequestRejected(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiListContactsResponse {
    /// ContactsList: Contacts.
    #[serde(rename = "contactsList")]
    ContactsList(Arc<ContactsListResponse>),
}

impl ApiListContactsResponse {
    pub fn into_inner(self) -> Arc<ContactsListResponse> {
        match self {
            Self::ContactsList(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiListGroupsResponse {
    /// GroupsList: Groups.
    #[serde(rename = "groupsList")]
    GroupsList(Arc<GroupsListResponse>),
}

impl ApiListGroupsResponse {
    pub fn into_inner(self) -> Arc<GroupsListResponse> {
        match self {
            Self::GroupsList(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteChatResponse {
    /// ContactDeleted: Contact deleted.
    #[serde(rename = "contactDeleted")]
    ContactDeleted(Arc<ContactDeletedResponse>),
    /// ContactConnectionDeleted: Connection deleted.
    #[serde(rename = "contactConnectionDeleted")]
    ContactConnectionDeleted(Arc<ContactConnectionDeletedResponse>),
    /// GroupDeletedUser: User deleted group.
    #[serde(rename = "groupDeletedUser")]
    GroupDeletedUser(Arc<GroupDeletedUserResponse>),
}

impl ApiDeleteChatResponse {
    pub fn contact_deleted(&self) -> Option<&ContactDeletedResponse> {
        if let Self::ContactDeleted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn contact_connection_deleted(&self) -> Option<&ContactConnectionDeletedResponse> {
        if let Self::ContactConnectionDeleted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn group_deleted_user(&self) -> Option<&GroupDeletedUserResponse> {
        if let Self::GroupDeletedUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetGroupCustomDataResponse {
    /// CmdOk: Ok.
    #[serde(rename = "cmdOk")]
    CmdOk(Arc<CmdOkResponse>),
}

impl ApiSetGroupCustomDataResponse {
    pub fn into_inner(self) -> Arc<CmdOkResponse> {
        match self {
            Self::CmdOk(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetContactCustomDataResponse {
    /// CmdOk: Ok.
    #[serde(rename = "cmdOk")]
    CmdOk(Arc<CmdOkResponse>),
}

impl ApiSetContactCustomDataResponse {
    pub fn into_inner(self) -> Arc<CmdOkResponse> {
        match self {
            Self::CmdOk(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetUserAutoAcceptMemberContactsResponse {
    /// CmdOk: Ok.
    #[serde(rename = "cmdOk")]
    CmdOk(Arc<CmdOkResponse>),
}

impl ApiSetUserAutoAcceptMemberContactsResponse {
    pub fn into_inner(self) -> Arc<CmdOkResponse> {
        match self {
            Self::CmdOk(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShowActiveUserResponse {
    /// ActiveUser: Active user profile.
    #[serde(rename = "activeUser")]
    ActiveUser(Arc<ActiveUserResponse>),
}

impl ShowActiveUserResponse {
    pub fn into_inner(self) -> Arc<ActiveUserResponse> {
        match self {
            Self::ActiveUser(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CreateActiveUserResponse {
    /// ActiveUser: Active user profile.
    #[serde(rename = "activeUser")]
    ActiveUser(Arc<ActiveUserResponse>),
}

impl CreateActiveUserResponse {
    pub fn into_inner(self) -> Arc<ActiveUserResponse> {
        match self {
            Self::ActiveUser(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ListUsersResponse {
    /// UsersList: Users.
    #[serde(rename = "usersList")]
    UsersList(Arc<UsersListResponse>),
}

impl ListUsersResponse {
    pub fn into_inner(self) -> Arc<UsersListResponse> {
        match self {
            Self::UsersList(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetActiveUserResponse {
    /// ActiveUser: Active user profile.
    #[serde(rename = "activeUser")]
    ActiveUser(Arc<ActiveUserResponse>),
}

impl ApiSetActiveUserResponse {
    pub fn into_inner(self) -> Arc<ActiveUserResponse> {
        match self {
            Self::ActiveUser(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteUserResponse {
    /// CmdOk: Ok.
    #[serde(rename = "cmdOk")]
    CmdOk(Arc<CmdOkResponse>),
}

impl ApiDeleteUserResponse {
    pub fn into_inner(self) -> Arc<CmdOkResponse> {
        match self {
            Self::CmdOk(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiUpdateProfileResponse {
    /// UserProfileUpdated: User profile updated.
    #[serde(rename = "userProfileUpdated")]
    UserProfileUpdated(Arc<UserProfileUpdatedResponse>),
    /// UserProfileNoChange: User profile was not changed.
    #[serde(rename = "userProfileNoChange")]
    UserProfileNoChange(Arc<UserProfileNoChangeResponse>),
}

impl ApiUpdateProfileResponse {
    pub fn user_profile_updated(&self) -> Option<&UserProfileUpdatedResponse> {
        if let Self::UserProfileUpdated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn user_profile_no_change(&self) -> Option<&UserProfileNoChangeResponse> {
        if let Self::UserProfileNoChange(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiSetContactPrefsResponse {
    /// ContactPrefsUpdated: Contact preferences updated.
    #[serde(rename = "contactPrefsUpdated")]
    ContactPrefsUpdated(Arc<ContactPrefsUpdatedResponse>),
}

impl ApiSetContactPrefsResponse {
    pub fn into_inner(self) -> Arc<ContactPrefsUpdatedResponse> {
        match self {
            Self::ContactPrefsUpdated(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StartChatResponse {
    /// ChatStarted: Chat started.
    #[serde(rename = "chatStarted")]
    ChatStarted(Arc<ChatStartedResponse>),
    /// ChatRunning: Chat running.
    #[serde(rename = "chatRunning")]
    ChatRunning(Arc<ChatRunningResponse>),
}

impl StartChatResponse {
    pub fn chat_started(&self) -> Option<&ChatStartedResponse> {
        if let Self::ChatStarted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_running(&self) -> Option<&ChatRunningResponse> {
        if let Self::ChatRunning(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiStopChatResponse {
    /// ChatStopped: Chat stopped.
    #[serde(rename = "chatStopped")]
    ChatStopped(Arc<ChatStoppedResponse>),
}

impl ApiStopChatResponse {
    pub fn into_inner(self) -> Arc<ChatStoppedResponse> {
        match self {
            Self::ChatStopped(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChatCmdError {
    #[serde(rename = "chatCmdError")]
    ChatCmdErrorResponse(Arc<ChatCmdErrorResponse>),
}

impl ChatCmdError {
    pub fn into_inner(self) -> Arc<ChatCmdErrorResponse> {
        match self {
            Self::ChatCmdErrorResponse(inner) => inner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatCmdErrorResponse {
    #[serde(rename = "chatError")]
    pub chat_error: Arc<ChatError>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AcceptingContactRequestResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ActiveUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemNotChangedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemReactionResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "added", default)]
    pub added: bool,

    #[serde(rename = "reaction")]
    pub reaction: ACIReaction,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemsDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItemDeletions")]
    pub chat_item_deletions: Vec<ChatItemDeletion>,

    #[serde(rename = "byUser", default)]
    pub by_user: bool,

    #[serde(rename = "timed", default)]
    pub timed: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatRunningResponse {
    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatStartedResponse {
    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatStoppedResponse {
    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CmdOkResponse {
    #[serde(rename = "user_", skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ConnectionPlanResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connLink")]
    pub conn_link: CreatedConnLink,

    #[serde(rename = "connectionPlan")]
    pub connection_plan: ConnectionPlan,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactAlreadyExistsResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactConnectionDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connection")]
    pub connection: PendingContactConnection,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactPrefsUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "fromContact")]
    pub from_contact: Contact,

    #[serde(rename = "toContact")]
    pub to_contact: Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactRequestRejectedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactRequest")]
    pub contact_request: UserContactRequest,

    #[serde(rename = "contact_", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactsListResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contacts")]
    pub contacts: Vec<Contact>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupDeletedUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "groupLink")]
    pub group_link: GroupLink,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "groupLink")]
    pub group_link: GroupLink,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMembersResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "group")]
    pub group: Group,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "fromGroup")]
    pub from_group: GroupInfo,

    #[serde(rename = "toGroup")]
    pub to_group: GroupInfo,

    #[serde(rename = "member_", skip_serializing_if = "Option::is_none")]
    pub member: Option<GroupMember>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupsListResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groups")]
    pub groups: Vec<GroupInfo>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct InvitationResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connLinkInvitation")]
    pub conn_link_invitation: CreatedConnLink,

    #[serde(rename = "connection")]
    pub connection: PendingContactConnection,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct LeftMemberUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MemberAcceptedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MembersBlockedForAllUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "members")]
    pub members: Vec<GroupMember>,

    #[serde(rename = "blocked", default)]
    pub blocked: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MembersRoleUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "members")]
    pub members: Vec<GroupMember>,

    #[serde(rename = "toRole")]
    pub to_role: GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewChatItemsResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<AChatItem>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAcceptedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAcceptedSndCancelledResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileCancelledResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<AChatItem>,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SentConfirmationResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connection")]
    pub connection: PendingContactConnection,

    #[serde(rename = "customUserProfile", skip_serializing_if = "Option::is_none")]
    pub custom_user_profile: Option<Profile>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SentGroupInvitationResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SentInvitationResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connection")]
    pub connection: PendingContactConnection,

    #[serde(rename = "customUserProfile", skip_serializing_if = "Option::is_none")]
    pub custom_user_profile: Option<Profile>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SndFileCancelledResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<AChatItem>,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: FileTransferMeta,

    #[serde(rename = "sndFileTransfers")]
    pub snd_file_transfers: Vec<SndFileTransfer>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserAcceptedGroupSentResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "hostContact", skip_serializing_if = "Option::is_none")]
    pub host_contact: Option<Contact>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connLinkContact")]
    pub conn_link_contact: CreatedConnLink,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactLink")]
    pub contact_link: UserContactLink,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactLink")]
    pub contact_link: UserContactLink,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserDeletedMembersResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "members")]
    pub members: Vec<GroupMember>,

    #[serde(rename = "withMessages", default)]
    pub with_messages: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserProfileNoChangeResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserProfileUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "fromProfile")]
    pub from_profile: Profile,

    #[serde(rename = "toProfile")]
    pub to_profile: Profile,

    #[serde(rename = "updateSummary")]
    pub update_summary: UserProfileUpdateSummary,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UsersListResponse {
    #[serde(rename = "users")]
    pub users: Vec<UserInfo>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}
