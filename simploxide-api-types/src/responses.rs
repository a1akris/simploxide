use super::{errors::*, *};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiCreateMyAddressResponse {
    /// UserContactLinkCreated: User contact address created.
    #[serde(rename = "userContactLinkCreated")]
    UserContactLinkCreated(UserContactLinkCreatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiDeleteMyAddressResponse {
    /// UserContactLinkDeleted: User contact address deleted.
    #[serde(rename = "userContactLinkDeleted")]
    UserContactLinkDeleted(UserContactLinkDeletedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiShowMyAddressResponse {
    /// UserContactLink: User contact address.
    #[serde(rename = "userContactLink")]
    UserContactLink(UserContactLinkResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiSetProfileAddressResponse {
    /// UserProfileUpdated: User profile updated.
    #[serde(rename = "userProfileUpdated")]
    UserProfileUpdated(UserProfileUpdatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiSetAddressSettingsResponse {
    /// UserContactLinkUpdated: User contact address updated.
    #[serde(rename = "userContactLinkUpdated")]
    UserContactLinkUpdated(UserContactLinkUpdatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiSendMessagesResponse {
    /// NewChatItems: New messages.
    #[serde(rename = "newChatItems")]
    NewChatItems(NewChatItemsResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiUpdateChatItemResponse {
    /// ChatItemUpdated: Message updated.
    #[serde(rename = "chatItemUpdated")]
    ChatItemUpdated(ChatItemUpdatedResponse),
    /// ChatItemNotChanged: Message not changed.
    #[serde(rename = "chatItemNotChanged")]
    ChatItemNotChanged(ChatItemNotChangedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiDeleteChatItemResponse {
    /// ChatItemsDeleted: Messages deleted.
    #[serde(rename = "chatItemsDeleted")]
    ChatItemsDeleted(ChatItemsDeletedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiDeleteMemberChatItemResponse {
    /// ChatItemsDeleted: Messages deleted.
    #[serde(rename = "chatItemsDeleted")]
    ChatItemsDeleted(ChatItemsDeletedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiChatItemReactionResponse {
    /// ChatItemReaction: Message reaction.
    #[serde(rename = "chatItemReaction")]
    ChatItemReaction(ChatItemReactionResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ReceiveFileResponse {
    /// RcvFileAccepted: File accepted to be received.
    #[serde(rename = "rcvFileAccepted")]
    RcvFileAccepted(RcvFileAcceptedResponse),
    /// RcvFileAcceptedSndCancelled: File accepted, but no longer sent.
    #[serde(rename = "rcvFileAcceptedSndCancelled")]
    RcvFileAcceptedSndCancelled(RcvFileAcceptedSndCancelledResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CancelFileResponse {
    /// SndFileCancelled: Cancelled sending file.
    #[serde(rename = "sndFileCancelled")]
    SndFileCancelled(SndFileCancelledResponse),
    /// RcvFileCancelled: Cancelled receiving file.
    #[serde(rename = "rcvFileCancelled")]
    RcvFileCancelled(RcvFileCancelledResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiAddMemberResponse {
    /// SentGroupInvitation: Group invitation sent.
    #[serde(rename = "sentGroupInvitation")]
    SentGroupInvitation(SentGroupInvitationResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiJoinGroupResponse {
    /// UserAcceptedGroupSent: User accepted group invitation.
    #[serde(rename = "userAcceptedGroupSent")]
    UserAcceptedGroupSent(UserAcceptedGroupSentResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiAcceptMemberResponse {
    /// MemberAccepted: Member accepted to group.
    #[serde(rename = "memberAccepted")]
    MemberAccepted(MemberAcceptedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiMembersRoleResponse {
    /// MembersRoleUser: Members role changed by user.
    #[serde(rename = "membersRoleUser")]
    MembersRoleUser(MembersRoleUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiBlockMembersForAllResponse {
    /// MembersBlockedForAllUser: Members blocked for all by admin.
    #[serde(rename = "membersBlockedForAllUser")]
    MembersBlockedForAllUser(MembersBlockedForAllUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiRemoveMembersResponse {
    /// UserDeletedMembers: Members deleted.
    #[serde(rename = "userDeletedMembers")]
    UserDeletedMembers(UserDeletedMembersResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiLeaveGroupResponse {
    /// LeftMemberUser: User left group.
    #[serde(rename = "leftMemberUser")]
    LeftMemberUser(LeftMemberUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiListMembersResponse {
    /// GroupMembers: Group members.
    #[serde(rename = "groupMembers")]
    GroupMembers(GroupMembersResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiNewGroupResponse {
    /// GroupCreated: Group created.
    #[serde(rename = "groupCreated")]
    GroupCreated(GroupCreatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiUpdateGroupProfileResponse {
    /// GroupUpdated: Group updated.
    #[serde(rename = "groupUpdated")]
    GroupUpdated(GroupUpdatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiCreateGroupLinkResponse {
    /// GroupLinkCreated: Group link created.
    #[serde(rename = "groupLinkCreated")]
    GroupLinkCreated(GroupLinkCreatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiGroupLinkMemberRoleResponse {
    /// GroupLink: Group link.
    #[serde(rename = "groupLink")]
    GroupLink(GroupLinkResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiDeleteGroupLinkResponse {
    /// GroupLinkDeleted: Group link deleted.
    #[serde(rename = "groupLinkDeleted")]
    GroupLinkDeleted(GroupLinkDeletedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiGetGroupLinkResponse {
    /// GroupLink: Group link.
    #[serde(rename = "groupLink")]
    GroupLink(GroupLinkResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiAddContactResponse {
    /// Invitation: One-time invitation.
    #[serde(rename = "invitation")]
    Invitation(InvitationResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiConnectPlanResponse {
    /// ConnectionPlan: Connection link information.
    #[serde(rename = "connectionPlan")]
    ConnectionPlan(ConnectionPlanResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiConnectResponse {
    /// SentConfirmation: Confirmation sent to one-time invitation.
    #[serde(rename = "sentConfirmation")]
    SentConfirmation(SentConfirmationResponse),
    /// ContactAlreadyExists: Contact already exists.
    #[serde(rename = "contactAlreadyExists")]
    ContactAlreadyExists(ContactAlreadyExistsResponse),
    /// SentInvitation: Invitation sent to contact address.
    #[serde(rename = "sentInvitation")]
    SentInvitation(SentInvitationResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnectResponse {
    /// SentConfirmation: Confirmation sent to one-time invitation.
    #[serde(rename = "sentConfirmation")]
    SentConfirmation(SentConfirmationResponse),
    /// ContactAlreadyExists: Contact already exists.
    #[serde(rename = "contactAlreadyExists")]
    ContactAlreadyExists(ContactAlreadyExistsResponse),
    /// SentInvitation: Invitation sent to contact address.
    #[serde(rename = "sentInvitation")]
    SentInvitation(SentInvitationResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiAcceptContactResponse {
    /// AcceptingContactRequest: Contact request accepted.
    #[serde(rename = "acceptingContactRequest")]
    AcceptingContactRequest(AcceptingContactRequestResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiRejectContactResponse {
    /// ContactRequestRejected: Contact request rejected.
    #[serde(rename = "contactRequestRejected")]
    ContactRequestRejected(ContactRequestRejectedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiListContactsResponse {
    /// ContactsList: Contacts.
    #[serde(rename = "contactsList")]
    ContactsList(ContactsListResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiListGroupsResponse {
    /// GroupsList: Groups.
    #[serde(rename = "groupsList")]
    GroupsList(GroupsListResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiDeleteChatResponse {
    /// ContactDeleted: Contact deleted.
    #[serde(rename = "contactDeleted")]
    ContactDeleted(ContactDeletedResponse),
    /// ContactConnectionDeleted: Connection deleted.
    #[serde(rename = "contactConnectionDeleted")]
    ContactConnectionDeleted(ContactConnectionDeletedResponse),
    /// GroupDeletedUser: User deleted group.
    #[serde(rename = "groupDeletedUser")]
    GroupDeletedUser(GroupDeletedUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ShowActiveUserResponse {
    /// ActiveUser: Active user profile.
    #[serde(rename = "activeUser")]
    ActiveUser(ActiveUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CreateActiveUserResponse {
    /// ActiveUser: Active user profile.
    #[serde(rename = "activeUser")]
    ActiveUser(ActiveUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ListUsersResponse {
    /// UsersList: Users.
    #[serde(rename = "usersList")]
    UsersList(UsersListResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiSetActiveUserResponse {
    /// ActiveUser: Active user profile.
    #[serde(rename = "activeUser")]
    ActiveUser(ActiveUserResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiDeleteUserResponse {
    /// CmdOk: Ok.
    #[serde(rename = "cmdOk")]
    CmdOk(CmdOkResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiUpdateProfileResponse {
    /// UserProfileUpdated: User profile updated.
    #[serde(rename = "userProfileUpdated")]
    UserProfileUpdated(UserProfileUpdatedResponse),
    /// UserProfileNoChange: User profile was not changed.
    #[serde(rename = "userProfileNoChange")]
    UserProfileNoChange(UserProfileNoChangeResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiSetContactPrefsResponse {
    /// ContactPrefsUpdated: Contact preferences updated.
    #[serde(rename = "contactPrefsUpdated")]
    ContactPrefsUpdated(ContactPrefsUpdatedResponse),
    /// ChatCmdError: Command error.
    #[serde(rename = "chatCmdError")]
    ChatCmdError(ChatCmdErrorResponse),
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AcceptingContactRequestResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ActiveUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatCmdErrorResponse {
    #[serde(rename = "chatError")]
    pub chat_error: ChatError,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemNotChangedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemReactionResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "added")]
    pub added: bool,

    #[serde(rename = "reaction")]
    pub reaction: ACIReaction,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemsDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItemDeletions")]
    pub chat_item_deletions: Vec<ChatItemDeletion>,

    #[serde(rename = "byUser")]
    pub by_user: bool,

    #[serde(rename = "timed")]
    pub timed: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CmdOkResponse {
    #[serde(rename = "user_", skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactAlreadyExistsResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactConnectionDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connection")]
    pub connection: PendingContactConnection,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactsListResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contacts")]
    pub contacts: Vec<Contact>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupDeletedUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMembersResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "group")]
    pub group: Group,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupsListResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groups")]
    pub groups: Vec<GroupInfoSummary>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct LeftMemberUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(rename = "blocked")]
    pub blocked: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewChatItemsResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<AChatItem>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAcceptedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAcceptedSndCancelledResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connLinkContact")]
    pub conn_link_contact: CreatedConnLink,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactLink")]
    pub contact_link: UserContactLink,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactLink")]
    pub contact_link: UserContactLink,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(rename = "withMessages")]
    pub with_messages: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserProfileNoChangeResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UsersListResponse {
    #[serde(rename = "users")]
    pub users: Vec<UserInfo>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}
