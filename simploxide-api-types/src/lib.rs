//! This crate is auto-generated

#![allow(clippy::large_enum_variant)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unnecessary_to_owned)]

pub mod client_api;
pub mod commands;
pub mod errors;
pub mod events;
pub mod responses;
pub mod utils;

use errors::*;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_number_from_string, deserialize_option_number_from_string,
};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use utils::CommandSyntax;

pub type UtcTime = String;
pub type JsonObject = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ACIReaction {
    #[serde(rename = "chatInfo")]
    pub chat_info: ChatInfo,

    #[serde(rename = "chatReaction")]
    pub chat_reaction: CIReaction,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AChat {
    #[serde(rename = "chatInfo")]
    pub chat_info: ChatInfo,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<ChatItem>,

    #[serde(rename = "chatStats")]
    pub chat_stats: ChatStats,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AChatItem {
    #[serde(rename = "chatInfo")]
    pub chat_info: ChatInfo,

    #[serde(rename = "chatItem")]
    pub chat_item: ChatItem,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AddressSettings {
    #[serde(rename = "businessAddress")]
    pub business_address: bool,

    #[serde(rename = "autoAccept", skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<AutoAccept>,

    #[serde(rename = "autoReply", skip_serializing_if = "Option::is_none")]
    pub auto_reply: Option<MsgContent>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AutoAccept {
    #[serde(rename = "acceptIncognito")]
    pub accept_incognito: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct BlockingInfo {
    #[serde(rename = "reason")]
    pub reason: BlockingReason,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BlockingReason {
    #[default]
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "content")]
    Content,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct BusinessChatInfo {
    #[serde(rename = "chatType")]
    pub chat_type: BusinessChatType,

    #[serde(rename = "businessId")]
    pub business_id: String,

    #[serde(rename = "customerId")]
    pub customer_id: String,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BusinessChatType {
    #[default]
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "customer")]
    Customer,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CICallStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "missed")]
    Missed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "negotiated")]
    Negotiated,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "ended")]
    Ended,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CIContent {
    #[serde(rename = "sndMsgContent")]
    SndMsgContent {
        #[serde(rename = "msgContent")]
        msg_content: MsgContent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvMsgContent")]
    RcvMsgContent {
        #[serde(rename = "msgContent")]
        msg_content: MsgContent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndDeleted")]
    SndDeleted {
        #[serde(rename = "deleteMode")]
        delete_mode: CIDeleteMode,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvDeleted")]
    RcvDeleted {
        #[serde(rename = "deleteMode")]
        delete_mode: CIDeleteMode,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndCall")]
    SndCall {
        #[serde(rename = "status")]
        status: CICallStatus,

        #[serde(
            rename = "duration",
            deserialize_with = "deserialize_number_from_string"
        )]
        duration: i32,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvCall")]
    RcvCall {
        #[serde(rename = "status")]
        status: CICallStatus,

        #[serde(
            rename = "duration",
            deserialize_with = "deserialize_number_from_string"
        )]
        duration: i32,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvIntegrityError")]
    RcvIntegrityError {
        #[serde(rename = "msgError")]
        msg_error: MsgErrorType,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvDecryptionError")]
    RcvDecryptionError {
        #[serde(rename = "msgDecryptError")]
        msg_decrypt_error: MsgDecryptError,

        #[serde(
            rename = "msgCount",
            deserialize_with = "deserialize_number_from_string"
        )]
        msg_count: u32,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvGroupInvitation")]
    RcvGroupInvitation {
        #[serde(rename = "groupInvitation")]
        group_invitation: CIGroupInvitation,

        #[serde(rename = "memberRole")]
        member_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndGroupInvitation")]
    SndGroupInvitation {
        #[serde(rename = "groupInvitation")]
        group_invitation: CIGroupInvitation,

        #[serde(rename = "memberRole")]
        member_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvDirectEvent")]
    RcvDirectEvent {
        #[serde(rename = "rcvDirectEvent")]
        rcv_direct_event: RcvDirectEvent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvGroupEvent")]
    RcvGroupEvent {
        #[serde(rename = "rcvGroupEvent")]
        rcv_group_event: RcvGroupEvent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndGroupEvent")]
    SndGroupEvent {
        #[serde(rename = "sndGroupEvent")]
        snd_group_event: SndGroupEvent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvConnEvent")]
    RcvConnEvent {
        #[serde(rename = "rcvConnEvent")]
        rcv_conn_event: RcvConnEvent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndConnEvent")]
    SndConnEvent {
        #[serde(rename = "sndConnEvent")]
        snd_conn_event: SndConnEvent,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvChatFeature")]
    RcvChatFeature {
        #[serde(rename = "feature")]
        feature: ChatFeature,

        #[serde(rename = "enabled")]
        enabled: PrefEnabled,

        #[serde(
            rename = "param",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        param: Option<i32>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndChatFeature")]
    SndChatFeature {
        #[serde(rename = "feature")]
        feature: ChatFeature,

        #[serde(rename = "enabled")]
        enabled: PrefEnabled,

        #[serde(
            rename = "param",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        param: Option<i32>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvChatPreference")]
    RcvChatPreference {
        #[serde(rename = "feature")]
        feature: ChatFeature,

        #[serde(rename = "allowed")]
        allowed: FeatureAllowed,

        #[serde(
            rename = "param",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        param: Option<i32>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndChatPreference")]
    SndChatPreference {
        #[serde(rename = "feature")]
        feature: ChatFeature,

        #[serde(rename = "allowed")]
        allowed: FeatureAllowed,

        #[serde(
            rename = "param",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        param: Option<i32>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvGroupFeature")]
    RcvGroupFeature {
        #[serde(rename = "groupFeature")]
        group_feature: GroupFeature,

        #[serde(rename = "preference")]
        preference: GroupPreference,

        #[serde(
            rename = "param",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        param: Option<i32>,

        #[serde(rename = "memberRole_", skip_serializing_if = "Option::is_none")]
        member_role: Option<GroupMemberRole>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndGroupFeature")]
    SndGroupFeature {
        #[serde(rename = "groupFeature")]
        group_feature: GroupFeature,

        #[serde(rename = "preference")]
        preference: GroupPreference,

        #[serde(
            rename = "param",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        param: Option<i32>,

        #[serde(rename = "memberRole_", skip_serializing_if = "Option::is_none")]
        member_role: Option<GroupMemberRole>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvChatFeatureRejected")]
    RcvChatFeatureRejected {
        #[serde(rename = "feature")]
        feature: ChatFeature,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvGroupFeatureRejected")]
    RcvGroupFeatureRejected {
        #[serde(rename = "groupFeature")]
        group_feature: GroupFeature,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndModerated")]
    SndModerated,
    #[serde(rename = "rcvModerated")]
    RcvModerated,
    #[serde(rename = "rcvBlocked")]
    RcvBlocked,
    #[serde(rename = "sndDirectE2EEInfo")]
    SndDirectE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvDirectE2EEInfo")]
    RcvDirectE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndGroupE2EEInfo")]
    SndGroupE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvGroupE2EEInfo")]
    RcvGroupE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "chatBanner")]
    ChatBanner,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CIDeleteMode {
    #[default]
    #[serde(rename = "broadcast")]
    Broadcast,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "internalMark")]
    InternalMark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CIDeleted {
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(rename = "chatType")]
        chat_type: ChatType,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "blocked")]
    Blocked {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "blockedByAdmin")]
    BlockedByAdmin {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "moderated")]
    Moderated {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(rename = "byGroupMember")]
        by_group_member: GroupMember,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CIDirection {
    #[serde(rename = "directSnd")]
    DirectSnd,
    #[serde(rename = "directRcv")]
    DirectRcv,
    #[serde(rename = "groupSnd")]
    GroupSnd,
    #[serde(rename = "groupRcv")]
    GroupRcv {
        #[serde(rename = "groupMember")]
        group_member: GroupMember,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "localSnd")]
    LocalSnd,
    #[serde(rename = "localRcv")]
    LocalRcv,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIFile {
    #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
    pub file_id: i64,

    #[serde(rename = "fileName")]
    pub file_name: String,

    #[serde(
        rename = "fileSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_size: i64,

    #[serde(rename = "fileSource", skip_serializing_if = "Option::is_none")]
    pub file_source: Option<CryptoFile>,

    #[serde(rename = "fileStatus")]
    pub file_status: CIFileStatus,

    #[serde(rename = "fileProtocol")]
    pub file_protocol: FileProtocol,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CIFileStatus {
    #[serde(rename = "sndStored")]
    SndStored,
    #[serde(rename = "sndTransfer")]
    SndTransfer {
        #[serde(
            rename = "sndProgress",
            deserialize_with = "deserialize_number_from_string"
        )]
        snd_progress: i64,

        #[serde(
            rename = "sndTotal",
            deserialize_with = "deserialize_number_from_string"
        )]
        snd_total: i64,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndCancelled")]
    SndCancelled,
    #[serde(rename = "sndComplete")]
    SndComplete,
    #[serde(rename = "sndError")]
    SndError {
        #[serde(rename = "sndFileError")]
        snd_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndWarning")]
    SndWarning {
        #[serde(rename = "sndFileError")]
        snd_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvInvitation")]
    RcvInvitation,
    #[serde(rename = "rcvAccepted")]
    RcvAccepted,
    #[serde(rename = "rcvTransfer")]
    RcvTransfer {
        #[serde(
            rename = "rcvProgress",
            deserialize_with = "deserialize_number_from_string"
        )]
        rcv_progress: i64,

        #[serde(
            rename = "rcvTotal",
            deserialize_with = "deserialize_number_from_string"
        )]
        rcv_total: i64,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvAborted")]
    RcvAborted,
    #[serde(rename = "rcvComplete")]
    RcvComplete,
    #[serde(rename = "rcvCancelled")]
    RcvCancelled,
    #[serde(rename = "rcvError")]
    RcvError {
        #[serde(rename = "rcvFileError")]
        rcv_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvWarning")]
    RcvWarning {
        #[serde(rename = "rcvFileError")]
        rcv_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "invalid")]
    Invalid {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CIForwardedFrom {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "contact")]
    Contact {
        #[serde(rename = "chatName")]
        chat_name: String,

        #[serde(rename = "msgDir")]
        msg_dir: MsgDirection,

        #[serde(
            rename = "contactId",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        contact_id: Option<i64>,

        #[serde(
            rename = "chatItemId",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        chat_item_id: Option<i64>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "chatName")]
        chat_name: String,

        #[serde(rename = "msgDir")]
        msg_dir: MsgDirection,

        #[serde(
            rename = "groupId",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        group_id: Option<i64>,

        #[serde(
            rename = "chatItemId",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        chat_item_id: Option<i64>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIGroupInvitation {
    #[serde(
        rename = "groupId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_id: i64,

    #[serde(
        rename = "groupMemberId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_member_id: i64,

    #[serde(rename = "localDisplayName")]
    pub local_display_name: String,

    #[serde(rename = "groupProfile")]
    pub group_profile: GroupProfile,

    #[serde(rename = "status")]
    pub status: CIGroupInvitationStatus,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CIGroupInvitationStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIMention {
    #[serde(rename = "memberId")]
    pub member_id: String,

    #[serde(rename = "memberRef", skip_serializing_if = "Option::is_none")]
    pub member_ref: Option<CIMentionMember>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIMentionMember {
    #[serde(
        rename = "groupMemberId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_member_id: i64,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "localAlias", skip_serializing_if = "Option::is_none")]
    pub local_alias: Option<String>,

    #[serde(rename = "memberRole")]
    pub member_role: GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIMeta {
    #[serde(rename = "itemId", deserialize_with = "deserialize_number_from_string")]
    pub item_id: i64,

    #[serde(rename = "itemTs")]
    pub item_ts: UtcTime,

    #[serde(rename = "itemText")]
    pub item_text: String,

    #[serde(rename = "itemStatus")]
    pub item_status: CIStatus,

    #[serde(rename = "sentViaProxy", skip_serializing_if = "Option::is_none")]
    pub sent_via_proxy: Option<bool>,

    #[serde(rename = "itemSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub item_shared_msg_id: Option<String>,

    #[serde(rename = "itemForwarded", skip_serializing_if = "Option::is_none")]
    pub item_forwarded: Option<CIForwardedFrom>,

    #[serde(rename = "itemDeleted", skip_serializing_if = "Option::is_none")]
    pub item_deleted: Option<CIDeleted>,

    #[serde(rename = "itemEdited")]
    pub item_edited: bool,

    #[serde(rename = "itemTimed", skip_serializing_if = "Option::is_none")]
    pub item_timed: Option<CITimed>,

    #[serde(rename = "itemLive", skip_serializing_if = "Option::is_none")]
    pub item_live: Option<bool>,

    #[serde(rename = "userMention")]
    pub user_mention: bool,

    #[serde(rename = "deletable")]
    pub deletable: bool,

    #[serde(rename = "editable")]
    pub editable: bool,

    #[serde(
        rename = "forwardedByMember",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub forwarded_by_member: Option<i64>,

    #[serde(rename = "showGroupAsSender")]
    pub show_group_as_sender: bool,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIQuote {
    #[serde(rename = "chatDir", skip_serializing_if = "Option::is_none")]
    pub chat_dir: Option<CIDirection>,

    #[serde(
        rename = "itemId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub item_id: Option<i64>,

    #[serde(rename = "sharedMsgId", skip_serializing_if = "Option::is_none")]
    pub shared_msg_id: Option<String>,

    #[serde(rename = "sentAt")]
    pub sent_at: UtcTime,

    #[serde(rename = "content")]
    pub content: MsgContent,

    #[serde(rename = "formattedText", skip_serializing_if = "Option::is_none")]
    pub formatted_text: Option<Vec<FormattedText>>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIReaction {
    #[serde(rename = "chatDir")]
    pub chat_dir: CIDirection,

    #[serde(rename = "chatItem")]
    pub chat_item: ChatItem,

    #[serde(rename = "sentAt")]
    pub sent_at: UtcTime,

    #[serde(rename = "reaction")]
    pub reaction: MsgReaction,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIReactionCount {
    #[serde(rename = "reaction")]
    pub reaction: MsgReaction,

    #[serde(rename = "userReacted")]
    pub user_reacted: bool,

    #[serde(
        rename = "totalReacted",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub total_reacted: i32,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CIStatus {
    #[serde(rename = "sndNew")]
    SndNew,
    #[serde(rename = "sndSent")]
    SndSent {
        #[serde(rename = "sndProgress")]
        snd_progress: SndCIStatusProgress,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndRcvd")]
    SndRcvd {
        #[serde(rename = "msgRcptStatus")]
        msg_rcpt_status: MsgReceiptStatus,

        #[serde(rename = "sndProgress")]
        snd_progress: SndCIStatusProgress,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndErrorAuth")]
    SndErrorAuth,
    #[serde(rename = "sndError")]
    SndError {
        #[serde(rename = "agentError")]
        agent_error: SndError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndWarning")]
    SndWarning {
        #[serde(rename = "agentError")]
        agent_error: SndError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvNew")]
    RcvNew,
    #[serde(rename = "rcvRead")]
    RcvRead,
    #[serde(rename = "invalid")]
    Invalid {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CITimed {
    #[serde(rename = "ttl", deserialize_with = "deserialize_number_from_string")]
    pub ttl: i32,

    #[serde(rename = "deleteAt", skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<UtcTime>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatBotCommand {
    #[serde(rename = "command")]
    Command {
        #[serde(rename = "keyword")]
        keyword: String,

        #[serde(rename = "label")]
        label: String,

        #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
        params: Option<String>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "menu")]
    Menu {
        #[serde(rename = "label")]
        label: String,

        #[serde(rename = "commands")]
        commands: Vec<ChatBotCommand>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

/// *Syntax:*
///
/// ```
/// full|entity|messages[ notify=off]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatDeleteMode {
    #[serde(rename = "full")]
    Full {
        #[serde(rename = "notify")]
        notify: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "entity")]
    Entity {
        #[serde(rename = "notify")]
        notify: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "messages")]
    Messages,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

impl CommandSyntax for ChatDeleteMode {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        match self {
            Self::Full { notify, .. } => {
                buf.push_str("full");
                if !notify {
                    buf.push_str(" notify=off");
                }
            }
            Self::Entity { notify, .. } => {
                buf.push_str("entity");
                if !notify {
                    buf.push_str(" notify=off");
                }
            }
            Self::Messages | Self::Undocumented(_) => {}
        }
        buf
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChatFeature {
    #[default]
    #[serde(rename = "timedMessages")]
    TimedMessages,
    #[serde(rename = "fullDelete")]
    FullDelete,
    #[serde(rename = "reactions")]
    Reactions,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "calls")]
    Calls,
    #[serde(rename = "sessions")]
    Sessions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatInfo {
    #[serde(rename = "direct")]
    Direct {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "groupChatScope", skip_serializing_if = "Option::is_none")]
        group_chat_scope: Option<GroupChatScopeInfo>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "local")]
    Local {
        #[serde(rename = "noteFolder")]
        note_folder: NoteFolder,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "contactRequest")]
    ContactRequest {
        #[serde(rename = "contactRequest")]
        contact_request: UserContactRequest,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "contactConnection")]
    ContactConnection {
        #[serde(rename = "contactConnection")]
        contact_connection: PendingContactConnection,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItem {
    #[serde(rename = "chatDir")]
    pub chat_dir: CIDirection,

    #[serde(rename = "meta")]
    pub meta: CIMeta,

    #[serde(rename = "content")]
    pub content: CIContent,

    #[serde(rename = "mentions")]
    pub mentions: HashMap<String, CIMention>,

    #[serde(rename = "formattedText", skip_serializing_if = "Option::is_none")]
    pub formatted_text: Option<Vec<FormattedText>>,

    #[serde(rename = "quotedItem", skip_serializing_if = "Option::is_none")]
    pub quoted_item: Option<CIQuote>,

    #[serde(rename = "reactions")]
    pub reactions: Vec<CIReactionCount>,

    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<CIFile>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

/// Message deletion result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemDeletion {
    #[serde(rename = "deletedChatItem")]
    pub deleted_chat_item: AChatItem,

    #[serde(rename = "toChatItem", skip_serializing_if = "Option::is_none")]
    pub to_chat_item: Option<AChatItem>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChatPeerType {
    #[default]
    #[serde(rename = "human")]
    Human,
    #[serde(rename = "bot")]
    Bot,
}

/// Used in API commands. Chat scope can only be passed with groups.
///
/// *Syntax:*
///
/// ```
/// <str(chatType)><chatId>[<str(chatScope)>]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatRef {
    #[serde(rename = "chatType")]
    pub chat_type: ChatType,

    #[serde(rename = "chatId", deserialize_with = "deserialize_number_from_string")]
    pub chat_id: i64,

    #[serde(rename = "chatScope", skip_serializing_if = "Option::is_none")]
    pub chat_scope: Option<GroupChatScope>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

impl CommandSyntax for ChatRef {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(256);
        buf.push_str(&self.chat_type.interpret());
        buf.push_str(&self.chat_id.to_string());
        if let Some(chat_scope) = &self.chat_scope {
            buf.push_str(&chat_scope.interpret());
        }
        buf
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatSettings {
    #[serde(rename = "enableNtfs")]
    pub enable_ntfs: MsgFilter,

    #[serde(rename = "sendRcpts", skip_serializing_if = "Option::is_none")]
    pub send_rcpts: Option<bool>,

    #[serde(rename = "favorite")]
    pub favorite: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatStats {
    #[serde(
        rename = "unreadCount",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub unread_count: i32,

    #[serde(
        rename = "unreadMentions",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub unread_mentions: i32,

    #[serde(
        rename = "reportsCount",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub reports_count: i32,

    #[serde(
        rename = "minUnreadItemId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub min_unread_item_id: i64,

    #[serde(rename = "unreadChat")]
    pub unread_chat: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

/// *Syntax:*
///
/// ```
/// @|#|*|
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChatType {
    #[default]
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "local")]
    Local,
}

impl CommandSyntax for ChatType {
    fn interpret(&self) -> String {
        let mut buf = String::new();
        match self {
            Self::Direct => {
                buf.push('@');
            }
            Self::Group => {
                buf.push('#');
            }
            Self::Local => {
                buf.push('*');
            }
        }
        buf
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatWallpaper {
    #[serde(rename = "preset", skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,

    #[serde(rename = "imageFile", skip_serializing_if = "Option::is_none")]
    pub image_file: Option<String>,

    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,

    #[serde(rename = "tint", skip_serializing_if = "Option::is_none")]
    pub tint: Option<String>,

    #[serde(rename = "scaleType", skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ChatWallpaperScale>,

    #[serde(
        rename = "scale",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub scale: Option<f64>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChatWallpaperScale {
    #[default]
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "repeat")]
    Repeat,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Color {
    #[default]
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "magenta")]
    Magenta,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "white")]
    White,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ComposedMessage {
    #[serde(rename = "fileSource", skip_serializing_if = "Option::is_none")]
    pub file_source: Option<CryptoFile>,

    #[serde(
        rename = "quotedItemId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub quoted_item_id: Option<i64>,

    #[serde(rename = "msgContent")]
    pub msg_content: MsgContent,

    #[serde(rename = "mentions")]
    pub mentions: HashMap<String, i64>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnStatus {
    #[default]
    #[serde(rename = "new")]
    New,
    #[serde(rename = "prepared")]
    Prepared,
    #[serde(rename = "joined")]
    Joined,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "snd-ready")]
    SndReady,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "deleted")]
    Deleted,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnType {
    #[default]
    #[serde(rename = "contact")]
    Contact,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "user_contact")]
    UserContact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct Connection {
    #[serde(rename = "connId", deserialize_with = "deserialize_number_from_string")]
    pub conn_id: i64,

    #[serde(rename = "agentConnId")]
    pub agent_conn_id: String,

    #[serde(
        rename = "connChatVersion",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub conn_chat_version: i32,

    #[serde(rename = "peerChatVRange")]
    pub peer_chat_v_range: VersionRange,

    #[serde(
        rename = "connLevel",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub conn_level: i32,

    #[serde(
        rename = "viaContact",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub via_contact: Option<i64>,

    #[serde(
        rename = "viaUserContactLink",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub via_user_contact_link: Option<i64>,

    #[serde(rename = "viaGroupLink")]
    pub via_group_link: bool,

    #[serde(rename = "groupLinkId", skip_serializing_if = "Option::is_none")]
    pub group_link_id: Option<String>,

    #[serde(rename = "xContactId", skip_serializing_if = "Option::is_none")]
    pub x_contact_id: Option<String>,

    #[serde(
        rename = "customUserProfileId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub custom_user_profile_id: Option<i64>,

    #[serde(rename = "connType")]
    pub conn_type: ConnType,

    #[serde(rename = "connStatus")]
    pub conn_status: ConnStatus,

    #[serde(rename = "contactConnInitiated")]
    pub contact_conn_initiated: bool,

    #[serde(rename = "localAlias")]
    pub local_alias: String,

    #[serde(
        rename = "entityId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub entity_id: Option<i64>,

    #[serde(rename = "connectionCode", skip_serializing_if = "Option::is_none")]
    pub connection_code: Option<SecurityCode>,

    #[serde(rename = "pqSupport")]
    pub pq_support: bool,

    #[serde(rename = "pqEncryption")]
    pub pq_encryption: bool,

    #[serde(rename = "pqSndEnabled", skip_serializing_if = "Option::is_none")]
    pub pq_snd_enabled: Option<bool>,

    #[serde(rename = "pqRcvEnabled", skip_serializing_if = "Option::is_none")]
    pub pq_rcv_enabled: Option<bool>,

    #[serde(
        rename = "authErrCounter",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub auth_err_counter: i32,

    #[serde(
        rename = "quotaErrCounter",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub quota_err_counter: i32,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnectionEntity {
    #[serde(rename = "rcvDirectMsgConnection")]
    RcvDirectMsgConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
        contact: Option<Contact>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvGroupMsgConnection")]
    RcvGroupMsgConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "groupMember")]
        group_member: GroupMember,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "sndFileConnection")]
    SndFileConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "sndFileTransfer")]
        snd_file_transfer: SndFileTransfer,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "rcvFileConnection")]
    RcvFileConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "rcvFileTransfer")]
        rcv_file_transfer: RcvFileTransfer,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userContactConnection")]
    UserContactConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "userContact")]
        user_contact: UserContact,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnectionMode {
    #[default]
    #[serde(rename = "inv")]
    Inv,
    #[serde(rename = "con")]
    Con,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnectionPlan {
    #[serde(rename = "invitationLink")]
    InvitationLink {
        #[serde(rename = "invitationLinkPlan")]
        invitation_link_plan: InvitationLinkPlan,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "contactAddress")]
    ContactAddress {
        #[serde(rename = "contactAddressPlan")]
        contact_address_plan: ContactAddressPlan,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "groupLink")]
    GroupLink {
        #[serde(rename = "groupLinkPlan")]
        group_link_plan: GroupLinkPlan,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "chatError")]
        chat_error: ChatError,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct Contact {
    #[serde(
        rename = "contactId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub contact_id: i64,

    #[serde(rename = "localDisplayName")]
    pub local_display_name: String,

    #[serde(rename = "profile")]
    pub profile: LocalProfile,

    #[serde(rename = "activeConn", skip_serializing_if = "Option::is_none")]
    pub active_conn: Option<Connection>,

    #[serde(
        rename = "viaGroup",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub via_group: Option<i64>,

    #[serde(rename = "contactUsed")]
    pub contact_used: bool,

    #[serde(rename = "contactStatus")]
    pub contact_status: ContactStatus,

    #[serde(rename = "chatSettings")]
    pub chat_settings: ChatSettings,

    #[serde(rename = "userPreferences")]
    pub user_preferences: Preferences,

    #[serde(rename = "mergedPreferences")]
    pub merged_preferences: ContactUserPreferences,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(rename = "chatTs", skip_serializing_if = "Option::is_none")]
    pub chat_ts: Option<UtcTime>,

    #[serde(rename = "preparedContact", skip_serializing_if = "Option::is_none")]
    pub prepared_contact: Option<PreparedContact>,

    #[serde(
        rename = "contactRequestId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub contact_request_id: Option<i64>,

    #[serde(
        rename = "contactGroupMemberId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub contact_group_member_id: Option<i64>,

    #[serde(rename = "contactGrpInvSent")]
    pub contact_grp_inv_sent: bool,

    #[serde(rename = "groupDirectInv", skip_serializing_if = "Option::is_none")]
    pub group_direct_inv: Option<GroupDirectInvitation>,

    #[serde(rename = "chatTags")]
    pub chat_tags: Vec<i64>,

    #[serde(
        rename = "chatItemTTL",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub chat_item_ttl: Option<i64>,

    #[serde(rename = "uiThemes", skip_serializing_if = "Option::is_none")]
    pub ui_themes: Option<UIThemeEntityOverrides>,

    #[serde(rename = "chatDeleted")]
    pub chat_deleted: bool,

    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<JsonObject>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContactAddressPlan {
    #[serde(rename = "ok")]
    Ok {
        #[serde(rename = "contactSLinkData_", skip_serializing_if = "Option::is_none")]
        contact_s_link_data: Option<ContactShortLinkData>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "ownLink")]
    OwnLink,
    #[serde(rename = "connectingConfirmReconnect")]
    ConnectingConfirmReconnect,
    #[serde(rename = "connectingProhibit")]
    ConnectingProhibit {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "known")]
    Known {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "contactViaAddress")]
    ContactViaAddress {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactShortLinkData {
    #[serde(rename = "profile")]
    pub profile: Profile,

    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<MsgContent>,

    #[serde(rename = "business")]
    pub business: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContactStatus {
    #[default]
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "deletedByUser")]
    DeletedByUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContactUserPref {
    #[serde(rename = "contact")]
    Contact {
        #[serde(rename = "preference")]
        preference: SimplePreference,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "user")]
    User {
        #[serde(rename = "preference")]
        preference: SimplePreference,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactUserPreference {
    #[serde(rename = "enabled")]
    pub enabled: PrefEnabled,

    #[serde(rename = "userPreference")]
    pub user_preference: ContactUserPref,

    #[serde(rename = "contactPreference")]
    pub contact_preference: SimplePreference,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactUserPreferences {
    #[serde(rename = "timedMessages")]
    pub timed_messages: ContactUserPreference,

    #[serde(rename = "fullDelete")]
    pub full_delete: ContactUserPreference,

    #[serde(rename = "reactions")]
    pub reactions: ContactUserPreference,

    #[serde(rename = "voice")]
    pub voice: ContactUserPreference,

    #[serde(rename = "files")]
    pub files: ContactUserPreference,

    #[serde(rename = "calls")]
    pub calls: ContactUserPreference,

    #[serde(rename = "sessions")]
    pub sessions: ContactUserPreference,

    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<ChatBotCommand>>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

/// *Syntax:*
///
/// ```
/// <connFullLink>[ <connShortLink>]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CreatedConnLink {
    #[serde(rename = "connFullLink")]
    pub conn_full_link: String,

    #[serde(rename = "connShortLink", skip_serializing_if = "Option::is_none")]
    pub conn_short_link: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

impl CommandSyntax for CreatedConnLink {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str(&self.conn_full_link.to_string());
        if let Some(conn_short_link) = &self.conn_short_link {
            buf.push(' ');
            buf.push_str(&conn_short_link.to_string());
        }
        buf
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CryptoFile {
    #[serde(rename = "filePath")]
    pub file_path: String,

    #[serde(rename = "cryptoArgs", skip_serializing_if = "Option::is_none")]
    pub crypto_args: Option<CryptoFileArgs>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CryptoFileArgs {
    #[serde(rename = "fileKey")]
    pub file_key: String,

    #[serde(rename = "fileNonce")]
    pub file_nonce: String,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct E2EInfo {
    #[serde(rename = "pqEnabled", skip_serializing_if = "Option::is_none")]
    pub pq_enabled: Option<bool>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FeatureAllowed {
    #[default]
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FileDescr {
    #[serde(rename = "fileDescrText")]
    pub file_descr_text: String,

    #[serde(
        rename = "fileDescrPartNo",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_descr_part_no: i32,

    #[serde(rename = "fileDescrComplete")]
    pub file_descr_complete: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FileInvitation {
    #[serde(rename = "fileName")]
    pub file_name: String,

    #[serde(
        rename = "fileSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_size: i64,

    #[serde(rename = "fileDigest", skip_serializing_if = "Option::is_none")]
    pub file_digest: Option<String>,

    #[serde(rename = "fileConnReq", skip_serializing_if = "Option::is_none")]
    pub file_conn_req: Option<String>,

    #[serde(rename = "fileInline", skip_serializing_if = "Option::is_none")]
    pub file_inline: Option<InlineFileMode>,

    #[serde(rename = "fileDescr", skip_serializing_if = "Option::is_none")]
    pub file_descr: Option<FileDescr>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FileProtocol {
    #[default]
    #[serde(rename = "smp")]
    Smp,
    #[serde(rename = "xftp")]
    Xftp,
    #[serde(rename = "local")]
    Local,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FileStatus {
    #[default]
    #[serde(rename = "new")]
    New,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FileTransferMeta {
    #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
    pub file_id: i64,

    #[serde(rename = "xftpSndFile", skip_serializing_if = "Option::is_none")]
    pub xftp_snd_file: Option<XFTPSndFile>,

    #[serde(
        rename = "xftpRedirectFor",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub xftp_redirect_for: Option<i64>,

    #[serde(rename = "fileName")]
    pub file_name: String,

    #[serde(rename = "filePath")]
    pub file_path: String,

    #[serde(
        rename = "fileSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_size: i64,

    #[serde(rename = "fileInline", skip_serializing_if = "Option::is_none")]
    pub file_inline: Option<InlineFileMode>,

    #[serde(
        rename = "chunkSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub chunk_size: i64,

    #[serde(rename = "cancelled")]
    pub cancelled: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Format {
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "strikeThrough")]
    StrikeThrough,
    #[serde(rename = "snippet")]
    Snippet,
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "colored")]
    Colored {
        #[serde(rename = "color")]
        color: Color,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "hyperLink")]
    HyperLink {
        #[serde(rename = "showText", skip_serializing_if = "Option::is_none")]
        show_text: Option<String>,

        #[serde(rename = "linkUri")]
        link_uri: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "simplexLink")]
    SimplexLink {
        #[serde(rename = "showText", skip_serializing_if = "Option::is_none")]
        show_text: Option<String>,

        #[serde(rename = "linkType")]
        link_type: SimplexLinkType,

        #[serde(rename = "simplexUri")]
        simplex_uri: String,

        #[serde(rename = "smpHosts")]
        smp_hosts: Vec<String>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "command")]
    Command {
        #[serde(rename = "commandStr")]
        command_str: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "mention")]
    Mention {
        #[serde(rename = "memberName")]
        member_name: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FormattedText {
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FullGroupPreferences {
    #[serde(rename = "timedMessages")]
    pub timed_messages: TimedMessagesGroupPreference,

    #[serde(rename = "directMessages")]
    pub direct_messages: RoleGroupPreference,

    #[serde(rename = "fullDelete")]
    pub full_delete: GroupPreference,

    #[serde(rename = "reactions")]
    pub reactions: GroupPreference,

    #[serde(rename = "voice")]
    pub voice: RoleGroupPreference,

    #[serde(rename = "files")]
    pub files: RoleGroupPreference,

    #[serde(rename = "simplexLinks")]
    pub simplex_links: RoleGroupPreference,

    #[serde(rename = "reports")]
    pub reports: GroupPreference,

    #[serde(rename = "history")]
    pub history: GroupPreference,

    #[serde(rename = "sessions")]
    pub sessions: RoleGroupPreference,

    #[serde(rename = "commands")]
    pub commands: Vec<ChatBotCommand>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FullPreferences {
    #[serde(rename = "timedMessages")]
    pub timed_messages: TimedMessagesPreference,

    #[serde(rename = "fullDelete")]
    pub full_delete: SimplePreference,

    #[serde(rename = "reactions")]
    pub reactions: SimplePreference,

    #[serde(rename = "voice")]
    pub voice: SimplePreference,

    #[serde(rename = "files")]
    pub files: SimplePreference,

    #[serde(rename = "calls")]
    pub calls: SimplePreference,

    #[serde(rename = "sessions")]
    pub sessions: SimplePreference,

    #[serde(rename = "commands")]
    pub commands: Vec<ChatBotCommand>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct Group {
    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "members")]
    pub members: Vec<GroupMember>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

/// *Syntax:*
///
/// ```
/// (_support[:<groupMemberId_>])
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GroupChatScope {
    #[serde(rename = "memberSupport")]
    MemberSupport {
        #[serde(
            rename = "groupMemberId_",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        group_member_id: Option<i64>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

impl CommandSyntax for GroupChatScope {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("(_support");
        match self {
            Self::MemberSupport {
                group_member_id, ..
            } => {
                if let Some(group_member_id) = group_member_id {
                    buf.push(':');
                    buf.push_str(&group_member_id.to_string());
                }
            }
            Self::Undocumented(_) => {}
        }
        buf.push(')');
        buf
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GroupChatScopeInfo {
    #[serde(rename = "memberSupport")]
    MemberSupport {
        #[serde(rename = "groupMember_", skip_serializing_if = "Option::is_none")]
        group_member: Option<GroupMember>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupDirectInvitation {
    #[serde(rename = "groupDirectInvLink")]
    pub group_direct_inv_link: String,

    #[serde(
        rename = "fromGroupId_",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub from_group_id: Option<i64>,

    #[serde(
        rename = "fromGroupMemberId_",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub from_group_member_id: Option<i64>,

    #[serde(
        rename = "fromGroupMemberConnId_",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub from_group_member_conn_id: Option<i64>,

    #[serde(rename = "groupDirectInvStartedConnection")]
    pub group_direct_inv_started_connection: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupFeature {
    #[default]
    #[serde(rename = "timedMessages")]
    TimedMessages,
    #[serde(rename = "directMessages")]
    DirectMessages,
    #[serde(rename = "fullDelete")]
    FullDelete,
    #[serde(rename = "reactions")]
    Reactions,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "simplexLinks")]
    SimplexLinks,
    #[serde(rename = "reports")]
    Reports,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "sessions")]
    Sessions,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupFeatureEnabled {
    #[default]
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupInfo {
    #[serde(
        rename = "groupId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_id: i64,

    #[serde(rename = "localDisplayName")]
    pub local_display_name: String,

    #[serde(rename = "groupProfile")]
    pub group_profile: GroupProfile,

    #[serde(rename = "localAlias")]
    pub local_alias: String,

    #[serde(rename = "businessChat", skip_serializing_if = "Option::is_none")]
    pub business_chat: Option<BusinessChatInfo>,

    #[serde(rename = "fullGroupPreferences")]
    pub full_group_preferences: FullGroupPreferences,

    #[serde(rename = "membership")]
    pub membership: GroupMember,

    #[serde(rename = "chatSettings")]
    pub chat_settings: ChatSettings,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(rename = "chatTs", skip_serializing_if = "Option::is_none")]
    pub chat_ts: Option<UtcTime>,

    #[serde(
        rename = "userMemberProfileSentAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_member_profile_sent_at: Option<UtcTime>,

    #[serde(rename = "preparedGroup", skip_serializing_if = "Option::is_none")]
    pub prepared_group: Option<PreparedGroup>,

    #[serde(rename = "chatTags")]
    pub chat_tags: Vec<i64>,

    #[serde(
        rename = "chatItemTTL",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub chat_item_ttl: Option<i64>,

    #[serde(rename = "uiThemes", skip_serializing_if = "Option::is_none")]
    pub ui_themes: Option<UIThemeEntityOverrides>,

    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<JsonObject>,

    #[serde(
        rename = "membersRequireAttention",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub members_require_attention: i32,

    #[serde(rename = "viaGroupLinkUri", skip_serializing_if = "Option::is_none")]
    pub via_group_link_uri: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupInfoSummary {
    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "groupSummary")]
    pub group_summary: GroupSummary,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLink {
    #[serde(
        rename = "userContactLinkId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub user_contact_link_id: i64,

    #[serde(rename = "connLinkContact")]
    pub conn_link_contact: CreatedConnLink,

    #[serde(rename = "shortLinkDataSet")]
    pub short_link_data_set: bool,

    #[serde(rename = "shortLinkLargeDataSet")]
    pub short_link_large_data_set: bool,

    #[serde(rename = "groupLinkId")]
    pub group_link_id: String,

    #[serde(rename = "acceptMemberRole")]
    pub accept_member_role: GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GroupLinkPlan {
    #[serde(rename = "ok")]
    Ok {
        #[serde(rename = "groupSLinkData_", skip_serializing_if = "Option::is_none")]
        group_s_link_data: Option<GroupShortLinkData>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "ownLink")]
    OwnLink {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "connectingConfirmReconnect")]
    ConnectingConfirmReconnect,
    #[serde(rename = "connectingProhibit")]
    ConnectingProhibit {
        #[serde(rename = "groupInfo_", skip_serializing_if = "Option::is_none")]
        group_info: Option<GroupInfo>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "known")]
    Known {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMember {
    #[serde(
        rename = "groupMemberId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_member_id: i64,

    #[serde(
        rename = "groupId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_id: i64,

    #[serde(rename = "memberId")]
    pub member_id: String,

    #[serde(rename = "memberRole")]
    pub member_role: GroupMemberRole,

    #[serde(rename = "memberCategory")]
    pub member_category: GroupMemberCategory,

    #[serde(rename = "memberStatus")]
    pub member_status: GroupMemberStatus,

    #[serde(rename = "memberSettings")]
    pub member_settings: GroupMemberSettings,

    #[serde(rename = "blockedByAdmin")]
    pub blocked_by_admin: bool,

    #[serde(rename = "invitedBy")]
    pub invited_by: InvitedBy,

    #[serde(
        rename = "invitedByGroupMemberId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub invited_by_group_member_id: Option<i64>,

    #[serde(rename = "localDisplayName")]
    pub local_display_name: String,

    #[serde(rename = "memberProfile")]
    pub member_profile: LocalProfile,

    #[serde(
        rename = "memberContactId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub member_contact_id: Option<i64>,

    #[serde(
        rename = "memberContactProfileId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub member_contact_profile_id: i64,

    #[serde(rename = "activeConn", skip_serializing_if = "Option::is_none")]
    pub active_conn: Option<Connection>,

    #[serde(rename = "memberChatVRange")]
    pub member_chat_v_range: VersionRange,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(rename = "supportChat", skip_serializing_if = "Option::is_none")]
    pub support_chat: Option<GroupSupportChat>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMemberAdmission {
    #[serde(rename = "review", skip_serializing_if = "Option::is_none")]
    pub review: Option<MemberCriteria>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupMemberCategory {
    #[default]
    #[serde(rename = "user")]
    User,
    #[serde(rename = "invitee")]
    Invitee,
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "pre")]
    Pre,
    #[serde(rename = "post")]
    Post,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMemberRef {
    #[serde(
        rename = "groupMemberId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_member_id: i64,

    #[serde(rename = "profile")]
    pub profile: Profile,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupMemberRole {
    #[default]
    #[serde(rename = "observer")]
    Observer,
    #[serde(rename = "author")]
    Author,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "moderator")]
    Moderator,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "owner")]
    Owner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMemberSettings {
    #[serde(rename = "showMessages")]
    pub show_messages: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupMemberStatus {
    #[default]
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "removed")]
    Removed,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "pending_approval")]
    PendingApproval,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "introduced")]
    Introduced,
    #[serde(rename = "intro-inv")]
    IntroInv,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "announced")]
    Announced,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "creator")]
    Creator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupPreference {
    #[serde(rename = "enable")]
    pub enable: GroupFeatureEnabled,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupPreferences {
    #[serde(rename = "timedMessages", skip_serializing_if = "Option::is_none")]
    pub timed_messages: Option<TimedMessagesGroupPreference>,

    #[serde(rename = "directMessages", skip_serializing_if = "Option::is_none")]
    pub direct_messages: Option<RoleGroupPreference>,

    #[serde(rename = "fullDelete", skip_serializing_if = "Option::is_none")]
    pub full_delete: Option<GroupPreference>,

    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<GroupPreference>,

    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<RoleGroupPreference>,

    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<RoleGroupPreference>,

    #[serde(rename = "simplexLinks", skip_serializing_if = "Option::is_none")]
    pub simplex_links: Option<RoleGroupPreference>,

    #[serde(rename = "reports", skip_serializing_if = "Option::is_none")]
    pub reports: Option<GroupPreference>,

    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<GroupPreference>,

    #[serde(rename = "sessions", skip_serializing_if = "Option::is_none")]
    pub sessions: Option<RoleGroupPreference>,

    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<ChatBotCommand>>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupProfile {
    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "fullName")]
    pub full_name: String,

    #[serde(rename = "shortDescr", skip_serializing_if = "Option::is_none")]
    pub short_descr: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "groupPreferences", skip_serializing_if = "Option::is_none")]
    pub group_preferences: Option<GroupPreferences>,

    #[serde(rename = "memberAdmission", skip_serializing_if = "Option::is_none")]
    pub member_admission: Option<GroupMemberAdmission>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupShortLinkData {
    #[serde(rename = "groupProfile")]
    pub group_profile: GroupProfile,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupSummary {
    #[serde(
        rename = "currentMembers",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub current_members: i32,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupSupportChat {
    #[serde(rename = "chatTs")]
    pub chat_ts: UtcTime,

    #[serde(rename = "unread", deserialize_with = "deserialize_number_from_string")]
    pub unread: i64,

    #[serde(
        rename = "memberAttention",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub member_attention: i64,

    #[serde(
        rename = "mentions",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub mentions: i64,

    #[serde(
        rename = "lastMsgFromMemberTs",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_msg_from_member_ts: Option<UtcTime>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InlineFileMode {
    #[default]
    #[serde(rename = "offer")]
    Offer,
    #[serde(rename = "sent")]
    Sent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum InvitationLinkPlan {
    #[serde(rename = "ok")]
    Ok {
        #[serde(rename = "contactSLinkData_", skip_serializing_if = "Option::is_none")]
        contact_s_link_data: Option<ContactShortLinkData>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "ownLink")]
    OwnLink,
    #[serde(rename = "connecting")]
    Connecting {
        #[serde(rename = "contact_", skip_serializing_if = "Option::is_none")]
        contact: Option<Contact>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "known")]
    Known {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum InvitedBy {
    #[serde(rename = "contact")]
    Contact {
        #[serde(
            rename = "byContactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        by_contact_id: i64,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "user")]
    User,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum LinkContent {
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video {
        #[serde(
            rename = "duration",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        duration: Option<i32>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "unknown")]
    Unknown {
        #[serde(rename = "tag")]
        tag: String,

        #[serde(rename = "json")]
        json: JsonObject,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct LinkPreview {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<LinkContent>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct LocalProfile {
    #[serde(
        rename = "profileId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub profile_id: i64,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "fullName")]
    pub full_name: String,

    #[serde(rename = "shortDescr", skip_serializing_if = "Option::is_none")]
    pub short_descr: Option<String>,

    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "contactLink", skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,

    #[serde(rename = "preferences", skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,

    #[serde(rename = "peerType", skip_serializing_if = "Option::is_none")]
    pub peer_type: Option<ChatPeerType>,

    #[serde(rename = "localAlias")]
    pub local_alias: String,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MemberCriteria {
    #[default]
    #[serde(rename = "all")]
    All,
}

/// Connection link sent in a message - only short links are allowed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MsgChatLink {
    #[serde(rename = "contact")]
    Contact {
        #[serde(rename = "connLink")]
        conn_link: String,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(rename = "business")]
        business: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "invitation")]
    Invitation {
        #[serde(rename = "invLink")]
        inv_link: String,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "connLink")]
        conn_link: String,

        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MsgContent {
    #[serde(rename = "text")]
    Text {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "link")]
    Link {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "preview")]
        preview: LinkPreview,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "image")]
    Image {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "image")]
        image: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "video")]
    Video {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "image")]
        image: String,

        #[serde(
            rename = "duration",
            deserialize_with = "deserialize_number_from_string"
        )]
        duration: i32,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "voice")]
    Voice {
        #[serde(rename = "text")]
        text: String,

        #[serde(
            rename = "duration",
            deserialize_with = "deserialize_number_from_string"
        )]
        duration: i32,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "file")]
    File {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "report")]
    Report {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "reason")]
        reason: ReportReason,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "chat")]
    Chat {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "chatLink")]
        chat_link: MsgChatLink,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "unknown")]
    Unknown {
        #[serde(rename = "tag")]
        tag: String,

        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "json")]
        json: JsonObject,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MsgDirection {
    #[default]
    #[serde(rename = "rcv")]
    Rcv,
    #[serde(rename = "snd")]
    Snd,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MsgFilter {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "mentions")]
    Mentions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MsgReaction {
    #[serde(rename = "emoji")]
    Emoji {
        #[serde(rename = "emoji")]
        emoji: String,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "unknown")]
    Unknown {
        #[serde(rename = "tag")]
        tag: String,

        #[serde(rename = "json")]
        json: JsonObject,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MsgReceiptStatus {
    #[default]
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "badMsgHash")]
    BadMsgHash,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewUser {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,

    #[serde(rename = "pastTimestamp")]
    pub past_timestamp: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NoteFolder {
    #[serde(
        rename = "noteFolderId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub note_folder_id: i64,

    #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
    pub user_id: i64,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(rename = "chatTs")]
    pub chat_ts: UtcTime,

    #[serde(rename = "favorite")]
    pub favorite: bool,

    #[serde(rename = "unread")]
    pub unread: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PendingContactConnection {
    #[serde(
        rename = "pccConnId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub pcc_conn_id: i64,

    #[serde(rename = "pccAgentConnId")]
    pub pcc_agent_conn_id: String,

    #[serde(rename = "pccConnStatus")]
    pub pcc_conn_status: ConnStatus,

    #[serde(rename = "viaContactUri")]
    pub via_contact_uri: bool,

    #[serde(
        rename = "viaUserContactLink",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub via_user_contact_link: Option<i64>,

    #[serde(rename = "groupLinkId", skip_serializing_if = "Option::is_none")]
    pub group_link_id: Option<String>,

    #[serde(
        rename = "customUserProfileId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub custom_user_profile_id: Option<i64>,

    #[serde(rename = "connLinkInv", skip_serializing_if = "Option::is_none")]
    pub conn_link_inv: Option<CreatedConnLink>,

    #[serde(rename = "localAlias")]
    pub local_alias: String,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PrefEnabled {
    #[serde(rename = "forUser")]
    pub for_user: bool,

    #[serde(rename = "forContact")]
    pub for_contact: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct Preferences {
    #[serde(rename = "timedMessages", skip_serializing_if = "Option::is_none")]
    pub timed_messages: Option<TimedMessagesPreference>,

    #[serde(rename = "fullDelete", skip_serializing_if = "Option::is_none")]
    pub full_delete: Option<SimplePreference>,

    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<SimplePreference>,

    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<SimplePreference>,

    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<SimplePreference>,

    #[serde(rename = "calls", skip_serializing_if = "Option::is_none")]
    pub calls: Option<SimplePreference>,

    #[serde(rename = "sessions", skip_serializing_if = "Option::is_none")]
    pub sessions: Option<SimplePreference>,

    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<ChatBotCommand>>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PreparedContact {
    #[serde(rename = "connLinkToConnect")]
    pub conn_link_to_connect: CreatedConnLink,

    #[serde(rename = "uiConnLinkType")]
    pub ui_conn_link_type: ConnectionMode,

    #[serde(rename = "welcomeSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub welcome_shared_msg_id: Option<String>,

    #[serde(rename = "requestSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub request_shared_msg_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PreparedGroup {
    #[serde(rename = "connLinkToConnect")]
    pub conn_link_to_connect: CreatedConnLink,

    #[serde(rename = "connLinkPreparedConnection")]
    pub conn_link_prepared_connection: bool,

    #[serde(rename = "connLinkStartedConnection")]
    pub conn_link_started_connection: bool,

    #[serde(rename = "welcomeSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub welcome_shared_msg_id: Option<String>,

    #[serde(rename = "requestSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub request_shared_msg_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct Profile {
    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "fullName")]
    pub full_name: String,

    #[serde(rename = "shortDescr", skip_serializing_if = "Option::is_none")]
    pub short_descr: Option<String>,

    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "contactLink", skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,

    #[serde(rename = "preferences", skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,

    #[serde(rename = "peerType", skip_serializing_if = "Option::is_none")]
    pub peer_type: Option<ChatPeerType>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RatchetSyncState {
    #[default]
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "agreed")]
    Agreed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RcvConnEvent {
    #[serde(rename = "switchQueue")]
    SwitchQueue {
        #[serde(rename = "phase")]
        phase: SwitchPhase,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "ratchetSync")]
    RatchetSync {
        #[serde(rename = "syncStatus")]
        sync_status: RatchetSyncState,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "verificationCodeReset")]
    VerificationCodeReset,
    #[serde(rename = "pqEnabled")]
    PqEnabled {
        #[serde(rename = "enabled")]
        enabled: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RcvDirectEvent {
    #[serde(rename = "contactDeleted")]
    ContactDeleted,
    #[serde(rename = "profileUpdated")]
    ProfileUpdated {
        #[serde(rename = "fromProfile")]
        from_profile: Profile,

        #[serde(rename = "toProfile")]
        to_profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "groupInvLinkReceived")]
    GroupInvLinkReceived {
        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileDescr {
    #[serde(
        rename = "fileDescrId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_descr_id: i64,

    #[serde(rename = "fileDescrText")]
    pub file_descr_text: String,

    #[serde(
        rename = "fileDescrPartNo",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_descr_part_no: i32,

    #[serde(rename = "fileDescrComplete")]
    pub file_descr_complete: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileInfo {
    #[serde(rename = "filePath")]
    pub file_path: String,

    #[serde(
        rename = "connId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub conn_id: Option<i64>,

    #[serde(rename = "agentConnId", skip_serializing_if = "Option::is_none")]
    pub agent_conn_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RcvFileStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "accepted")]
    Accepted {
        #[serde(rename = "fileInfo")]
        file_info: RcvFileInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "connected")]
    Connected {
        #[serde(rename = "fileInfo")]
        file_info: RcvFileInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "complete")]
    Complete {
        #[serde(rename = "fileInfo")]
        file_info: RcvFileInfo,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "cancelled")]
    Cancelled {
        #[serde(rename = "fileInfo_", skip_serializing_if = "Option::is_none")]
        file_info: Option<RcvFileInfo>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileTransfer {
    #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
    pub file_id: i64,

    #[serde(rename = "xftpRcvFile", skip_serializing_if = "Option::is_none")]
    pub xftp_rcv_file: Option<XFTPRcvFile>,

    #[serde(rename = "fileInvitation")]
    pub file_invitation: FileInvitation,

    #[serde(rename = "fileStatus")]
    pub file_status: RcvFileStatus,

    #[serde(rename = "rcvFileInline", skip_serializing_if = "Option::is_none")]
    pub rcv_file_inline: Option<InlineFileMode>,

    #[serde(rename = "senderDisplayName")]
    pub sender_display_name: String,

    #[serde(
        rename = "chunkSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub chunk_size: i64,

    #[serde(rename = "cancelled")]
    pub cancelled: bool,

    #[serde(
        rename = "grpMemberId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub grp_member_id: Option<i64>,

    #[serde(rename = "cryptoArgs", skip_serializing_if = "Option::is_none")]
    pub crypto_args: Option<CryptoFileArgs>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RcvGroupEvent {
    #[serde(rename = "memberAdded")]
    MemberAdded {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "memberConnected")]
    MemberConnected,
    #[serde(rename = "memberAccepted")]
    MemberAccepted {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userAccepted")]
    UserAccepted,
    #[serde(rename = "memberLeft")]
    MemberLeft,
    #[serde(rename = "memberRole")]
    MemberRole {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(rename = "role")]
        role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "memberBlocked")]
    MemberBlocked {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(rename = "blocked")]
        blocked: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userRole")]
    UserRole {
        #[serde(rename = "role")]
        role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "memberDeleted")]
    MemberDeleted {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userDeleted")]
    UserDeleted,
    #[serde(rename = "groupDeleted")]
    GroupDeleted,
    #[serde(rename = "groupUpdated")]
    GroupUpdated {
        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "invitedViaGroupLink")]
    InvitedViaGroupLink,
    #[serde(rename = "memberCreatedContact")]
    MemberCreatedContact,
    #[serde(rename = "memberProfileUpdated")]
    MemberProfileUpdated {
        #[serde(rename = "fromProfile")]
        from_profile: Profile,

        #[serde(rename = "toProfile")]
        to_profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "newMemberPendingReview")]
    NewMemberPendingReview,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReportReason {
    #[default]
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "community")]
    Community,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RoleGroupPreference {
    #[serde(rename = "enable")]
    pub enable: GroupFeatureEnabled,

    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<GroupMemberRole>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SecurityCode {
    #[serde(rename = "securityCode")]
    pub security_code: String,

    #[serde(rename = "verifiedAt")]
    pub verified_at: UtcTime,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SimplePreference {
    #[serde(rename = "allow")]
    pub allow: FeatureAllowed,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SimplexLinkType {
    #[default]
    #[serde(rename = "contact")]
    Contact,
    #[serde(rename = "invitation")]
    Invitation,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "relay")]
    Relay,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SndCIStatusProgress {
    #[default]
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "complete")]
    Complete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SndConnEvent {
    #[serde(rename = "switchQueue")]
    SwitchQueue {
        #[serde(rename = "phase")]
        phase: SwitchPhase,

        #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
        member: Option<GroupMemberRef>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "ratchetSync")]
    RatchetSync {
        #[serde(rename = "syncStatus")]
        sync_status: RatchetSyncState,

        #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
        member: Option<GroupMemberRef>,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "pqEnabled")]
    PqEnabled {
        #[serde(rename = "enabled")]
        enabled: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SndFileTransfer {
    #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
    pub file_id: i64,

    #[serde(rename = "fileName")]
    pub file_name: String,

    #[serde(rename = "filePath")]
    pub file_path: String,

    #[serde(
        rename = "fileSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub file_size: i64,

    #[serde(
        rename = "chunkSize",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub chunk_size: i64,

    #[serde(rename = "recipientDisplayName")]
    pub recipient_display_name: String,

    #[serde(rename = "connId", deserialize_with = "deserialize_number_from_string")]
    pub conn_id: i64,

    #[serde(rename = "agentConnId")]
    pub agent_conn_id: String,

    #[serde(
        rename = "groupMemberId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub group_member_id: Option<i64>,

    #[serde(rename = "fileStatus")]
    pub file_status: FileStatus,

    #[serde(
        rename = "fileDescrId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub file_descr_id: Option<i64>,

    #[serde(rename = "fileInline", skip_serializing_if = "Option::is_none")]
    pub file_inline: Option<InlineFileMode>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SndGroupEvent {
    #[serde(rename = "memberRole")]
    MemberRole {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(rename = "role")]
        role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "memberBlocked")]
    MemberBlocked {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(rename = "blocked")]
        blocked: bool,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userRole")]
    UserRole {
        #[serde(rename = "role")]
        role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "memberDeleted")]
    MemberDeleted {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userLeft")]
    UserLeft,
    #[serde(rename = "groupUpdated")]
    GroupUpdated {
        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "memberAccepted")]
    MemberAccepted {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
        undocumented: BTreeMap<String, JsonObject>,
    },
    #[serde(rename = "userPendingReview")]
    UserPendingReview,
    #[serde(untagged)]
    Undocumented(BTreeMap<String, JsonObject>),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SwitchPhase {
    #[default]
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "secured")]
    Secured,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct TimedMessagesGroupPreference {
    #[serde(rename = "enable")]
    pub enable: GroupFeatureEnabled,

    #[serde(
        rename = "ttl",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub ttl: Option<i32>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct TimedMessagesPreference {
    #[serde(rename = "allow")]
    pub allow: FeatureAllowed,

    #[serde(
        rename = "ttl",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub ttl: Option<i32>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UIColorMode {
    #[default]
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UIColors {
    #[serde(rename = "accent", skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,

    #[serde(rename = "accentVariant", skip_serializing_if = "Option::is_none")]
    pub accent_variant: Option<String>,

    #[serde(rename = "secondary", skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,

    #[serde(rename = "secondaryVariant", skip_serializing_if = "Option::is_none")]
    pub secondary_variant: Option<String>,

    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,

    #[serde(rename = "menus", skip_serializing_if = "Option::is_none")]
    pub menus: Option<String>,

    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "accentVariant2", skip_serializing_if = "Option::is_none")]
    pub accent_variant_2: Option<String>,

    #[serde(rename = "sentMessage", skip_serializing_if = "Option::is_none")]
    pub sent_message: Option<String>,

    #[serde(rename = "sentReply", skip_serializing_if = "Option::is_none")]
    pub sent_reply: Option<String>,

    #[serde(rename = "receivedMessage", skip_serializing_if = "Option::is_none")]
    pub received_message: Option<String>,

    #[serde(rename = "receivedReply", skip_serializing_if = "Option::is_none")]
    pub received_reply: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UIThemeEntityOverride {
    #[serde(rename = "mode")]
    pub mode: UIColorMode,

    #[serde(rename = "wallpaper", skip_serializing_if = "Option::is_none")]
    pub wallpaper: Option<ChatWallpaper>,

    #[serde(rename = "colors")]
    pub colors: UIColors,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UIThemeEntityOverrides {
    #[serde(rename = "light", skip_serializing_if = "Option::is_none")]
    pub light: Option<UIThemeEntityOverride>,

    #[serde(rename = "dark", skip_serializing_if = "Option::is_none")]
    pub dark: Option<UIThemeEntityOverride>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UpdatedMessage {
    #[serde(rename = "msgContent")]
    pub msg_content: MsgContent,

    #[serde(rename = "mentions")]
    pub mentions: HashMap<String, i64>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct User {
    #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
    pub user_id: i64,

    #[serde(
        rename = "agentUserId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub agent_user_id: i64,

    #[serde(
        rename = "userContactId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub user_contact_id: i64,

    #[serde(rename = "localDisplayName")]
    pub local_display_name: String,

    #[serde(rename = "profile")]
    pub profile: LocalProfile,

    #[serde(rename = "fullPreferences")]
    pub full_preferences: FullPreferences,

    #[serde(rename = "activeUser")]
    pub active_user: bool,

    #[serde(
        rename = "activeOrder",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub active_order: i64,

    #[serde(rename = "viewPwdHash", skip_serializing_if = "Option::is_none")]
    pub view_pwd_hash: Option<UserPwdHash>,

    #[serde(rename = "showNtfs")]
    pub show_ntfs: bool,

    #[serde(rename = "sendRcptsContacts")]
    pub send_rcpts_contacts: bool,

    #[serde(rename = "sendRcptsSmallGroups")]
    pub send_rcpts_small_groups: bool,

    #[serde(rename = "autoAcceptMemberContacts")]
    pub auto_accept_member_contacts: bool,

    #[serde(
        rename = "userMemberProfileUpdatedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_member_profile_updated_at: Option<UtcTime>,

    #[serde(rename = "uiThemes", skip_serializing_if = "Option::is_none")]
    pub ui_themes: Option<UIThemeEntityOverrides>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContact {
    #[serde(
        rename = "userContactLinkId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub user_contact_link_id: i64,

    #[serde(rename = "connReqContact")]
    pub conn_req_contact: String,

    #[serde(
        rename = "groupId",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub group_id: Option<i64>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLink {
    #[serde(
        rename = "userContactLinkId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub user_contact_link_id: i64,

    #[serde(rename = "connLinkContact")]
    pub conn_link_contact: CreatedConnLink,

    #[serde(rename = "shortLinkDataSet")]
    pub short_link_data_set: bool,

    #[serde(rename = "shortLinkLargeDataSet")]
    pub short_link_large_data_set: bool,

    #[serde(rename = "addressSettings")]
    pub address_settings: AddressSettings,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactRequest {
    #[serde(
        rename = "contactRequestId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub contact_request_id: i64,

    #[serde(rename = "agentInvitationId")]
    pub agent_invitation_id: String,

    #[serde(
        rename = "contactId_",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub contact_id: Option<i64>,

    #[serde(
        rename = "businessGroupId_",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub business_group_id: Option<i64>,

    #[serde(
        rename = "userContactLinkId_",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub user_contact_link_id: Option<i64>,

    #[serde(rename = "cReqChatVRange")]
    pub c_req_chat_v_range: VersionRange,

    #[serde(rename = "localDisplayName")]
    pub local_display_name: String,

    #[serde(
        rename = "profileId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub profile_id: i64,

    #[serde(rename = "profile")]
    pub profile: Profile,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(rename = "xContactId", skip_serializing_if = "Option::is_none")]
    pub x_contact_id: Option<String>,

    #[serde(rename = "pqSupport")]
    pub pq_support: bool,

    #[serde(rename = "welcomeSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub welcome_shared_msg_id: Option<String>,

    #[serde(rename = "requestSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub request_shared_msg_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserInfo {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(
        rename = "unreadCount",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub unread_count: i32,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserProfileUpdateSummary {
    #[serde(
        rename = "updateSuccesses",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub update_successes: i32,

    #[serde(
        rename = "updateFailures",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub update_failures: i32,

    #[serde(rename = "changedContacts")]
    pub changed_contacts: Vec<Contact>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserPwdHash {
    #[serde(rename = "hash")]
    pub hash: String,

    #[serde(rename = "salt")]
    pub salt: String,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct VersionRange {
    #[serde(
        rename = "minVersion",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub min_version: i32,

    #[serde(
        rename = "maxVersion",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub max_version: i32,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct XFTPRcvFile {
    #[serde(rename = "rcvFileDescription")]
    pub rcv_file_description: RcvFileDescr,

    #[serde(rename = "agentRcvFileId", skip_serializing_if = "Option::is_none")]
    pub agent_rcv_file_id: Option<String>,

    #[serde(rename = "agentRcvFileDeleted")]
    pub agent_rcv_file_deleted: bool,

    #[serde(rename = "userApprovedRelays")]
    pub user_approved_relays: bool,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct XFTPSndFile {
    #[serde(rename = "agentSndFileId")]
    pub agent_snd_file_id: String,

    #[serde(
        rename = "privateSndFileDescr",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_snd_file_descr: Option<String>,

    #[serde(rename = "agentSndFileDeleted")]
    pub agent_snd_file_deleted: bool,

    #[serde(rename = "cryptoArgs", skip_serializing_if = "Option::is_none")]
    pub crypto_args: Option<CryptoFileArgs>,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: BTreeMap<String, JsonObject>,
}
