use super::*;
use crate::utils::CommandSyntax;

/// ### Address commands
///
/// Bots can use these commands to automatically check and create address when initialized
///
/// ----
///
/// Create bot address.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_address <userId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiCreateMyAddress {
    pub user_id: i64,
}

impl CommandSyntax for ApiCreateMyAddress {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_address ");
        buf.push_str(&self.user_id.to_string());
        buf
    }
}

/// ### Address commands
///
/// Bots can use these commands to automatically check and create address when initialized
///
/// ----
///
/// Delete bot address.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_delete_address <userId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiDeleteMyAddress {
    pub user_id: i64,
}

impl CommandSyntax for ApiDeleteMyAddress {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_delete_address ");
        buf.push_str(&self.user_id.to_string());
        buf
    }
}

/// ### Address commands
///
/// Bots can use these commands to automatically check and create address when initialized
///
/// ----
///
/// Get bot address and settings.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_show_address <userId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiShowMyAddress {
    pub user_id: i64,
}

impl CommandSyntax for ApiShowMyAddress {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_show_address ");
        buf.push_str(&self.user_id.to_string());
        buf
    }
}

/// ### Address commands
///
/// Bots can use these commands to automatically check and create address when initialized
///
/// ----
///
/// Add address to bot profile.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_profile_address <userId> on|off
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetProfileAddress {
    pub user_id: i64,
    pub enable: bool,
}

impl CommandSyntax for ApiSetProfileAddress {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_profile_address ");
        buf.push_str(&self.user_id.to_string());
        buf.push(' ');
        if self.enable {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
        buf
    }
}

/// ### Address commands
///
/// Bots can use these commands to automatically check and create address when initialized
///
/// ----
///
/// Set bot address settings.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_address_settings <userId> <json(settings)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetAddressSettings {
    pub user_id: i64,
    pub settings: AddressSettings,
}

impl CommandSyntax for ApiSetAddressSettings {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_address_settings ");
        buf.push_str(&self.user_id.to_string());
        buf.push(' ');
        buf.push_str(&serde_json::to_string(&self.settings).unwrap());
        buf
    }
}

/// ### Message commands
///
/// Commands to send, update, delete, moderate messages and set message reactions
///
/// ----
///
/// Send messages.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_send <str(sendRef)>[ live=on][ ttl=<ttl>] json <json(composedMessages)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSendMessages {
    pub send_ref: ChatRef,
    pub live_message: bool,
    pub ttl: Option<i32>,
    pub composed_messages: Vec<ComposedMessage>,
}

impl CommandSyntax for ApiSendMessages {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_send ");
        buf.push_str(&self.send_ref.interpret());
        if self.live_message {
            buf.push(' ');
            buf.push_str("live=");
            buf.push_str("on");
        }
        if let Some(ttl) = &self.ttl {
            buf.push(' ');
            buf.push_str("ttl=");
            buf.push_str(&ttl.to_string());
        }
        buf.push(' ');
        buf.push_str("json ");
        buf.push_str(&serde_json::to_string(&self.composed_messages).unwrap());
        buf
    }
}

/// ### Message commands
///
/// Commands to send, update, delete, moderate messages and set message reactions
///
/// ----
///
/// Update message.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_update item <str(chatRef)> <chatItemId>[ live=on] json <json(updatedMessage)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiUpdateChatItem {
    pub chat_ref: ChatRef,
    pub chat_item_id: i64,
    pub live_message: bool,
    pub updated_message: UpdatedMessage,
}

impl CommandSyntax for ApiUpdateChatItem {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_update ");
        buf.push_str("item ");
        buf.push_str(&self.chat_ref.interpret());
        buf.push(' ');
        buf.push_str(&self.chat_item_id.to_string());
        if self.live_message {
            buf.push(' ');
            buf.push_str("live=");
            buf.push_str("on");
        }
        buf.push(' ');
        buf.push_str("json ");
        buf.push_str(&serde_json::to_string(&self.updated_message).unwrap());
        buf
    }
}

/// ### Message commands
///
/// Commands to send, update, delete, moderate messages and set message reactions
///
/// ----
///
/// Delete message.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_delete item <str(chatRef)> <chatItemIds[0]>[,<chatItemIds[1]>...] broadcast|internal|internalMark
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiDeleteChatItem {
    pub chat_ref: ChatRef,
    pub chat_item_ids: Vec<i64>,
    pub delete_mode: CIDeleteMode,
}

impl CommandSyntax for ApiDeleteChatItem {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_delete ");
        buf.push_str("item ");
        buf.push_str(&self.chat_ref.interpret());
        buf.push(' ');
        let mut iter = self.chat_item_ids.iter();
        if let Some(el) = iter.next() {
            buf.push_str(&el.to_string());
        }
        for el in iter {
            buf.push(',');
            buf.push_str(&el.to_string());
        }
        buf.push(' ');
        match self.delete_mode {
            CIDeleteMode::Broadcast => {
                buf.push_str("broadcast");
            }
            CIDeleteMode::Internal => {
                buf.push_str("internal");
            }
            CIDeleteMode::InternalMark => {
                buf.push_str("internalMark");
            }
        }
        buf
    }
}

/// ### Message commands
///
/// Commands to send, update, delete, moderate messages and set message reactions
///
/// ----
///
/// Moderate message. Requires Moderator role (and higher than message author's).
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_delete member item #<groupId> <chatItemIds[0]>[,<chatItemIds[1]>...]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiDeleteMemberChatItem {
    pub group_id: i64,
    pub chat_item_ids: Vec<i64>,
}

impl CommandSyntax for ApiDeleteMemberChatItem {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_delete ");
        buf.push_str("member ");
        buf.push_str("item ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        let mut iter = self.chat_item_ids.iter();
        if let Some(el) = iter.next() {
            buf.push_str(&el.to_string());
        }
        for el in iter {
            buf.push(',');
            buf.push_str(&el.to_string());
        }
        buf
    }
}

/// ### Message commands
///
/// Commands to send, update, delete, moderate messages and set message reactions
///
/// ----
///
/// Add/remove message reaction.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_reaction <str(chatRef)> <chatItemId> on|off <json(reaction)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiChatItemReaction {
    pub chat_ref: ChatRef,
    pub chat_item_id: i64,
    pub add: bool,
    pub reaction: MsgReaction,
}

impl CommandSyntax for ApiChatItemReaction {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_reaction ");
        buf.push_str(&self.chat_ref.interpret());
        buf.push(' ');
        buf.push_str(&self.chat_item_id.to_string());
        buf.push(' ');
        if self.add {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
        buf.push(' ');
        buf.push_str(&serde_json::to_string(&self.reaction).unwrap());
        buf
    }
}

/// ### File commands
///
/// Commands to receive and to cancel files. Files are sent as part of the message, there are no separate commands to send files.
///
/// ----
///
/// Receive file.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /freceive <fileId>[ approved_relays=on][ encrypt=on|off][ inline=on|off][ <filePath>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ReceiveFile {
    pub file_id: i64,
    pub user_approved_relays: bool,
    pub store_encrypted: Option<bool>,
    pub file_inline: Option<bool>,
    pub file_path: Option<String>,
}

impl CommandSyntax for ReceiveFile {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/freceive ");
        buf.push_str(&self.file_id.to_string());
        if self.user_approved_relays {
            buf.push(' ');
            buf.push_str("approved_relays=");
            buf.push_str("on");
        }
        if let Some(store_encrypted) = &self.store_encrypted {
            buf.push(' ');
            buf.push_str("encrypt=");
            if *store_encrypted {
                buf.push_str("on");
            } else {
                buf.push_str("off");
            }
        }
        if let Some(file_inline) = &self.file_inline {
            buf.push(' ');
            buf.push_str("inline=");
            if *file_inline {
                buf.push_str("on");
            } else {
                buf.push_str("off");
            }
        }
        if let Some(file_path) = &self.file_path {
            buf.push(' ');
            buf.push_str(&file_path.to_string());
        }
        buf
    }
}

/// ### File commands
///
/// Commands to receive and to cancel files. Files are sent as part of the message, there are no separate commands to send files.
///
/// ----
///
/// Cancel file.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /fcancel <fileId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct CancelFile {
    pub file_id: i64,
}

impl CommandSyntax for CancelFile {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/fcancel ");
        buf.push_str(&self.file_id.to_string());
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Add contact to group. Requires bot to have Admin role.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_add #<groupId> <contactId> observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiAddMember {
    pub group_id: i64,
    pub contact_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiAddMember {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_add ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        buf.push_str(&self.contact_id.to_string());
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Observer => {
                buf.push_str("observer");
            }
            GroupMemberRole::Author => {
                buf.push_str("author");
            }
            GroupMemberRole::Member => {
                buf.push_str("member");
            }
            GroupMemberRole::Moderator => {
                buf.push_str("moderator");
            }
            GroupMemberRole::Admin => {
                buf.push_str("admin");
            }
            GroupMemberRole::Owner => {
                buf.push_str("owner");
            }
        }
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Join group.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_join #<groupId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiJoinGroup {
    pub group_id: i64,
}

impl CommandSyntax for ApiJoinGroup {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_join ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Accept group member. Requires Admin role.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_accept member #<groupId> <groupMemberId> observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiAcceptMember {
    pub group_id: i64,
    pub group_member_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiAcceptMember {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_accept ");
        buf.push_str("member ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        buf.push_str(&self.group_member_id.to_string());
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Observer => {
                buf.push_str("observer");
            }
            GroupMemberRole::Author => {
                buf.push_str("author");
            }
            GroupMemberRole::Member => {
                buf.push_str("member");
            }
            GroupMemberRole::Moderator => {
                buf.push_str("moderator");
            }
            GroupMemberRole::Admin => {
                buf.push_str("admin");
            }
            GroupMemberRole::Owner => {
                buf.push_str("owner");
            }
        }
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Set members role. Requires Admin role.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_member role #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...] observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiMembersRole {
    pub group_id: i64,
    pub group_member_ids: Vec<i64>,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiMembersRole {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_member ");
        buf.push_str("role ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        let mut iter = self.group_member_ids.iter();
        if let Some(el) = iter.next() {
            buf.push_str(&el.to_string());
        }
        for el in iter {
            buf.push(',');
            buf.push_str(&el.to_string());
        }
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Observer => {
                buf.push_str("observer");
            }
            GroupMemberRole::Author => {
                buf.push_str("author");
            }
            GroupMemberRole::Member => {
                buf.push_str("member");
            }
            GroupMemberRole::Moderator => {
                buf.push_str("moderator");
            }
            GroupMemberRole::Admin => {
                buf.push_str("admin");
            }
            GroupMemberRole::Owner => {
                buf.push_str("owner");
            }
        }
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Block members. Requires Moderator role.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_block #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...] blocked=on|off
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiBlockMembersForAll {
    pub group_id: i64,
    pub group_member_ids: Vec<i64>,
    pub blocked: bool,
}

impl CommandSyntax for ApiBlockMembersForAll {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_block ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        let mut iter = self.group_member_ids.iter();
        if let Some(el) = iter.next() {
            buf.push_str(&el.to_string());
        }
        for el in iter {
            buf.push(',');
            buf.push_str(&el.to_string());
        }
        buf.push(' ');
        buf.push_str("blocked=");
        if self.blocked {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Remove members. Requires Admin role.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_remove #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...][ messages=on]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiRemoveMembers {
    pub group_id: i64,
    pub group_member_ids: Vec<i64>,
    pub with_messages: bool,
}

impl CommandSyntax for ApiRemoveMembers {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_remove ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        let mut iter = self.group_member_ids.iter();
        if let Some(el) = iter.next() {
            buf.push_str(&el.to_string());
        }
        for el in iter {
            buf.push(',');
            buf.push_str(&el.to_string());
        }
        if self.with_messages {
            buf.push(' ');
            buf.push_str("messages=");
            buf.push_str("on");
        }
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Leave group.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_leave #<groupId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiLeaveGroup {
    pub group_id: i64,
}

impl CommandSyntax for ApiLeaveGroup {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_leave ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Get group members.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_members #<groupId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiListMembers {
    pub group_id: i64,
}

impl CommandSyntax for ApiListMembers {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_members ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Create group.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_group <userId>[ incognito=on] <json(groupProfile)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiNewGroup {
    pub user_id: i64,
    pub incognito: bool,
    pub group_profile: GroupProfile,
}

impl CommandSyntax for ApiNewGroup {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_group ");
        buf.push_str(&self.user_id.to_string());
        if self.incognito {
            buf.push(' ');
            buf.push_str("incognito=");
            buf.push_str("on");
        }
        buf.push(' ');
        buf.push_str(&serde_json::to_string(&self.group_profile).unwrap());
        buf
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Update group profile.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_group_profile #<groupId> <json(groupProfile)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiUpdateGroupProfile {
    pub group_id: i64,
    pub group_profile: GroupProfile,
}

impl CommandSyntax for ApiUpdateGroupProfile {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_group_profile ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        buf.push_str(&serde_json::to_string(&self.group_profile).unwrap());
        buf
    }
}

/// ### Group link commands
///
/// These commands can be used by bots that manage multiple public groups
///
/// ----
///
/// Create group link.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_create link #<groupId> observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiCreateGroupLink {
    pub group_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiCreateGroupLink {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_create ");
        buf.push_str("link ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Observer => {
                buf.push_str("observer");
            }
            GroupMemberRole::Author => {
                buf.push_str("author");
            }
            GroupMemberRole::Member => {
                buf.push_str("member");
            }
            GroupMemberRole::Moderator => {
                buf.push_str("moderator");
            }
            GroupMemberRole::Admin => {
                buf.push_str("admin");
            }
            GroupMemberRole::Owner => {
                buf.push_str("owner");
            }
        }
        buf
    }
}

/// ### Group link commands
///
/// These commands can be used by bots that manage multiple public groups
///
/// ----
///
/// Set member role for group link.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_set link role #<groupId> observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiGroupLinkMemberRole {
    pub group_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiGroupLinkMemberRole {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_set ");
        buf.push_str("link ");
        buf.push_str("role ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Observer => {
                buf.push_str("observer");
            }
            GroupMemberRole::Author => {
                buf.push_str("author");
            }
            GroupMemberRole::Member => {
                buf.push_str("member");
            }
            GroupMemberRole::Moderator => {
                buf.push_str("moderator");
            }
            GroupMemberRole::Admin => {
                buf.push_str("admin");
            }
            GroupMemberRole::Owner => {
                buf.push_str("owner");
            }
        }
        buf
    }
}

/// ### Group link commands
///
/// These commands can be used by bots that manage multiple public groups
///
/// ----
///
/// Delete group link.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_delete link #<groupId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiDeleteGroupLink {
    pub group_id: i64,
}

impl CommandSyntax for ApiDeleteGroupLink {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_delete ");
        buf.push_str("link ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf
    }
}

/// ### Group link commands
///
/// These commands can be used by bots that manage multiple public groups
///
/// ----
///
/// Get group link.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_get link #<groupId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiGetGroupLink {
    pub group_id: i64,
}

impl CommandSyntax for ApiGetGroupLink {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_get ");
        buf.push_str("link ");
        buf.push('#');
        buf.push_str(&self.group_id.to_string());
        buf
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Create 1-time invitation link.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_connect <userId>[ incognito=on]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiAddContact {
    pub user_id: i64,
    pub incognito: bool,
}

impl CommandSyntax for ApiAddContact {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_connect ");
        buf.push_str(&self.user_id.to_string());
        if self.incognito {
            buf.push(' ');
            buf.push_str("incognito=");
            buf.push_str("on");
        }
        buf
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Determine SimpleX link type and if the bot is already connected via this link.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_connect plan <userId> <connectionLink>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiConnectPlan {
    pub user_id: i64,
    pub connection_link: Option<String>,
}

impl CommandSyntax for ApiConnectPlan {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_connect ");
        buf.push_str("plan ");
        buf.push_str(&self.user_id.to_string());
        buf.push(' ');
        buf.push_str(
            &self
                .connection_link
                .as_deref()
                .unwrap_or_default()
                .to_string(),
        );
        buf
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Connect via prepared SimpleX link. The link can be 1-time invitation link, contact address or group link
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_connect <userId>[ <str(preparedLink_)>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiConnect {
    pub user_id: i64,
    pub incognito: bool,
    pub prepared_link: Option<CreatedConnLink>,
}

impl CommandSyntax for ApiConnect {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_connect ");
        buf.push_str(&self.user_id.to_string());
        if let Some(prepared_link) = &self.prepared_link {
            buf.push(' ');
            buf.push_str(&prepared_link.interpret());
        }
        buf
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Connect via SimpleX link as string in the active user profile.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /connect[ <connLink_>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct Connect {
    pub incognito: bool,
    pub conn_link: Option<String>,
}

impl CommandSyntax for Connect {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/connect");
        if let Some(conn_link) = &self.conn_link {
            buf.push(' ');
            buf.push_str(&conn_link.to_string());
        }
        buf
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Accept contact request.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_accept <contactReqId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiAcceptContact {
    pub contact_req_id: i64,
}

impl CommandSyntax for ApiAcceptContact {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_accept ");
        buf.push_str(&self.contact_req_id.to_string());
        buf
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Reject contact request. The user who sent the request is **not notified**.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_reject <contactReqId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiRejectContact {
    pub contact_req_id: i64,
}

impl CommandSyntax for ApiRejectContact {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_reject ");
        buf.push_str(&self.contact_req_id.to_string());
        buf
    }
}

/// ### Chat commands
///
/// Commands to list and delete conversations.
///
/// ----
///
/// Get contacts.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_contacts <userId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiListContacts {
    pub user_id: i64,
}

impl CommandSyntax for ApiListContacts {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_contacts ");
        buf.push_str(&self.user_id.to_string());
        buf
    }
}

/// ### Chat commands
///
/// Commands to list and delete conversations.
///
/// ----
///
/// Get groups.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_groups <userId>[ @<contactId_>][ <search>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiListGroups {
    pub user_id: i64,
    pub contact_id: Option<i64>,
    pub search: Option<String>,
}

impl CommandSyntax for ApiListGroups {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str("/_groups ");
        buf.push_str(&self.user_id.to_string());
        if let Some(contact_id) = &self.contact_id {
            buf.push(' ');
            buf.push('@');
            buf.push_str(&contact_id.to_string());
        }
        if let Some(search) = &self.search {
            buf.push(' ');
            buf.push_str(&search.to_string());
        }
        buf
    }
}

/// ### Chat commands
///
/// Commands to list and delete conversations.
///
/// ----
///
/// Delete chat.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_delete <str(chatRef)> <str(chatDeleteMode)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiDeleteChat {
    pub chat_ref: ChatRef,
    pub chat_delete_mode: ChatDeleteMode,
}

impl CommandSyntax for ApiDeleteChat {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("/_delete ");
        buf.push_str(&self.chat_ref.interpret());
        buf.push(' ');
        buf.push_str(&self.chat_delete_mode.interpret());
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Get active user profile
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /user
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ShowActiveUser {}

impl CommandSyntax for ShowActiveUser {
    fn interpret(&self) -> String {
        let mut buf = String::new();
        buf.push_str("/user");
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Create new user profile
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_create user <json(newUser)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct CreateActiveUser {
    pub new_user: NewUser,
}

impl CommandSyntax for CreateActiveUser {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_create ");
        buf.push_str("user ");
        buf.push_str(&serde_json::to_string(&self.new_user).unwrap());
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Get all user profiles
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /users
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ListUsers {}

impl CommandSyntax for ListUsers {
    fn interpret(&self) -> String {
        let mut buf = String::new();
        buf.push_str("/users");
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Set active user profile
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_user <userId>[ <json(viewPwd)>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetActiveUser {
    pub user_id: i64,
    pub view_pwd: Option<String>,
}

impl CommandSyntax for ApiSetActiveUser {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_user ");
        buf.push_str(&self.user_id.to_string());
        if let Some(view_pwd) = &self.view_pwd {
            buf.push(' ');
            buf.push_str(&serde_json::to_string(&view_pwd).unwrap());
        }
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Delete user profile.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_delete user <userId> del_smp=on|off[ <json(viewPwd)>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiDeleteUser {
    pub user_id: i64,
    pub del_smp_queues: bool,
    pub view_pwd: Option<String>,
}

impl CommandSyntax for ApiDeleteUser {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_delete ");
        buf.push_str("user ");
        buf.push_str(&self.user_id.to_string());
        buf.push(' ');
        buf.push_str("del_smp=");
        if self.del_smp_queues {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
        if let Some(view_pwd) = &self.view_pwd {
            buf.push(' ');
            buf.push_str(&serde_json::to_string(&view_pwd).unwrap());
        }
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Update user profile.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_profile <userId> <json(profile)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiUpdateProfile {
    pub user_id: i64,
    pub profile: Profile,
}

impl CommandSyntax for ApiUpdateProfile {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_profile ");
        buf.push_str(&self.user_id.to_string());
        buf.push(' ');
        buf.push_str(&serde_json::to_string(&self.profile).unwrap());
        buf
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Configure chat preference overrides for the contact.
///
/// *Network usage*: background.
///
/// *Syntax:*
///
/// ```
/// /_set prefs @<contactId> <json(preferences)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetContactPrefs {
    pub contact_id: i64,
    pub preferences: Preferences,
}

impl CommandSyntax for ApiSetContactPrefs {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(1024);
        buf.push_str("/_set ");
        buf.push_str("prefs ");
        buf.push('@');
        buf.push_str(&self.contact_id.to_string());
        buf.push(' ');
        buf.push_str(&serde_json::to_string(&self.preferences).unwrap());
        buf
    }
}
