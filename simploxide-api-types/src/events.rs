use crate::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Event {
    /// Contact connection events
    #[serde(rename = "contactConnected")]
    ContactConnected(Arc<ContactConnected>),
    /// Contact connection events
    #[serde(rename = "contactUpdated")]
    ContactUpdated(Arc<ContactUpdated>),
    /// Contact connection events
    #[serde(rename = "contactDeletedByContact")]
    ContactDeletedByContact(Arc<ContactDeletedByContact>),
    /// Contact connection events
    #[serde(rename = "receivedContactRequest")]
    ReceivedContactRequest(Arc<ReceivedContactRequest>),
    /// Contact connection events
    #[serde(rename = "newMemberContactReceivedInv")]
    NewMemberContactReceivedInv(Arc<NewMemberContactReceivedInv>),
    /// Contact connection events
    #[serde(rename = "contactSndReady")]
    ContactSndReady(Arc<ContactSndReady>),
    /// Message events
    #[serde(rename = "newChatItems")]
    NewChatItems(Arc<NewChatItems>),
    /// Message events
    #[serde(rename = "chatItemReaction")]
    ChatItemReaction(Arc<ChatItemReaction>),
    /// Message events
    #[serde(rename = "chatItemsDeleted")]
    ChatItemsDeleted(Arc<ChatItemsDeleted>),
    /// Message events
    #[serde(rename = "chatItemUpdated")]
    ChatItemUpdated(Arc<ChatItemUpdated>),
    /// Message events
    #[serde(rename = "groupChatItemsDeleted")]
    GroupChatItemsDeleted(Arc<GroupChatItemsDeleted>),
    /// Message events
    #[serde(rename = "chatItemsStatusesUpdated")]
    ChatItemsStatusesUpdated(Arc<ChatItemsStatusesUpdated>),
    /// Group events
    #[serde(rename = "receivedGroupInvitation")]
    ReceivedGroupInvitation(Arc<ReceivedGroupInvitation>),
    /// Group events
    #[serde(rename = "userJoinedGroup")]
    UserJoinedGroup(Arc<UserJoinedGroup>),
    /// Group events
    #[serde(rename = "groupUpdated")]
    GroupUpdated(Arc<GroupUpdated>),
    /// Group events
    #[serde(rename = "joinedGroupMember")]
    JoinedGroupMember(Arc<JoinedGroupMember>),
    /// Group events
    #[serde(rename = "memberRole")]
    MemberRole(Arc<MemberRole>),
    /// Group events
    #[serde(rename = "deletedMember")]
    DeletedMember(Arc<DeletedMember>),
    /// Group events
    #[serde(rename = "leftMember")]
    LeftMember(Arc<LeftMember>),
    /// Group events
    #[serde(rename = "deletedMemberUser")]
    DeletedMemberUser(Arc<DeletedMemberUser>),
    /// Group events
    #[serde(rename = "groupDeleted")]
    GroupDeleted(Arc<GroupDeleted>),
    /// Group events
    #[serde(rename = "connectedToGroupMember")]
    ConnectedToGroupMember(Arc<ConnectedToGroupMember>),
    /// Group events
    #[serde(rename = "memberAcceptedByOther")]
    MemberAcceptedByOther(Arc<MemberAcceptedByOther>),
    /// Group events
    #[serde(rename = "memberBlockedForAll")]
    MemberBlockedForAll(Arc<MemberBlockedForAll>),
    /// Group events
    #[serde(rename = "groupMemberUpdated")]
    GroupMemberUpdated(Arc<GroupMemberUpdated>),
    /// Group events
    #[serde(rename = "groupLinkDataUpdated")]
    GroupLinkDataUpdated(Arc<GroupLinkDataUpdated>),
    /// Group events
    #[serde(rename = "groupRelayUpdated")]
    GroupRelayUpdated(Arc<GroupRelayUpdated>),
    /// File events
    #[serde(rename = "rcvFileDescrReady")]
    RcvFileDescrReady(Arc<RcvFileDescrReady>),
    /// File events
    #[serde(rename = "rcvFileComplete")]
    RcvFileComplete(Arc<RcvFileComplete>),
    /// File events
    #[serde(rename = "sndFileCompleteXFTP")]
    SndFileCompleteXftp(Arc<SndFileCompleteXftp>),
    /// File events
    #[serde(rename = "rcvFileStart")]
    RcvFileStart(Arc<RcvFileStart>),
    /// File events
    #[serde(rename = "rcvFileSndCancelled")]
    RcvFileSndCancelled(Arc<RcvFileSndCancelled>),
    /// File events
    #[serde(rename = "rcvFileAccepted")]
    RcvFileAccepted(Arc<RcvFileAccepted>),
    /// File events
    #[serde(rename = "rcvFileError")]
    RcvFileError(Arc<RcvFileError>),
    /// File events
    #[serde(rename = "rcvFileWarning")]
    RcvFileWarning(Arc<RcvFileWarning>),
    /// File events
    #[serde(rename = "sndFileError")]
    SndFileError(Arc<SndFileError>),
    /// File events
    #[serde(rename = "sndFileWarning")]
    SndFileWarning(Arc<SndFileWarning>),
    /// Connection progress events
    #[serde(rename = "acceptingContactRequest")]
    AcceptingContactRequest(Arc<AcceptingContactRequest>),
    /// Connection progress events
    #[serde(rename = "acceptingBusinessRequest")]
    AcceptingBusinessRequest(Arc<AcceptingBusinessRequest>),
    /// Connection progress events
    #[serde(rename = "contactConnecting")]
    ContactConnecting(Arc<ContactConnecting>),
    /// Connection progress events
    #[serde(rename = "businessLinkConnecting")]
    BusinessLinkConnecting(Arc<BusinessLinkConnecting>),
    /// Connection progress events
    #[serde(rename = "joinedGroupMemberConnecting")]
    JoinedGroupMemberConnecting(Arc<JoinedGroupMemberConnecting>),
    /// Connection progress events
    #[serde(rename = "sentGroupInvitation")]
    SentGroupInvitation(Arc<SentGroupInvitation>),
    /// Connection progress events
    #[serde(rename = "groupLinkConnecting")]
    GroupLinkConnecting(Arc<GroupLinkConnecting>),
    /// Network connection events
    #[serde(rename = "hostConnected")]
    HostConnected(Arc<HostConnected>),
    /// Network connection events
    #[serde(rename = "hostDisconnected")]
    HostDisconnected(Arc<HostDisconnected>),
    /// Network connection events
    #[serde(rename = "subscriptionStatus")]
    SubscriptionStatus(Arc<SubscriptionStatus>),
    /// Error events
    #[serde(rename = "messageError")]
    MessageError(Arc<MessageError>),
    /// Error events
    #[serde(rename = "chatError")]
    ChatError(Arc<ChatError>),
    /// Error events
    #[serde(rename = "chatErrors")]
    ChatErrors(Arc<ChatErrors>),
    #[serde(untagged)]
    Undocumented(JsonObject),
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [APIShowMyAddress](./COMMANDS.md#apishowmyaddress) to check if address exists,
/// - [APICreateMyAddress](./COMMANDS.md#apicreatemyaddress) to create address,
/// - [APISetAddressSettings](./COMMANDS.md#apisetaddresssettings) to enable auto-access.
///
/// ----
///
/// This event is sent after a user connects via bot SimpleX address (not a business address).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactConnected {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(rename = "userCustomProfile", skip_serializing_if = "Option::is_none")]
    pub user_custom_profile: Option<crate::Profile>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [APIShowMyAddress](./COMMANDS.md#apishowmyaddress) to check if address exists,
/// - [APICreateMyAddress](./COMMANDS.md#apicreatemyaddress) to create address,
/// - [APISetAddressSettings](./COMMANDS.md#apisetaddresssettings) to enable auto-access.
///
/// ----
///
/// Contact profile of another user is updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "fromContact")]
    pub from_contact: crate::Contact,

    #[serde(rename = "toContact")]
    pub to_contact: crate::Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [APIShowMyAddress](./COMMANDS.md#apishowmyaddress) to check if address exists,
/// - [APICreateMyAddress](./COMMANDS.md#apicreatemyaddress) to create address,
/// - [APISetAddressSettings](./COMMANDS.md#apisetaddresssettings) to enable auto-access.
///
/// ----
///
/// Bot user's connection with another contact is deleted (conversation is kept).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactDeletedByContact {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [APIShowMyAddress](./COMMANDS.md#apishowmyaddress) to check if address exists,
/// - [APICreateMyAddress](./COMMANDS.md#apicreatemyaddress) to create address,
/// - [APISetAddressSettings](./COMMANDS.md#apisetaddresssettings) to enable auto-access.
///
/// ----
///
/// Contact request received.
///
/// This event is only sent when auto-accept is disabled.
///
/// The request needs to be accepted using [APIAcceptContact](./COMMANDS.md#apiacceptcontact) command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ReceivedContactRequest {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contactRequest")]
    pub contact_request: crate::UserContactRequest,

    #[serde(rename = "chat_", skip_serializing_if = "Option::is_none")]
    pub chat: Option<crate::AChat>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [APIShowMyAddress](./COMMANDS.md#apishowmyaddress) to check if address exists,
/// - [APICreateMyAddress](./COMMANDS.md#apicreatemyaddress) to create address,
/// - [APISetAddressSettings](./COMMANDS.md#apisetaddresssettings) to enable auto-access.
///
/// ----
///
/// Received invitation to connect directly with a group member.
///
/// This event only needs to be processed to associate contact with group, the connection will proceed automatically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewMemberContactReceivedInv {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [APIShowMyAddress](./COMMANDS.md#apishowmyaddress) to check if address exists,
/// - [APICreateMyAddress](./COMMANDS.md#apicreatemyaddress) to create address,
/// - [APISetAddressSettings](./COMMANDS.md#apisetaddresssettings) to enable auto-access.
///
/// ----
///
/// Connecting via 1-time invitation or after accepting contact request.
///
/// After this event bot can send messages to this contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactSndReady {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Message events
///
/// Bots must use these events to process received messages.
///
/// ----
///
/// Received message(s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewChatItems {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<crate::AChatItem>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Message events
///
/// Bots must use these events to process received messages.
///
/// ----
///
/// Received message reaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemReaction {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "added", default)]
    pub added: bool,

    #[serde(rename = "reaction")]
    pub reaction: crate::ACIReaction,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Message events
///
/// Bots must use these events to process received messages.
///
/// ----
///
/// Message was deleted by another user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemsDeleted {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItemDeletions")]
    pub chat_item_deletions: Vec<crate::ChatItemDeletion>,

    #[serde(rename = "byUser", default)]
    pub by_user: bool,

    #[serde(rename = "timed", default)]
    pub timed: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Message events
///
/// Bots must use these events to process received messages.
///
/// ----
///
/// Message was updated by another user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Message events
///
/// Bots must use these events to process received messages.
///
/// ----
///
/// Group messages are deleted or moderated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupChatItemsDeleted {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "chatItemIDs")]
    pub chat_item_i_ds: Vec<i64>,

    #[serde(rename = "byUser", default)]
    pub by_user: bool,

    #[serde(rename = "member_", skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::GroupMember>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Message events
///
/// Bots must use these events to process received messages.
///
/// ----
///
/// Message delivery status updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemsStatusesUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<crate::AChatItem>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Received group invitation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ReceivedGroupInvitation {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(rename = "fromMemberRole")]
    pub from_member_role: crate::GroupMemberRole,

    #[serde(rename = "memberRole")]
    pub member_role: crate::GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Bot user joined group. Received when connection via group link completes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserJoinedGroup {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Group profile or preferences updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "fromGroup")]
    pub from_group: crate::GroupInfo,

    #[serde(rename = "toGroup")]
    pub to_group: crate::GroupInfo,

    #[serde(rename = "member_", skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::GroupMember>,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Another member joined group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct JoinedGroupMember {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Member (or bot user's) group role changed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MemberRole {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "byMember")]
    pub by_member: crate::GroupMember,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "fromRole")]
    pub from_role: crate::GroupMemberRole,

    #[serde(rename = "toRole")]
    pub to_role: crate::GroupMemberRole,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Another member is removed from the group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct DeletedMember {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "byMember")]
    pub by_member: crate::GroupMember,

    #[serde(rename = "deletedMember")]
    pub deleted_member: crate::GroupMember,

    #[serde(rename = "withMessages", default)]
    pub with_messages: bool,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Another member left the group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct LeftMember {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Bot user was removed from the group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct DeletedMemberUser {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "withMessages", default)]
    pub with_messages: bool,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Group was deleted by the owner (not bot user).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupDeleted {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Connected to another group member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ConnectedToGroupMember {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "memberContact", skip_serializing_if = "Option::is_none")]
    pub member_contact: Option<crate::Contact>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Another group owner, admin or moderator accepted member to the group after review ("knocking").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MemberAcceptedByOther {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "acceptingMember")]
    pub accepting_member: crate::GroupMember,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Another member blocked for all members.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MemberBlockedForAll {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "byMember")]
    pub by_member: crate::GroupMember,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "blocked", default)]
    pub blocked: bool,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<crate::MsgSigStatus>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Another group member profile updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMemberUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "fromMember")]
    pub from_member: crate::GroupMember,

    #[serde(rename = "toMember")]
    pub to_member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Group link data updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkDataUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "groupLink")]
    pub group_link: crate::GroupLink,

    #[serde(rename = "groupRelays")]
    pub group_relays: Vec<crate::GroupRelay>,

    #[serde(rename = "relaysChanged", default)]
    pub relays_changed: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Group events
///
/// Bots may use these events to manage users' groups and business address groups.
///
/// *Please note*: programming groups is more complex than programming direct connections
///
/// ----
///
/// Group relay member updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupRelayUpdated {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(rename = "groupRelay")]
    pub group_relay: crate::GroupRelay,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// File is ready to be received.
///
/// This event is useful for processing sender file servers and monitoring file reception progress.
///
/// [ReceiveFile](./COMMANDS.md#receivefile) command can be used before this event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileDescrReady {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: crate::RcvFileTransfer,

    #[serde(rename = "rcvFileDescr")]
    pub rcv_file_descr: crate::RcvFileDescr,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// File reception is competed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileComplete {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// File upload is competed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SndFileCompleteXftp {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: crate::FileTransferMeta,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// File reception started. This event will be sent after [CEvtRcvFileDescrReady](#rcvfiledescrready) event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileStart {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// File was cancelled by the sender. This event may be sent instead of [CEvtRcvFileDescrReady](#rcvfiledescrready) event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileSndCancelled {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: crate::RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// This event will be sent when file is automatically accepted because of CLI option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAccepted {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem")]
    pub chat_item: crate::AChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// Error receiving file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileError {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<crate::AChatItem>,

    #[serde(rename = "agentError")]
    pub agent_error: errors::AgentErrorType,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: crate::RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// Warning when receiving file. It can happen when CLI settings do not allow to connect to file server(s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileWarning {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<crate::AChatItem>,

    #[serde(rename = "agentError")]
    pub agent_error: errors::AgentErrorType,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: crate::RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// Error sending file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SndFileError {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<crate::AChatItem>,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: crate::FileTransferMeta,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile](./COMMANDS.md#receivefile) or [APIDeleteMemberChatItem](./COMMANDS.md#apideletememberchatitem)) when processing [NewChatItems](#newchatitems) event.
///
/// Bots that need to send files should use [APISendMessages](./COMMANDS.md#apisendmessages) command.
///
/// ----
///
/// Warning when sending file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SndFileWarning {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<crate::AChatItem>,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: crate::FileTransferMeta,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Automatically accepting contact request via bot's SimpleX address with auto-accept enabled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AcceptingContactRequest {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Automatically accepting contact request via bot's business address.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AcceptingBusinessRequest {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Contact confirmed connection.
///
/// Sent when contact started connecting via bot's 1-time invitation link or when bot connects to another SimpleX address.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactConnecting {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Contact confirmed connection.
///
/// Sent when bot connects to another business address.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct BusinessLinkConnecting {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: crate::GroupMember,

    #[serde(rename = "fromContact")]
    pub from_contact: crate::Contact,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Group member is announced to the group and will be connecting to bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct JoinedGroupMemberConnecting {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: crate::GroupMember,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Sent when another user joins group via bot's link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SentGroupInvitation {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "contact")]
    pub contact: crate::Contact,

    #[serde(rename = "member")]
    pub member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Connection progress events
///
/// Bots may use these events to track progress of connections for monitoring or debugging.
///
/// ----
///
/// Sent when bot joins group via another user link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkConnecting {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "groupInfo")]
    pub group_info: crate::GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: crate::GroupMember,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Network connection events
///
///
/// ----
///
/// Messaging or file server connected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct HostConnected {
    #[serde(rename = "protocol")]
    pub protocol: String,

    #[serde(rename = "transportHost")]
    pub transport_host: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Network connection events
///
///
/// ----
///
/// Messaging or file server disconnected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct HostDisconnected {
    #[serde(rename = "protocol")]
    pub protocol: String,

    #[serde(rename = "transportHost")]
    pub transport_host: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Network connection events
///
///
/// ----
///
/// Messaging subscription status changed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SubscriptionStatus {
    #[serde(rename = "server")]
    pub server: String,

    #[serde(rename = "subscriptionStatus")]
    pub subscription_status: crate::SubscriptionStatus,

    #[serde(rename = "connections")]
    pub connections: Vec<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Error events
///
/// Bots may log these events for debugging. There will be many error events - this does NOT indicate a malfunction - e.g., they may happen because of bad network connectivity, or because messages may be delivered to deleted chats for a short period of time (they will be ignored).
///
/// ----
///
/// Message error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct MessageError {
    #[serde(rename = "user")]
    pub user: crate::User,

    #[serde(rename = "severity")]
    pub severity: String,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Error events
///
/// Bots may log these events for debugging. There will be many error events - this does NOT indicate a malfunction - e.g., they may happen because of bad network connectivity, or because messages may be delivered to deleted chats for a short period of time (they will be ignored).
///
/// ----
///
/// Chat error (only used in WebSockets API).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatError {
    #[serde(rename = "chatError")]
    pub chat_error: errors::ChatError,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

/// ### Error events
///
/// Bots may log these events for debugging. There will be many error events - this does NOT indicate a malfunction - e.g., they may happen because of bad network connectivity, or because messages may be delivered to deleted chats for a short period of time (they will be ignored).
///
/// ----
///
/// Chat errors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatErrors {
    #[serde(rename = "chatErrors")]
    pub chat_errors: Vec<errors::ChatError>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}
