use {crate::utils::CommandSyntax, crate::*};

use std::fmt::Write;
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_address ");
        write!(buf, "{}", self.user_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_delete_address ");
        write!(buf, "{}", self.user_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_show_address ");
        write!(buf, "{}", self.user_id).unwrap();
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

impl ApiSetProfileAddress {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            enable: false,
        }
    }
}

impl CommandSyntax for ApiSetProfileAddress {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_profile_address ");
        write!(buf, "{}", self.user_id).unwrap();
        buf.push(' ');
        if self.enable {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
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
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_address_settings ");
        write!(buf, "{}", self.user_id).unwrap();
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.settings).unwrap();
        }
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

impl ApiSendMessages {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(send_ref: ChatRef, composed_messages: Vec<ComposedMessage>) -> Self {
        Self {
            send_ref,
            live_message: false,
            ttl: None,
            composed_messages,
        }
    }
}

impl CommandSyntax for ApiSendMessages {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_send ");
        self.send_ref.append_command_syntax(buf);
        if self.live_message {
            buf.push(' ');
            buf.push_str("live=");
            buf.push_str("on");
        }
        if let Some(ttl) = &self.ttl {
            buf.push(' ');
            buf.push_str("ttl=");
            write!(buf, "{}", ttl).unwrap();
        }
        buf.push(' ');
        buf.push_str("json ");
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.composed_messages).unwrap();
        }
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

impl ApiUpdateChatItem {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(chat_ref: ChatRef, chat_item_id: i64, updated_message: UpdatedMessage) -> Self {
        Self {
            chat_ref,
            chat_item_id,
            live_message: false,
            updated_message,
        }
    }
}

impl CommandSyntax for ApiUpdateChatItem {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_update ");
        buf.push_str("item ");
        self.chat_ref.append_command_syntax(buf);
        buf.push(' ');
        write!(buf, "{}", self.chat_item_id).unwrap();
        if self.live_message {
            buf.push(' ');
            buf.push_str("live=");
            buf.push_str("on");
        }
        buf.push(' ');
        buf.push_str("json ");
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.updated_message).unwrap();
        }
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
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_delete ");
        buf.push_str("item ");
        self.chat_ref.append_command_syntax(buf);
        buf.push(' ');
        let mut iter = self.chat_item_ids.iter();
        if let Some(el) = iter.next() {
            write!(buf, "{el}").unwrap();
        }
        for el in iter {
            buf.push(',');
            write!(buf, "{el}").unwrap();
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
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_delete ");
        buf.push_str("member ");
        buf.push_str("item ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        let mut iter = self.chat_item_ids.iter();
        if let Some(el) = iter.next() {
            write!(buf, "{el}").unwrap();
        }
        for el in iter {
            buf.push(',');
            write!(buf, "{el}").unwrap();
        }
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

impl ApiChatItemReaction {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(chat_ref: ChatRef, chat_item_id: i64, reaction: MsgReaction) -> Self {
        Self {
            chat_ref,
            chat_item_id,
            add: false,
            reaction,
        }
    }
}

impl CommandSyntax for ApiChatItemReaction {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_reaction ");
        self.chat_ref.append_command_syntax(buf);
        buf.push(' ');
        write!(buf, "{}", self.chat_item_id).unwrap();
        buf.push(' ');
        if self.add {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.reaction).unwrap();
        }
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

impl ReceiveFile {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(file_id: i64) -> Self {
        Self {
            file_id,
            user_approved_relays: false,
            store_encrypted: None,
            file_inline: None,
            file_path: None,
        }
    }
}

impl CommandSyntax for ReceiveFile {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/freceive ");
        write!(buf, "{}", self.file_id).unwrap();
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
            write!(buf, "{}", file_path).unwrap();
        }
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/fcancel ");
        write!(buf, "{}", self.file_id).unwrap();
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
/// /_add #<groupId> <contactId> relay|observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiAddMember {
    pub group_id: i64,
    pub contact_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiAddMember {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_add ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        write!(buf, "{}", self.contact_id).unwrap();
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Relay => {
                buf.push_str("relay");
            }
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_join ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
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
/// /_accept member #<groupId> <groupMemberId> relay|observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiAcceptMember {
    pub group_id: i64,
    pub group_member_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiAcceptMember {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_accept ");
        buf.push_str("member ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        write!(buf, "{}", self.group_member_id).unwrap();
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Relay => {
                buf.push_str("relay");
            }
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
/// /_member role #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...] relay|observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiMembersRole {
    pub group_id: i64,
    pub group_member_ids: Vec<i64>,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiMembersRole {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_member ");
        buf.push_str("role ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        let mut iter = self.group_member_ids.iter();
        if let Some(el) = iter.next() {
            write!(buf, "{el}").unwrap();
        }
        for el in iter {
            buf.push(',');
            write!(buf, "{el}").unwrap();
        }
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Relay => {
                buf.push_str("relay");
            }
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

impl ApiBlockMembersForAll {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(group_id: i64, group_member_ids: Vec<i64>) -> Self {
        Self {
            group_id,
            group_member_ids,
            blocked: false,
        }
    }
}

impl CommandSyntax for ApiBlockMembersForAll {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_block ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        let mut iter = self.group_member_ids.iter();
        if let Some(el) = iter.next() {
            write!(buf, "{el}").unwrap();
        }
        for el in iter {
            buf.push(',');
            write!(buf, "{el}").unwrap();
        }
        buf.push(' ');
        buf.push_str("blocked=");
        if self.blocked {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
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

impl ApiRemoveMembers {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(group_id: i64, group_member_ids: Vec<i64>) -> Self {
        Self {
            group_id,
            group_member_ids,
            with_messages: false,
        }
    }
}

impl CommandSyntax for ApiRemoveMembers {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_remove ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        let mut iter = self.group_member_ids.iter();
        if let Some(el) = iter.next() {
            write!(buf, "{el}").unwrap();
        }
        for el in iter {
            buf.push(',');
            write!(buf, "{el}").unwrap();
        }
        if self.with_messages {
            buf.push(' ');
            buf.push_str("messages=");
            buf.push_str("on");
        }
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_leave ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_members ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
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

impl ApiNewGroup {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64, group_profile: GroupProfile) -> Self {
        Self {
            user_id,
            incognito: false,
            group_profile,
        }
    }
}

impl CommandSyntax for ApiNewGroup {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_group ");
        write!(buf, "{}", self.user_id).unwrap();
        if self.incognito {
            buf.push(' ');
            buf.push_str("incognito=");
            buf.push_str("on");
        }
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.group_profile).unwrap();
        }
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Create public group.
///
/// *Network usage*: interactive.
///
/// *Syntax:*
///
/// ```
/// /_public group <userId>[ incognito=on] <relayIds[0]>[,<relayIds[1]>...] <json(groupProfile)>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiNewPublicGroup {
    pub user_id: i64,
    pub incognito: bool,
    pub relay_ids: Vec<i64>,
    pub group_profile: GroupProfile,
}

impl ApiNewPublicGroup {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64, relay_ids: Vec<i64>, group_profile: GroupProfile) -> Self {
        Self {
            user_id,
            incognito: false,
            relay_ids,
            group_profile,
        }
    }
}

impl CommandSyntax for ApiNewPublicGroup {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_public ");
        buf.push_str("group ");
        write!(buf, "{}", self.user_id).unwrap();
        if self.incognito {
            buf.push(' ');
            buf.push_str("incognito=");
            buf.push_str("on");
        }
        buf.push(' ');
        let mut iter = self.relay_ids.iter();
        if let Some(el) = iter.next() {
            write!(buf, "{el}").unwrap();
        }
        for el in iter {
            buf.push(',');
            write!(buf, "{el}").unwrap();
        }
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.group_profile).unwrap();
        }
    }
}

/// ### Group commands
///
/// Commands to manage and moderate groups. These commands can be used with business chats as well - they are groups. E.g., a common scenario would be to add human agents to business chat with the customer who connected via business address.
///
/// ----
///
/// Get group relays.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_get relays #<groupId>
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiGetGroupRelays {
    pub group_id: i64,
}

impl CommandSyntax for ApiGetGroupRelays {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_get ");
        buf.push_str("relays ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_group_profile ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.group_profile).unwrap();
        }
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
/// /_create link #<groupId> relay|observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiCreateGroupLink {
    pub group_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiCreateGroupLink {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_create ");
        buf.push_str("link ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Relay => {
                buf.push_str("relay");
            }
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
/// /_set link role #<groupId> relay|observer|author|member|moderator|admin|owner
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiGroupLinkMemberRole {
    pub group_id: i64,
    pub member_role: GroupMemberRole,
}

impl CommandSyntax for ApiGroupLinkMemberRole {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_set ");
        buf.push_str("link ");
        buf.push_str("role ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        buf.push(' ');
        match self.member_role {
            GroupMemberRole::Relay => {
                buf.push_str("relay");
            }
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_delete ");
        buf.push_str("link ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_get ");
        buf.push_str("link ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
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

impl ApiAddContact {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            incognito: false,
        }
    }
}

impl CommandSyntax for ApiAddContact {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_connect ");
        write!(buf, "{}", self.user_id).unwrap();
        if self.incognito {
            buf.push(' ');
            buf.push_str("incognito=");
            buf.push_str("on");
        }
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
    pub link_owner_sig: Option<LinkOwnerSig>,
}

impl ApiConnectPlan {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            connection_link: None,
            link_owner_sig: None,
        }
    }
}

impl CommandSyntax for ApiConnectPlan {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_connect ");
        buf.push_str("plan ");
        write!(buf, "{}", self.user_id).unwrap();
        buf.push(' ');
        write!(
            buf,
            "{}",
            self.connection_link.as_deref().unwrap_or_default()
        )
        .unwrap();
    }
}

/// ### Connection commands
///
/// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
///
/// ----
///
/// Connect via prepared SimpleX link. The link can be 1-time invitation link, contact address or group link.
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

impl ApiConnect {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            incognito: false,
            prepared_link: None,
        }
    }
}

impl CommandSyntax for ApiConnect {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_connect ");
        write!(buf, "{}", self.user_id).unwrap();
        if let Some(prepared_link) = &self.prepared_link {
            buf.push(' ');
            prepared_link.append_command_syntax(buf);
        }
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

impl Connect {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new() -> Self {
        Self {
            incognito: false,
            conn_link: None,
        }
    }
}

impl CommandSyntax for Connect {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/connect");
        if let Some(conn_link) = &self.conn_link {
            buf.push(' ');
            write!(buf, "{}", conn_link).unwrap();
        }
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_accept ");
        write!(buf, "{}", self.contact_req_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_reject ");
        write!(buf, "{}", self.contact_req_id).unwrap();
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_contacts ");
        write!(buf, "{}", self.user_id).unwrap();
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

impl ApiListGroups {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            contact_id: None,
            search: None,
        }
    }
}

impl CommandSyntax for ApiListGroups {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_groups ");
        write!(buf, "{}", self.user_id).unwrap();
        if let Some(contact_id) = &self.contact_id {
            buf.push(' ');
            buf.push('@');
            write!(buf, "{}", contact_id).unwrap();
        }
        if let Some(search) = &self.search {
            buf.push(' ');
            write!(buf, "{}", search).unwrap();
        }
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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_delete ");
        self.chat_ref.append_command_syntax(buf);
        buf.push(' ');
        self.chat_delete_mode.append_command_syntax(buf);
    }
}

/// ### Chat commands
///
/// Commands to list and delete conversations.
///
/// ----
///
/// Set group custom data.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_set custom #<groupId>[ <json(customData)>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetGroupCustomData {
    pub group_id: i64,
    pub custom_data: Option<JsonObject>,
}

impl ApiSetGroupCustomData {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            custom_data: None,
        }
    }
}

impl CommandSyntax for ApiSetGroupCustomData {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_set ");
        buf.push_str("custom ");
        buf.push('#');
        write!(buf, "{}", self.group_id).unwrap();
        if let Some(custom_data) = &self.custom_data {
            buf.push(' ');
            // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
            unsafe {
                serde_json::to_writer(buf.as_mut_vec(), &custom_data).unwrap();
            }
        }
    }
}

/// ### Chat commands
///
/// Commands to list and delete conversations.
///
/// ----
///
/// Set contact custom data.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_set custom @<contactId>[ <json(customData)>]
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetContactCustomData {
    pub contact_id: i64,
    pub custom_data: Option<JsonObject>,
}

impl ApiSetContactCustomData {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(contact_id: i64) -> Self {
        Self {
            contact_id,
            custom_data: None,
        }
    }
}

impl CommandSyntax for ApiSetContactCustomData {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_set ");
        buf.push_str("custom ");
        buf.push('@');
        write!(buf, "{}", self.contact_id).unwrap();
        if let Some(custom_data) = &self.custom_data {
            buf.push(' ');
            // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
            unsafe {
                serde_json::to_writer(buf.as_mut_vec(), &custom_data).unwrap();
            }
        }
    }
}

/// ### Chat commands
///
/// Commands to list and delete conversations.
///
/// ----
///
/// Set auto-accept member contacts.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_set accept member contacts <userId> on|off
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiSetUserAutoAcceptMemberContacts {
    pub user_id: i64,
    pub on_off: bool,
}

impl ApiSetUserAutoAcceptMemberContacts {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            on_off: false,
        }
    }
}

impl CommandSyntax for ApiSetUserAutoAcceptMemberContacts {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_set ");
        buf.push_str("accept ");
        buf.push_str("member ");
        buf.push_str("contacts ");
        write!(buf, "{}", self.user_id).unwrap();
        buf.push(' ');
        if self.on_off {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Get active user profile.
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
    const COMMAND_BUF_SIZE: usize = 0;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/user");
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Create new user profile.
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
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_create ");
        buf.push_str("user ");
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.new_user).unwrap();
        }
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Get all user profiles.
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
    const COMMAND_BUF_SIZE: usize = 0;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/users");
    }
}

/// ### User profile commands
///
/// Most bots don't need to use these commands, as bot profile can be configured manually via CLI or desktop client. These commands can be used by bots that need to manage multiple user profiles (e.g., the profiles of support agents).
///
/// ----
///
/// Set active user profile.
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

impl ApiSetActiveUser {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            view_pwd: None,
        }
    }
}

impl CommandSyntax for ApiSetActiveUser {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_user ");
        write!(buf, "{}", self.user_id).unwrap();
        if let Some(view_pwd) = &self.view_pwd {
            buf.push(' ');
            // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
            unsafe {
                serde_json::to_writer(buf.as_mut_vec(), &view_pwd).unwrap();
            }
        }
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

impl ApiDeleteUser {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            del_smp_queues: false,
            view_pwd: None,
        }
    }
}

impl CommandSyntax for ApiDeleteUser {
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_delete ");
        buf.push_str("user ");
        write!(buf, "{}", self.user_id).unwrap();
        buf.push(' ');
        buf.push_str("del_smp=");
        if self.del_smp_queues {
            buf.push_str("on");
        } else {
            buf.push_str("off");
        }
        if let Some(view_pwd) = &self.view_pwd {
            buf.push(' ');
            // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
            unsafe {
                serde_json::to_writer(buf.as_mut_vec(), &view_pwd).unwrap();
            }
        }
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
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_profile ");
        write!(buf, "{}", self.user_id).unwrap();
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.profile).unwrap();
        }
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
    const COMMAND_BUF_SIZE: usize = 1024;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_set ");
        buf.push_str("prefs ");
        buf.push('@');
        write!(buf, "{}", self.contact_id).unwrap();
        buf.push(' ');
        // SAFETY: serde_json guarantees to produce valid UTF-8 sequences
        unsafe {
            serde_json::to_writer(buf.as_mut_vec(), &self.preferences).unwrap();
        }
    }
}

/// ### Chat management
///
/// These commands should not be used with CLI-based bots
///
/// ----
///
/// Start chat controller.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_start
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct StartChat {
    pub main_app: bool,
    pub enable_snd_files: bool,
}

impl StartChat {
    /// Creates a command with all `Option` parameters set to `None` and all `bool` parameters set to false
    pub fn new() -> Self {
        Self {
            main_app: false,
            enable_snd_files: false,
        }
    }
}

impl CommandSyntax for StartChat {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_start");
    }
}

/// ### Chat management
///
/// These commands should not be used with CLI-based bots
///
/// ----
///
/// Stop chat controller.
///
/// *Network usage*: no.
///
/// *Syntax:*
///
/// ```
/// /_stop
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ApiStopChat {}

impl CommandSyntax for ApiStopChat {
    const COMMAND_BUF_SIZE: usize = 0;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("/_stop");
    }
}
