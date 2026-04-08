#[allow(unused_imports)]
use crate::commands::*;
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

impl Event {
    pub fn kind(&self) -> EventKind {
        match self {
            Self::ContactConnected(_) => EventKind::ContactConnected,
            Self::ContactUpdated(_) => EventKind::ContactUpdated,
            Self::ContactDeletedByContact(_) => EventKind::ContactDeletedByContact,
            Self::ReceivedContactRequest(_) => EventKind::ReceivedContactRequest,
            Self::NewMemberContactReceivedInv(_) => EventKind::NewMemberContactReceivedInv,
            Self::ContactSndReady(_) => EventKind::ContactSndReady,
            Self::NewChatItems(_) => EventKind::NewChatItems,
            Self::ChatItemReaction(_) => EventKind::ChatItemReaction,
            Self::ChatItemsDeleted(_) => EventKind::ChatItemsDeleted,
            Self::ChatItemUpdated(_) => EventKind::ChatItemUpdated,
            Self::GroupChatItemsDeleted(_) => EventKind::GroupChatItemsDeleted,
            Self::ChatItemsStatusesUpdated(_) => EventKind::ChatItemsStatusesUpdated,
            Self::ReceivedGroupInvitation(_) => EventKind::ReceivedGroupInvitation,
            Self::UserJoinedGroup(_) => EventKind::UserJoinedGroup,
            Self::GroupUpdated(_) => EventKind::GroupUpdated,
            Self::JoinedGroupMember(_) => EventKind::JoinedGroupMember,
            Self::MemberRole(_) => EventKind::MemberRole,
            Self::DeletedMember(_) => EventKind::DeletedMember,
            Self::LeftMember(_) => EventKind::LeftMember,
            Self::DeletedMemberUser(_) => EventKind::DeletedMemberUser,
            Self::GroupDeleted(_) => EventKind::GroupDeleted,
            Self::ConnectedToGroupMember(_) => EventKind::ConnectedToGroupMember,
            Self::MemberAcceptedByOther(_) => EventKind::MemberAcceptedByOther,
            Self::MemberBlockedForAll(_) => EventKind::MemberBlockedForAll,
            Self::GroupMemberUpdated(_) => EventKind::GroupMemberUpdated,
            Self::GroupLinkDataUpdated(_) => EventKind::GroupLinkDataUpdated,
            Self::GroupRelayUpdated(_) => EventKind::GroupRelayUpdated,
            Self::RcvFileDescrReady(_) => EventKind::RcvFileDescrReady,
            Self::RcvFileComplete(_) => EventKind::RcvFileComplete,
            Self::SndFileCompleteXftp(_) => EventKind::SndFileCompleteXftp,
            Self::RcvFileStart(_) => EventKind::RcvFileStart,
            Self::RcvFileSndCancelled(_) => EventKind::RcvFileSndCancelled,
            Self::RcvFileAccepted(_) => EventKind::RcvFileAccepted,
            Self::RcvFileError(_) => EventKind::RcvFileError,
            Self::RcvFileWarning(_) => EventKind::RcvFileWarning,
            Self::SndFileError(_) => EventKind::SndFileError,
            Self::SndFileWarning(_) => EventKind::SndFileWarning,
            Self::AcceptingContactRequest(_) => EventKind::AcceptingContactRequest,
            Self::AcceptingBusinessRequest(_) => EventKind::AcceptingBusinessRequest,
            Self::ContactConnecting(_) => EventKind::ContactConnecting,
            Self::BusinessLinkConnecting(_) => EventKind::BusinessLinkConnecting,
            Self::JoinedGroupMemberConnecting(_) => EventKind::JoinedGroupMemberConnecting,
            Self::SentGroupInvitation(_) => EventKind::SentGroupInvitation,
            Self::GroupLinkConnecting(_) => EventKind::GroupLinkConnecting,
            Self::HostConnected(_) => EventKind::HostConnected,
            Self::HostDisconnected(_) => EventKind::HostDisconnected,
            Self::SubscriptionStatus(_) => EventKind::SubscriptionStatus,
            Self::MessageError(_) => EventKind::MessageError,
            Self::ChatError(_) => EventKind::ChatError,
            Self::ChatErrors(_) => EventKind::ChatErrors,
            Self::Undocumented(_) => EventKind::Undocumented,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventKind {
    ContactConnected,
    ContactUpdated,
    ContactDeletedByContact,
    ReceivedContactRequest,
    NewMemberContactReceivedInv,
    ContactSndReady,
    NewChatItems,
    ChatItemReaction,
    ChatItemsDeleted,
    ChatItemUpdated,
    GroupChatItemsDeleted,
    ChatItemsStatusesUpdated,
    ReceivedGroupInvitation,
    UserJoinedGroup,
    GroupUpdated,
    JoinedGroupMember,
    MemberRole,
    DeletedMember,
    LeftMember,
    DeletedMemberUser,
    GroupDeleted,
    ConnectedToGroupMember,
    MemberAcceptedByOther,
    MemberBlockedForAll,
    GroupMemberUpdated,
    GroupLinkDataUpdated,
    GroupRelayUpdated,
    RcvFileDescrReady,
    RcvFileComplete,
    SndFileCompleteXftp,
    RcvFileStart,
    RcvFileSndCancelled,
    RcvFileAccepted,
    RcvFileError,
    RcvFileWarning,
    SndFileError,
    SndFileWarning,
    AcceptingContactRequest,
    AcceptingBusinessRequest,
    ContactConnecting,
    BusinessLinkConnecting,
    JoinedGroupMemberConnecting,
    SentGroupInvitation,
    GroupLinkConnecting,
    HostConnected,
    HostDisconnected,
    SubscriptionStatus,
    MessageError,
    ChatError,
    ChatErrors,
    Undocumented,
}

impl EventKind {
    pub const COUNT: usize = 51;

    pub fn as_usize(&self) -> usize {
        match self {
            Self::ContactConnected => 0,
            Self::ContactUpdated => 1,
            Self::ContactDeletedByContact => 2,
            Self::ReceivedContactRequest => 3,
            Self::NewMemberContactReceivedInv => 4,
            Self::ContactSndReady => 5,
            Self::NewChatItems => 6,
            Self::ChatItemReaction => 7,
            Self::ChatItemsDeleted => 8,
            Self::ChatItemUpdated => 9,
            Self::GroupChatItemsDeleted => 10,
            Self::ChatItemsStatusesUpdated => 11,
            Self::ReceivedGroupInvitation => 12,
            Self::UserJoinedGroup => 13,
            Self::GroupUpdated => 14,
            Self::JoinedGroupMember => 15,
            Self::MemberRole => 16,
            Self::DeletedMember => 17,
            Self::LeftMember => 18,
            Self::DeletedMemberUser => 19,
            Self::GroupDeleted => 20,
            Self::ConnectedToGroupMember => 21,
            Self::MemberAcceptedByOther => 22,
            Self::MemberBlockedForAll => 23,
            Self::GroupMemberUpdated => 24,
            Self::GroupLinkDataUpdated => 25,
            Self::GroupRelayUpdated => 26,
            Self::RcvFileDescrReady => 27,
            Self::RcvFileComplete => 28,
            Self::SndFileCompleteXftp => 29,
            Self::RcvFileStart => 30,
            Self::RcvFileSndCancelled => 31,
            Self::RcvFileAccepted => 32,
            Self::RcvFileError => 33,
            Self::RcvFileWarning => 34,
            Self::SndFileError => 35,
            Self::SndFileWarning => 36,
            Self::AcceptingContactRequest => 37,
            Self::AcceptingBusinessRequest => 38,
            Self::ContactConnecting => 39,
            Self::BusinessLinkConnecting => 40,
            Self::JoinedGroupMemberConnecting => 41,
            Self::SentGroupInvitation => 42,
            Self::GroupLinkConnecting => 43,
            Self::HostConnected => 44,
            Self::HostDisconnected => 45,
            Self::SubscriptionStatus => 46,
            Self::MessageError => 47,
            Self::ChatError => 48,
            Self::ChatErrors => 49,
            Self::Undocumented => 50,
        }
    }
    pub fn from_type_str(type_str: &str) -> Self {
        match type_str {
            "contactConnected" => Self::ContactConnected,
            "contactUpdated" => Self::ContactUpdated,
            "contactDeletedByContact" => Self::ContactDeletedByContact,
            "receivedContactRequest" => Self::ReceivedContactRequest,
            "newMemberContactReceivedInv" => Self::NewMemberContactReceivedInv,
            "contactSndReady" => Self::ContactSndReady,
            "newChatItems" => Self::NewChatItems,
            "chatItemReaction" => Self::ChatItemReaction,
            "chatItemsDeleted" => Self::ChatItemsDeleted,
            "chatItemUpdated" => Self::ChatItemUpdated,
            "groupChatItemsDeleted" => Self::GroupChatItemsDeleted,
            "chatItemsStatusesUpdated" => Self::ChatItemsStatusesUpdated,
            "receivedGroupInvitation" => Self::ReceivedGroupInvitation,
            "userJoinedGroup" => Self::UserJoinedGroup,
            "groupUpdated" => Self::GroupUpdated,
            "joinedGroupMember" => Self::JoinedGroupMember,
            "memberRole" => Self::MemberRole,
            "deletedMember" => Self::DeletedMember,
            "leftMember" => Self::LeftMember,
            "deletedMemberUser" => Self::DeletedMemberUser,
            "groupDeleted" => Self::GroupDeleted,
            "connectedToGroupMember" => Self::ConnectedToGroupMember,
            "memberAcceptedByOther" => Self::MemberAcceptedByOther,
            "memberBlockedForAll" => Self::MemberBlockedForAll,
            "groupMemberUpdated" => Self::GroupMemberUpdated,
            "groupLinkDataUpdated" => Self::GroupLinkDataUpdated,
            "groupRelayUpdated" => Self::GroupRelayUpdated,
            "rcvFileDescrReady" => Self::RcvFileDescrReady,
            "rcvFileComplete" => Self::RcvFileComplete,
            "sndFileCompleteXFTP" => Self::SndFileCompleteXftp,
            "rcvFileStart" => Self::RcvFileStart,
            "rcvFileSndCancelled" => Self::RcvFileSndCancelled,
            "rcvFileAccepted" => Self::RcvFileAccepted,
            "rcvFileError" => Self::RcvFileError,
            "rcvFileWarning" => Self::RcvFileWarning,
            "sndFileError" => Self::SndFileError,
            "sndFileWarning" => Self::SndFileWarning,
            "acceptingContactRequest" => Self::AcceptingContactRequest,
            "acceptingBusinessRequest" => Self::AcceptingBusinessRequest,
            "contactConnecting" => Self::ContactConnecting,
            "businessLinkConnecting" => Self::BusinessLinkConnecting,
            "joinedGroupMemberConnecting" => Self::JoinedGroupMemberConnecting,
            "sentGroupInvitation" => Self::SentGroupInvitation,
            "groupLinkConnecting" => Self::GroupLinkConnecting,
            "hostConnected" => Self::HostConnected,
            "hostDisconnected" => Self::HostDisconnected,
            "subscriptionStatus" => Self::SubscriptionStatus,
            "messageError" => Self::MessageError,
            "chatError" => Self::ChatError,
            "chatErrors" => Self::ChatErrors,
            _ => Self::Undocumented,
        }
    }
}

/// Generalization of event data
pub trait EventData {
    const KIND: EventKind;

    fn from_event(event: Event) -> Result<Arc<Self>, Event>;

    fn into_event(self: Arc<Self>) -> Event;
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [ApiShowMyAddress] to check if address exists,
/// - [ApiCreateMyAddress] to create address,
/// - [ApiSetAddressSettings] to enable auto-access.
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

impl EventData for ContactConnected {
    const KIND: EventKind = EventKind::ContactConnected;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ContactConnected(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ContactConnected(self)
    }
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [ApiShowMyAddress] to check if address exists,
/// - [ApiCreateMyAddress] to create address,
/// - [ApiSetAddressSettings] to enable auto-access.
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

impl EventData for ContactUpdated {
    const KIND: EventKind = EventKind::ContactUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ContactUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ContactUpdated(self)
    }
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [ApiShowMyAddress] to check if address exists,
/// - [ApiCreateMyAddress] to create address,
/// - [ApiSetAddressSettings] to enable auto-access.
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

impl EventData for ContactDeletedByContact {
    const KIND: EventKind = EventKind::ContactDeletedByContact;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ContactDeletedByContact(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ContactDeletedByContact(self)
    }
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [ApiShowMyAddress] to check if address exists,
/// - [ApiCreateMyAddress] to create address,
/// - [ApiSetAddressSettings] to enable auto-access.
///
/// ----
///
/// Contact request received.
///
/// This event is only sent when auto-accept is disabled.
///
/// The request needs to be accepted using [ApiAcceptContact] command
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

impl EventData for ReceivedContactRequest {
    const KIND: EventKind = EventKind::ReceivedContactRequest;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ReceivedContactRequest(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ReceivedContactRequest(self)
    }
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [ApiShowMyAddress] to check if address exists,
/// - [ApiCreateMyAddress] to create address,
/// - [ApiSetAddressSettings] to enable auto-access.
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

impl EventData for NewMemberContactReceivedInv {
    const KIND: EventKind = EventKind::NewMemberContactReceivedInv;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::NewMemberContactReceivedInv(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::NewMemberContactReceivedInv(self)
    }
}

/// ### Contact connection events
///
/// Bots must use these events to process connecting users.
///
/// Most bots enable auto-accept and don't need to accept connections via commands.
///
/// You may create bot SimpleX address manually via CLI or desktop app or from bot code with these commands:
/// - [ApiShowMyAddress] to check if address exists,
/// - [ApiCreateMyAddress] to create address,
/// - [ApiSetAddressSettings] to enable auto-access.
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

impl EventData for ContactSndReady {
    const KIND: EventKind = EventKind::ContactSndReady;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ContactSndReady(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ContactSndReady(self)
    }
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

impl EventData for NewChatItems {
    const KIND: EventKind = EventKind::NewChatItems;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::NewChatItems(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::NewChatItems(self)
    }
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

impl EventData for ChatItemReaction {
    const KIND: EventKind = EventKind::ChatItemReaction;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ChatItemReaction(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ChatItemReaction(self)
    }
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

impl EventData for ChatItemsDeleted {
    const KIND: EventKind = EventKind::ChatItemsDeleted;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ChatItemsDeleted(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ChatItemsDeleted(self)
    }
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

impl EventData for ChatItemUpdated {
    const KIND: EventKind = EventKind::ChatItemUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ChatItemUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ChatItemUpdated(self)
    }
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

impl EventData for GroupChatItemsDeleted {
    const KIND: EventKind = EventKind::GroupChatItemsDeleted;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupChatItemsDeleted(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupChatItemsDeleted(self)
    }
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

impl EventData for ChatItemsStatusesUpdated {
    const KIND: EventKind = EventKind::ChatItemsStatusesUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ChatItemsStatusesUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ChatItemsStatusesUpdated(self)
    }
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

impl EventData for ReceivedGroupInvitation {
    const KIND: EventKind = EventKind::ReceivedGroupInvitation;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ReceivedGroupInvitation(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ReceivedGroupInvitation(self)
    }
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

impl EventData for UserJoinedGroup {
    const KIND: EventKind = EventKind::UserJoinedGroup;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::UserJoinedGroup(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::UserJoinedGroup(self)
    }
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

impl EventData for GroupUpdated {
    const KIND: EventKind = EventKind::GroupUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupUpdated(self)
    }
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

impl EventData for JoinedGroupMember {
    const KIND: EventKind = EventKind::JoinedGroupMember;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::JoinedGroupMember(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::JoinedGroupMember(self)
    }
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

impl EventData for MemberRole {
    const KIND: EventKind = EventKind::MemberRole;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::MemberRole(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::MemberRole(self)
    }
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

impl EventData for DeletedMember {
    const KIND: EventKind = EventKind::DeletedMember;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::DeletedMember(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::DeletedMember(self)
    }
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

impl EventData for LeftMember {
    const KIND: EventKind = EventKind::LeftMember;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::LeftMember(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::LeftMember(self)
    }
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

impl EventData for DeletedMemberUser {
    const KIND: EventKind = EventKind::DeletedMemberUser;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::DeletedMemberUser(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::DeletedMemberUser(self)
    }
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

impl EventData for GroupDeleted {
    const KIND: EventKind = EventKind::GroupDeleted;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupDeleted(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupDeleted(self)
    }
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

impl EventData for ConnectedToGroupMember {
    const KIND: EventKind = EventKind::ConnectedToGroupMember;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ConnectedToGroupMember(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ConnectedToGroupMember(self)
    }
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

impl EventData for MemberAcceptedByOther {
    const KIND: EventKind = EventKind::MemberAcceptedByOther;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::MemberAcceptedByOther(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::MemberAcceptedByOther(self)
    }
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

impl EventData for MemberBlockedForAll {
    const KIND: EventKind = EventKind::MemberBlockedForAll;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::MemberBlockedForAll(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::MemberBlockedForAll(self)
    }
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

impl EventData for GroupMemberUpdated {
    const KIND: EventKind = EventKind::GroupMemberUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupMemberUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupMemberUpdated(self)
    }
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

impl EventData for GroupLinkDataUpdated {
    const KIND: EventKind = EventKind::GroupLinkDataUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupLinkDataUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupLinkDataUpdated(self)
    }
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

impl EventData for GroupRelayUpdated {
    const KIND: EventKind = EventKind::GroupRelayUpdated;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupRelayUpdated(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupRelayUpdated(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
///
/// ----
///
/// File is ready to be received.
///
/// This event is useful for processing sender file servers and monitoring file reception progress.
///
/// [ReceiveFile] command can be used before this event.
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

impl EventData for RcvFileDescrReady {
    const KIND: EventKind = EventKind::RcvFileDescrReady;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileDescrReady(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileDescrReady(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for RcvFileComplete {
    const KIND: EventKind = EventKind::RcvFileComplete;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileComplete(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileComplete(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for SndFileCompleteXftp {
    const KIND: EventKind = EventKind::SndFileCompleteXftp;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::SndFileCompleteXftp(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::SndFileCompleteXftp(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
///
/// ----
///
/// File reception started. This event will be sent after [CEvtRcvFileDescrReady] event.
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

impl EventData for RcvFileStart {
    const KIND: EventKind = EventKind::RcvFileStart;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileStart(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileStart(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
///
/// ----
///
/// File was cancelled by the sender. This event may be sent instead of [CEvtRcvFileDescrReady] event.
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

impl EventData for RcvFileSndCancelled {
    const KIND: EventKind = EventKind::RcvFileSndCancelled;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileSndCancelled(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileSndCancelled(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for RcvFileAccepted {
    const KIND: EventKind = EventKind::RcvFileAccepted;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileAccepted(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileAccepted(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for RcvFileError {
    const KIND: EventKind = EventKind::RcvFileError;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileError(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileError(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for RcvFileWarning {
    const KIND: EventKind = EventKind::RcvFileWarning;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::RcvFileWarning(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::RcvFileWarning(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for SndFileError {
    const KIND: EventKind = EventKind::SndFileError;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::SndFileError(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::SndFileError(self)
    }
}

/// ### File events
///
/// Bots that send or receive files may process these events to track delivery status and to process completion.
///
/// Bots that need to receive or moderate files (e.g., based on name, size or extension), can use relevant commands (e.g., [ReceiveFile] or [ApiDeleteMemberChatItem]) when processing [NewChatItems] event.
///
/// Bots that need to send files should use [ApiSendMessages] command.
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

impl EventData for SndFileWarning {
    const KIND: EventKind = EventKind::SndFileWarning;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::SndFileWarning(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::SndFileWarning(self)
    }
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

impl EventData for AcceptingContactRequest {
    const KIND: EventKind = EventKind::AcceptingContactRequest;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::AcceptingContactRequest(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::AcceptingContactRequest(self)
    }
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

impl EventData for AcceptingBusinessRequest {
    const KIND: EventKind = EventKind::AcceptingBusinessRequest;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::AcceptingBusinessRequest(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::AcceptingBusinessRequest(self)
    }
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

impl EventData for ContactConnecting {
    const KIND: EventKind = EventKind::ContactConnecting;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ContactConnecting(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ContactConnecting(self)
    }
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

impl EventData for BusinessLinkConnecting {
    const KIND: EventKind = EventKind::BusinessLinkConnecting;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::BusinessLinkConnecting(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::BusinessLinkConnecting(self)
    }
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

impl EventData for JoinedGroupMemberConnecting {
    const KIND: EventKind = EventKind::JoinedGroupMemberConnecting;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::JoinedGroupMemberConnecting(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::JoinedGroupMemberConnecting(self)
    }
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

impl EventData for SentGroupInvitation {
    const KIND: EventKind = EventKind::SentGroupInvitation;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::SentGroupInvitation(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::SentGroupInvitation(self)
    }
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

impl EventData for GroupLinkConnecting {
    const KIND: EventKind = EventKind::GroupLinkConnecting;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::GroupLinkConnecting(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::GroupLinkConnecting(self)
    }
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

impl EventData for HostConnected {
    const KIND: EventKind = EventKind::HostConnected;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::HostConnected(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::HostConnected(self)
    }
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

impl EventData for HostDisconnected {
    const KIND: EventKind = EventKind::HostDisconnected;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::HostDisconnected(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::HostDisconnected(self)
    }
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

impl EventData for SubscriptionStatus {
    const KIND: EventKind = EventKind::SubscriptionStatus;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::SubscriptionStatus(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::SubscriptionStatus(self)
    }
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

impl EventData for MessageError {
    const KIND: EventKind = EventKind::MessageError;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::MessageError(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::MessageError(self)
    }
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

impl EventData for ChatError {
    const KIND: EventKind = EventKind::ChatError;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ChatError(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ChatError(self)
    }
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

impl EventData for ChatErrors {
    const KIND: EventKind = EventKind::ChatErrors;
    fn from_event(ev: Event) -> Result<Arc<Self>, Event> {
        if let Event::ChatErrors(data) = ev {
            Ok(data)
        } else {
            Err(ev)
        }
    }
    fn into_event(self: Arc<Self>) -> Event {
        Event::ChatErrors(self)
    }
}
