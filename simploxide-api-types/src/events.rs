use crate::{errors::*, *};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Event {
    /// Contact connection events
    #[serde(rename = "contactConnected")]
    ContactConnected(ContactConnected),
    /// Contact connection events
    #[serde(rename = "contactUpdated")]
    ContactUpdated(ContactUpdated),
    /// Contact connection events
    #[serde(rename = "contactDeletedByContact")]
    ContactDeletedByContact(ContactDeletedByContact),
    /// Contact connection events
    #[serde(rename = "receivedContactRequest")]
    ReceivedContactRequest(ReceivedContactRequest),
    /// Contact connection events
    #[serde(rename = "newMemberContactReceivedInv")]
    NewMemberContactReceivedInv(NewMemberContactReceivedInv),
    /// Contact connection events
    #[serde(rename = "contactSndReady")]
    ContactSndReady(ContactSndReady),
    /// Message events
    #[serde(rename = "newChatItems")]
    NewChatItems(NewChatItems),
    /// Message events
    #[serde(rename = "chatItemReaction")]
    ChatItemReaction(ChatItemReaction),
    /// Message events
    #[serde(rename = "chatItemsDeleted")]
    ChatItemsDeleted(ChatItemsDeleted),
    /// Message events
    #[serde(rename = "chatItemUpdated")]
    ChatItemUpdated(ChatItemUpdated),
    /// Message events
    #[serde(rename = "groupChatItemsDeleted")]
    GroupChatItemsDeleted(GroupChatItemsDeleted),
    /// Message events
    #[serde(rename = "chatItemsStatusesUpdated")]
    ChatItemsStatusesUpdated(ChatItemsStatusesUpdated),
    /// Group events
    #[serde(rename = "receivedGroupInvitation")]
    ReceivedGroupInvitation(ReceivedGroupInvitation),
    /// Group events
    #[serde(rename = "userJoinedGroup")]
    UserJoinedGroup(UserJoinedGroup),
    /// Group events
    #[serde(rename = "groupUpdated")]
    GroupUpdated(GroupUpdated),
    /// Group events
    #[serde(rename = "joinedGroupMember")]
    JoinedGroupMember(JoinedGroupMember),
    /// Group events
    #[serde(rename = "memberRole")]
    MemberRole(MemberRole),
    /// Group events
    #[serde(rename = "deletedMember")]
    DeletedMember(DeletedMember),
    /// Group events
    #[serde(rename = "leftMember")]
    LeftMember(LeftMember),
    /// Group events
    #[serde(rename = "deletedMemberUser")]
    DeletedMemberUser(DeletedMemberUser),
    /// Group events
    #[serde(rename = "groupDeleted")]
    GroupDeleted(GroupDeleted),
    /// Group events
    #[serde(rename = "connectedToGroupMember")]
    ConnectedToGroupMember(ConnectedToGroupMember),
    /// Group events
    #[serde(rename = "memberAcceptedByOther")]
    MemberAcceptedByOther(MemberAcceptedByOther),
    /// Group events
    #[serde(rename = "memberBlockedForAll")]
    MemberBlockedForAll(MemberBlockedForAll),
    /// Group events
    #[serde(rename = "groupMemberUpdated")]
    GroupMemberUpdated(GroupMemberUpdated),
    /// File events
    #[serde(rename = "rcvFileDescrReady")]
    RcvFileDescrReady(RcvFileDescrReady),
    /// File events
    #[serde(rename = "rcvFileComplete")]
    RcvFileComplete(RcvFileComplete),
    /// File events
    #[serde(rename = "sndFileCompleteXFTP")]
    SndFileCompleteXftp(SndFileCompleteXftp),
    /// File events
    #[serde(rename = "rcvFileStart")]
    RcvFileStart(RcvFileStart),
    /// File events
    #[serde(rename = "rcvFileSndCancelled")]
    RcvFileSndCancelled(RcvFileSndCancelled),
    /// File events
    #[serde(rename = "rcvFileAccepted")]
    RcvFileAccepted(RcvFileAccepted),
    /// File events
    #[serde(rename = "rcvFileError")]
    RcvFileError(RcvFileError),
    /// File events
    #[serde(rename = "rcvFileWarning")]
    RcvFileWarning(RcvFileWarning),
    /// File events
    #[serde(rename = "sndFileError")]
    SndFileError(SndFileError),
    /// File events
    #[serde(rename = "sndFileWarning")]
    SndFileWarning(SndFileWarning),
    /// Connection progress events
    #[serde(rename = "acceptingContactRequest")]
    AcceptingContactRequest(AcceptingContactRequest),
    /// Connection progress events
    #[serde(rename = "acceptingBusinessRequest")]
    AcceptingBusinessRequest(AcceptingBusinessRequest),
    /// Connection progress events
    #[serde(rename = "contactConnecting")]
    ContactConnecting(ContactConnecting),
    /// Connection progress events
    #[serde(rename = "businessLinkConnecting")]
    BusinessLinkConnecting(BusinessLinkConnecting),
    /// Connection progress events
    #[serde(rename = "joinedGroupMemberConnecting")]
    JoinedGroupMemberConnecting(JoinedGroupMemberConnecting),
    /// Connection progress events
    #[serde(rename = "sentGroupInvitation")]
    SentGroupInvitation(SentGroupInvitation),
    /// Connection progress events
    #[serde(rename = "groupLinkConnecting")]
    GroupLinkConnecting(GroupLinkConnecting),
    /// Error events
    #[serde(rename = "messageError")]
    MessageError(MessageError),
    /// Error events
    #[serde(rename = "chatError")]
    ChatError(ChatError),
    /// Error events
    #[serde(rename = "chatErrors")]
    ChatErrors(ChatErrors),
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
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(rename = "userCustomProfile", skip_serializing_if = "Option::is_none")]
    pub user_custom_profile: Option<Profile>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "fromContact")]
    pub from_contact: Contact,

    #[serde(rename = "toContact")]
    pub to_contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "contactRequest")]
    pub contact_request: UserContactRequest,

    #[serde(rename = "chat_", skip_serializing_if = "Option::is_none")]
    pub chat: Option<AChat>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<AChatItem>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "added")]
    pub added: bool,

    #[serde(rename = "reaction")]
    pub reaction: ACIReaction,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItemDeletions")]
    pub chat_item_deletions: Vec<ChatItemDeletion>,

    #[serde(rename = "byUser")]
    pub by_user: bool,

    #[serde(rename = "timed")]
    pub timed: bool,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "chatItemIDs")]
    pub chat_item_i_ds: Vec<i64>,

    #[serde(rename = "byUser")]
    pub by_user: bool,

    #[serde(rename = "member_", skip_serializing_if = "Option::is_none")]
    pub member: Option<GroupMember>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<AChatItem>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(rename = "fromMemberRole")]
    pub from_member_role: GroupMemberRole,

    #[serde(rename = "memberRole")]
    pub member_role: GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "fromGroup")]
    pub from_group: GroupInfo,

    #[serde(rename = "toGroup")]
    pub to_group: GroupInfo,

    #[serde(rename = "member_", skip_serializing_if = "Option::is_none")]
    pub member: Option<GroupMember>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "byMember")]
    pub by_member: GroupMember,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(rename = "fromRole")]
    pub from_role: GroupMemberRole,

    #[serde(rename = "toRole")]
    pub to_role: GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "byMember")]
    pub by_member: GroupMember,

    #[serde(rename = "deletedMember")]
    pub deleted_member: GroupMember,

    #[serde(rename = "withMessages")]
    pub with_messages: bool,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(rename = "withMessages")]
    pub with_messages: bool,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(rename = "memberContact", skip_serializing_if = "Option::is_none")]
    pub member_contact: Option<Contact>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "acceptingMember")]
    pub accepting_member: GroupMember,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "byMember")]
    pub by_member: GroupMember,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(rename = "blocked")]
    pub blocked: bool,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "fromMember")]
    pub from_member: GroupMember,

    #[serde(rename = "toMember")]
    pub to_member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(rename = "rcvFileDescr")]
    pub rcv_file_descr: RcvFileDescr,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: FileTransferMeta,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<AChatItem>,

    #[serde(rename = "agentError")]
    pub agent_error: errors::AgentErrorType,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<AChatItem>,

    #[serde(rename = "agentError")]
    pub agent_error: AgentErrorType,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<AChatItem>,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: FileTransferMeta,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "chatItem_", skip_serializing_if = "Option::is_none")]
    pub chat_item: Option<AChatItem>,

    #[serde(rename = "fileTransferMeta")]
    pub file_transfer_meta: FileTransferMeta,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: GroupMember,

    #[serde(rename = "fromContact")]
    pub from_contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: GroupMember,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(rename = "member")]
    pub member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "hostMember")]
    pub host_member: GroupMember,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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
    pub user: User,

    #[serde(rename = "severity")]
    pub severity: String,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

/// ### Error events
///
/// Bots may log these events for debugging. There will be many error events - this does NOT indicate a malfunction - e.g., they may happen because of bad network connectivity, or because messages may be delivered to deleted chats for a short period of time (they will be ignored).
///
/// ----
///
/// Chat error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatError {
    #[serde(rename = "chatError")]
    pub chat_error: errors::ChatError,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}
