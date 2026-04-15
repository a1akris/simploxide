//! This crate is auto-generated

#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
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
use std::{collections::BTreeMap, fmt::Write as _, sync::Arc};
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AChatItem {
    #[serde(rename = "chatInfo")]
    pub chat_info: ChatInfo,

    #[serde(rename = "chatItem")]
    pub chat_item: ChatItem,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AddressSettings {
    #[serde(rename = "businessAddress", default)]
    pub business_address: bool,

    #[serde(rename = "autoAccept", skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<AutoAccept>,

    #[serde(rename = "autoReply", skip_serializing_if = "Option::is_none")]
    pub auto_reply: Option<MsgContent>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AutoAccept {
    #[serde(rename = "acceptIncognito", default)]
    pub accept_incognito: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct BlockingInfo {
    #[serde(rename = "reason")]
    pub reason: BlockingReason,

    #[serde(rename = "notice", skip_serializing_if = "Option::is_none")]
    pub notice: Option<ClientNotice>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvMsgContent")]
    RcvMsgContent {
        #[serde(rename = "msgContent")]
        msg_content: MsgContent,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndDeleted")]
    SndDeleted {
        #[serde(rename = "deleteMode")]
        delete_mode: CIDeleteMode,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvDeleted")]
    RcvDeleted {
        #[serde(rename = "deleteMode")]
        delete_mode: CIDeleteMode,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvIntegrityError")]
    RcvIntegrityError {
        #[serde(rename = "msgError")]
        msg_error: MsgErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvMsgError")]
    RcvMsgError {
        #[serde(rename = "rcvMsgError")]
        rcv_msg_error: RcvMsgError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvGroupInvitation")]
    RcvGroupInvitation {
        #[serde(rename = "groupInvitation")]
        group_invitation: CIGroupInvitation,

        #[serde(rename = "memberRole")]
        member_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndGroupInvitation")]
    SndGroupInvitation {
        #[serde(rename = "groupInvitation")]
        group_invitation: CIGroupInvitation,

        #[serde(rename = "memberRole")]
        member_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvDirectEvent")]
    RcvDirectEvent {
        #[serde(rename = "rcvDirectEvent")]
        rcv_direct_event: RcvDirectEvent,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvGroupEvent")]
    RcvGroupEvent {
        #[serde(rename = "rcvGroupEvent")]
        rcv_group_event: RcvGroupEvent,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndGroupEvent")]
    SndGroupEvent {
        #[serde(rename = "sndGroupEvent")]
        snd_group_event: SndGroupEvent,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvConnEvent")]
    RcvConnEvent {
        #[serde(rename = "rcvConnEvent")]
        rcv_conn_event: RcvConnEvent,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndConnEvent")]
    SndConnEvent {
        #[serde(rename = "sndConnEvent")]
        snd_conn_event: SndConnEvent,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvChatFeatureRejected")]
    RcvChatFeatureRejected {
        #[serde(rename = "feature")]
        feature: ChatFeature,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvGroupFeatureRejected")]
    RcvGroupFeatureRejected {
        #[serde(rename = "groupFeature")]
        group_feature: GroupFeature,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvDirectE2EEInfo")]
    RcvDirectE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndGroupE2EEInfo")]
    SndGroupE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvGroupE2EEInfo")]
    RcvGroupE2EeInfo {
        #[serde(rename = "e2eeInfo")]
        e_2_ee_info: E2EInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatBanner")]
    ChatBanner,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CIContent {
    pub fn make_snd_msg_content(msg_content: MsgContent) -> Self {
        Self::SndMsgContent {
            msg_content,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_msg_content(msg_content: MsgContent) -> Self {
        Self::RcvMsgContent {
            msg_content,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_deleted(delete_mode: CIDeleteMode) -> Self {
        Self::SndDeleted {
            delete_mode,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_deleted(delete_mode: CIDeleteMode) -> Self {
        Self::RcvDeleted {
            delete_mode,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_call(status: CICallStatus, duration: i32) -> Self {
        Self::SndCall {
            status,
            duration,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_call(status: CICallStatus, duration: i32) -> Self {
        Self::RcvCall {
            status,
            duration,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_integrity_error(msg_error: MsgErrorType) -> Self {
        Self::RcvIntegrityError {
            msg_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_decryption_error(msg_decrypt_error: MsgDecryptError, msg_count: u32) -> Self {
        Self::RcvDecryptionError {
            msg_decrypt_error,
            msg_count,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_msg_error(rcv_msg_error: RcvMsgError) -> Self {
        Self::RcvMsgError {
            rcv_msg_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_group_invitation(
        group_invitation: CIGroupInvitation,
        member_role: GroupMemberRole,
    ) -> Self {
        Self::RcvGroupInvitation {
            group_invitation,
            member_role,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_group_invitation(
        group_invitation: CIGroupInvitation,
        member_role: GroupMemberRole,
    ) -> Self {
        Self::SndGroupInvitation {
            group_invitation,
            member_role,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_direct_event(rcv_direct_event: RcvDirectEvent) -> Self {
        Self::RcvDirectEvent {
            rcv_direct_event,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_group_event(rcv_group_event: RcvGroupEvent) -> Self {
        Self::RcvGroupEvent {
            rcv_group_event,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_group_event(snd_group_event: SndGroupEvent) -> Self {
        Self::SndGroupEvent {
            snd_group_event,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_conn_event(rcv_conn_event: RcvConnEvent) -> Self {
        Self::RcvConnEvent {
            rcv_conn_event,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_conn_event(snd_conn_event: SndConnEvent) -> Self {
        Self::SndConnEvent {
            snd_conn_event,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_chat_feature(
        feature: ChatFeature,
        enabled: PrefEnabled,
        param: Option<i32>,
    ) -> Self {
        Self::RcvChatFeature {
            feature,
            enabled,
            param,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_chat_feature(
        feature: ChatFeature,
        enabled: PrefEnabled,
        param: Option<i32>,
    ) -> Self {
        Self::SndChatFeature {
            feature,
            enabled,
            param,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_chat_preference(
        feature: ChatFeature,
        allowed: FeatureAllowed,
        param: Option<i32>,
    ) -> Self {
        Self::RcvChatPreference {
            feature,
            allowed,
            param,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_chat_preference(
        feature: ChatFeature,
        allowed: FeatureAllowed,
        param: Option<i32>,
    ) -> Self {
        Self::SndChatPreference {
            feature,
            allowed,
            param,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_group_feature(
        group_feature: GroupFeature,
        preference: GroupPreference,
        param: Option<i32>,
        member_role: Option<GroupMemberRole>,
    ) -> Self {
        Self::RcvGroupFeature {
            group_feature,
            preference,
            param,
            member_role,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_group_feature(
        group_feature: GroupFeature,
        preference: GroupPreference,
        param: Option<i32>,
        member_role: Option<GroupMemberRole>,
    ) -> Self {
        Self::SndGroupFeature {
            group_feature,
            preference,
            param,
            member_role,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_chat_feature_rejected(feature: ChatFeature) -> Self {
        Self::RcvChatFeatureRejected {
            feature,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_group_feature_rejected(group_feature: GroupFeature) -> Self {
        Self::RcvGroupFeatureRejected {
            group_feature,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_moderated() -> Self {
        Self::SndModerated
    }

    pub fn make_rcv_moderated() -> Self {
        Self::RcvModerated
    }

    pub fn make_rcv_blocked() -> Self {
        Self::RcvBlocked
    }

    pub fn make_snd_direct_e_2_ee_info(e_2_ee_info: E2EInfo) -> Self {
        Self::SndDirectE2EeInfo {
            e_2_ee_info,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_direct_e_2_ee_info(e_2_ee_info: E2EInfo) -> Self {
        Self::RcvDirectE2EeInfo {
            e_2_ee_info,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_group_e_2_ee_info(e_2_ee_info: E2EInfo) -> Self {
        Self::SndGroupE2EeInfo {
            e_2_ee_info,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_group_e_2_ee_info(e_2_ee_info: E2EInfo) -> Self {
        Self::RcvGroupE2EeInfo {
            e_2_ee_info,
            undocumented: Default::default(),
        }
    }

    pub fn make_chat_banner() -> Self {
        Self::ChatBanner
    }
}

impl CIContent {
    pub fn snd_msg_content(&self) -> Option<&MsgContent> {
        if let Self::SndMsgContent { msg_content, .. } = self {
            Some(msg_content)
        } else {
            None
        }
    }
    pub fn rcv_msg_content(&self) -> Option<&MsgContent> {
        if let Self::RcvMsgContent { msg_content, .. } = self {
            Some(msg_content)
        } else {
            None
        }
    }
    pub fn snd_deleted(&self) -> Option<&CIDeleteMode> {
        if let Self::SndDeleted { delete_mode, .. } = self {
            Some(delete_mode)
        } else {
            None
        }
    }
    pub fn rcv_deleted(&self) -> Option<&CIDeleteMode> {
        if let Self::RcvDeleted { delete_mode, .. } = self {
            Some(delete_mode)
        } else {
            None
        }
    }
    pub fn snd_call(&self) -> Option<CIContentSndCallRef<'_>> {
        if let Self::SndCall {
            status, duration, ..
        } = self
        {
            Some(CIContentSndCallRef { status, duration })
        } else {
            None
        }
    }
    pub fn rcv_call(&self) -> Option<CIContentRcvCallRef<'_>> {
        if let Self::RcvCall {
            status, duration, ..
        } = self
        {
            Some(CIContentRcvCallRef { status, duration })
        } else {
            None
        }
    }
    pub fn rcv_integrity_error(&self) -> Option<&MsgErrorType> {
        if let Self::RcvIntegrityError { msg_error, .. } = self {
            Some(msg_error)
        } else {
            None
        }
    }
    pub fn rcv_decryption_error(&self) -> Option<CIContentRcvDecryptionErrorRef<'_>> {
        if let Self::RcvDecryptionError {
            msg_decrypt_error,
            msg_count,
            ..
        } = self
        {
            Some(CIContentRcvDecryptionErrorRef {
                msg_decrypt_error,
                msg_count,
            })
        } else {
            None
        }
    }
    pub fn rcv_msg_error(&self) -> Option<&RcvMsgError> {
        if let Self::RcvMsgError { rcv_msg_error, .. } = self {
            Some(rcv_msg_error)
        } else {
            None
        }
    }
    pub fn rcv_group_invitation(&self) -> Option<CIContentRcvGroupInvitationRef<'_>> {
        if let Self::RcvGroupInvitation {
            group_invitation,
            member_role,
            ..
        } = self
        {
            Some(CIContentRcvGroupInvitationRef {
                group_invitation,
                member_role,
            })
        } else {
            None
        }
    }
    pub fn snd_group_invitation(&self) -> Option<CIContentSndGroupInvitationRef<'_>> {
        if let Self::SndGroupInvitation {
            group_invitation,
            member_role,
            ..
        } = self
        {
            Some(CIContentSndGroupInvitationRef {
                group_invitation,
                member_role,
            })
        } else {
            None
        }
    }
    pub fn rcv_direct_event(&self) -> Option<&RcvDirectEvent> {
        if let Self::RcvDirectEvent {
            rcv_direct_event, ..
        } = self
        {
            Some(rcv_direct_event)
        } else {
            None
        }
    }
    pub fn rcv_group_event(&self) -> Option<&RcvGroupEvent> {
        if let Self::RcvGroupEvent {
            rcv_group_event, ..
        } = self
        {
            Some(rcv_group_event)
        } else {
            None
        }
    }
    pub fn snd_group_event(&self) -> Option<&SndGroupEvent> {
        if let Self::SndGroupEvent {
            snd_group_event, ..
        } = self
        {
            Some(snd_group_event)
        } else {
            None
        }
    }
    pub fn rcv_conn_event(&self) -> Option<&RcvConnEvent> {
        if let Self::RcvConnEvent { rcv_conn_event, .. } = self {
            Some(rcv_conn_event)
        } else {
            None
        }
    }
    pub fn snd_conn_event(&self) -> Option<&SndConnEvent> {
        if let Self::SndConnEvent { snd_conn_event, .. } = self {
            Some(snd_conn_event)
        } else {
            None
        }
    }
    pub fn rcv_chat_feature(&self) -> Option<CIContentRcvChatFeatureRef<'_>> {
        if let Self::RcvChatFeature {
            feature,
            enabled,
            param,
            ..
        } = self
        {
            Some(CIContentRcvChatFeatureRef {
                feature,
                enabled,
                param,
            })
        } else {
            None
        }
    }
    pub fn snd_chat_feature(&self) -> Option<CIContentSndChatFeatureRef<'_>> {
        if let Self::SndChatFeature {
            feature,
            enabled,
            param,
            ..
        } = self
        {
            Some(CIContentSndChatFeatureRef {
                feature,
                enabled,
                param,
            })
        } else {
            None
        }
    }
    pub fn rcv_chat_preference(&self) -> Option<CIContentRcvChatPreferenceRef<'_>> {
        if let Self::RcvChatPreference {
            feature,
            allowed,
            param,
            ..
        } = self
        {
            Some(CIContentRcvChatPreferenceRef {
                feature,
                allowed,
                param,
            })
        } else {
            None
        }
    }
    pub fn snd_chat_preference(&self) -> Option<CIContentSndChatPreferenceRef<'_>> {
        if let Self::SndChatPreference {
            feature,
            allowed,
            param,
            ..
        } = self
        {
            Some(CIContentSndChatPreferenceRef {
                feature,
                allowed,
                param,
            })
        } else {
            None
        }
    }
    pub fn rcv_group_feature(&self) -> Option<CIContentRcvGroupFeatureRef<'_>> {
        if let Self::RcvGroupFeature {
            group_feature,
            preference,
            param,
            member_role,
            ..
        } = self
        {
            Some(CIContentRcvGroupFeatureRef {
                group_feature,
                preference,
                param,
                member_role,
            })
        } else {
            None
        }
    }
    pub fn snd_group_feature(&self) -> Option<CIContentSndGroupFeatureRef<'_>> {
        if let Self::SndGroupFeature {
            group_feature,
            preference,
            param,
            member_role,
            ..
        } = self
        {
            Some(CIContentSndGroupFeatureRef {
                group_feature,
                preference,
                param,
                member_role,
            })
        } else {
            None
        }
    }
    pub fn rcv_chat_feature_rejected(&self) -> Option<&ChatFeature> {
        if let Self::RcvChatFeatureRejected { feature, .. } = self {
            Some(feature)
        } else {
            None
        }
    }
    pub fn rcv_group_feature_rejected(&self) -> Option<&GroupFeature> {
        if let Self::RcvGroupFeatureRejected { group_feature, .. } = self {
            Some(group_feature)
        } else {
            None
        }
    }
    pub fn is_snd_moderated(&self) -> bool {
        matches!(self, Self::SndModerated)
    }
    pub fn is_rcv_moderated(&self) -> bool {
        matches!(self, Self::RcvModerated)
    }
    pub fn is_rcv_blocked(&self) -> bool {
        matches!(self, Self::RcvBlocked)
    }
    pub fn snd_direct_e_2_ee_info(&self) -> Option<&E2EInfo> {
        if let Self::SndDirectE2EeInfo { e_2_ee_info, .. } = self {
            Some(e_2_ee_info)
        } else {
            None
        }
    }
    pub fn rcv_direct_e_2_ee_info(&self) -> Option<&E2EInfo> {
        if let Self::RcvDirectE2EeInfo { e_2_ee_info, .. } = self {
            Some(e_2_ee_info)
        } else {
            None
        }
    }
    pub fn snd_group_e_2_ee_info(&self) -> Option<&E2EInfo> {
        if let Self::SndGroupE2EeInfo { e_2_ee_info, .. } = self {
            Some(e_2_ee_info)
        } else {
            None
        }
    }
    pub fn rcv_group_e_2_ee_info(&self) -> Option<&E2EInfo> {
        if let Self::RcvGroupE2EeInfo { e_2_ee_info, .. } = self {
            Some(e_2_ee_info)
        } else {
            None
        }
    }
    pub fn is_chat_banner(&self) -> bool {
        matches!(self, Self::ChatBanner)
    }
}
#[derive(Clone, Copy)]
pub struct CIContentSndCallRef<'a> {
    pub status: &'a CICallStatus,
    pub duration: &'a i32,
}
#[derive(Clone, Copy)]
pub struct CIContentRcvCallRef<'a> {
    pub status: &'a CICallStatus,
    pub duration: &'a i32,
}
#[derive(Clone, Copy)]
pub struct CIContentRcvDecryptionErrorRef<'a> {
    pub msg_decrypt_error: &'a MsgDecryptError,
    pub msg_count: &'a u32,
}
#[derive(Clone, Copy)]
pub struct CIContentRcvGroupInvitationRef<'a> {
    pub group_invitation: &'a CIGroupInvitation,
    pub member_role: &'a GroupMemberRole,
}
#[derive(Clone, Copy)]
pub struct CIContentSndGroupInvitationRef<'a> {
    pub group_invitation: &'a CIGroupInvitation,
    pub member_role: &'a GroupMemberRole,
}
#[derive(Clone, Copy)]
pub struct CIContentRcvChatFeatureRef<'a> {
    pub feature: &'a ChatFeature,
    pub enabled: &'a PrefEnabled,
    pub param: &'a Option<i32>,
}
#[derive(Clone, Copy)]
pub struct CIContentSndChatFeatureRef<'a> {
    pub feature: &'a ChatFeature,
    pub enabled: &'a PrefEnabled,
    pub param: &'a Option<i32>,
}
#[derive(Clone, Copy)]
pub struct CIContentRcvChatPreferenceRef<'a> {
    pub feature: &'a ChatFeature,
    pub allowed: &'a FeatureAllowed,
    pub param: &'a Option<i32>,
}
#[derive(Clone, Copy)]
pub struct CIContentSndChatPreferenceRef<'a> {
    pub feature: &'a ChatFeature,
    pub allowed: &'a FeatureAllowed,
    pub param: &'a Option<i32>,
}
#[derive(Clone, Copy)]
pub struct CIContentRcvGroupFeatureRef<'a> {
    pub group_feature: &'a GroupFeature,
    pub preference: &'a GroupPreference,
    pub param: &'a Option<i32>,
    pub member_role: &'a Option<GroupMemberRole>,
}
#[derive(Clone, Copy)]
pub struct CIContentSndGroupFeatureRef<'a> {
    pub group_feature: &'a GroupFeature,
    pub preference: &'a GroupPreference,
    pub param: &'a Option<i32>,
    pub member_role: &'a Option<GroupMemberRole>,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "blocked")]
    Blocked {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "blockedByAdmin")]
    BlockedByAdmin {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "moderated")]
    Moderated {
        #[serde(rename = "deletedTs", skip_serializing_if = "Option::is_none")]
        deleted_ts: Option<UtcTime>,

        #[serde(rename = "byGroupMember")]
        by_group_member: GroupMember,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CIDeleted {
    pub fn make_deleted(deleted_ts: Option<UtcTime>, chat_type: ChatType) -> Self {
        Self::Deleted {
            deleted_ts,
            chat_type,
            undocumented: Default::default(),
        }
    }

    pub fn make_blocked(deleted_ts: Option<UtcTime>) -> Self {
        Self::Blocked {
            deleted_ts,
            undocumented: Default::default(),
        }
    }

    pub fn make_blocked_by_admin(deleted_ts: Option<UtcTime>) -> Self {
        Self::BlockedByAdmin {
            deleted_ts,
            undocumented: Default::default(),
        }
    }

    pub fn make_moderated(deleted_ts: Option<UtcTime>, by_group_member: GroupMember) -> Self {
        Self::Moderated {
            deleted_ts,
            by_group_member,
            undocumented: Default::default(),
        }
    }
}

impl CIDeleted {
    pub fn deleted(&self) -> Option<CIDeletedDeletedRef<'_>> {
        if let Self::Deleted {
            deleted_ts,
            chat_type,
            ..
        } = self
        {
            Some(CIDeletedDeletedRef {
                deleted_ts,
                chat_type,
            })
        } else {
            None
        }
    }
    pub fn blocked(&self) -> Option<&Option<UtcTime>> {
        if let Self::Blocked { deleted_ts, .. } = self {
            Some(deleted_ts)
        } else {
            None
        }
    }
    pub fn blocked_by_admin(&self) -> Option<&Option<UtcTime>> {
        if let Self::BlockedByAdmin { deleted_ts, .. } = self {
            Some(deleted_ts)
        } else {
            None
        }
    }
    pub fn moderated(&self) -> Option<CIDeletedModeratedRef<'_>> {
        if let Self::Moderated {
            deleted_ts,
            by_group_member,
            ..
        } = self
        {
            Some(CIDeletedModeratedRef {
                deleted_ts,
                by_group_member,
            })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct CIDeletedDeletedRef<'a> {
    pub deleted_ts: &'a Option<UtcTime>,
    pub chat_type: &'a ChatType,
}
#[derive(Clone, Copy)]
pub struct CIDeletedModeratedRef<'a> {
    pub deleted_ts: &'a Option<UtcTime>,
    pub by_group_member: &'a GroupMember,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "channelRcv")]
    ChannelRcv,
    #[serde(rename = "localSnd")]
    LocalSnd,
    #[serde(rename = "localRcv")]
    LocalRcv,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CIDirection {
    pub fn make_direct_snd() -> Self {
        Self::DirectSnd
    }

    pub fn make_direct_rcv() -> Self {
        Self::DirectRcv
    }

    pub fn make_group_snd() -> Self {
        Self::GroupSnd
    }

    pub fn make_group_rcv(group_member: GroupMember) -> Self {
        Self::GroupRcv {
            group_member,
            undocumented: Default::default(),
        }
    }

    pub fn make_channel_rcv() -> Self {
        Self::ChannelRcv
    }

    pub fn make_local_snd() -> Self {
        Self::LocalSnd
    }

    pub fn make_local_rcv() -> Self {
        Self::LocalRcv
    }
}

impl CIDirection {
    pub fn is_direct_snd(&self) -> bool {
        matches!(self, Self::DirectSnd)
    }
    pub fn is_direct_rcv(&self) -> bool {
        matches!(self, Self::DirectRcv)
    }
    pub fn is_group_snd(&self) -> bool {
        matches!(self, Self::GroupSnd)
    }
    pub fn group_rcv(&self) -> Option<&GroupMember> {
        if let Self::GroupRcv { group_member, .. } = self {
            Some(group_member)
        } else {
            None
        }
    }
    pub fn is_channel_rcv(&self) -> bool {
        matches!(self, Self::ChannelRcv)
    }
    pub fn is_local_snd(&self) -> bool {
        matches!(self, Self::LocalSnd)
    }
    pub fn is_local_rcv(&self) -> bool {
        matches!(self, Self::LocalRcv)
    }
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndCancelled")]
    SndCancelled,
    #[serde(rename = "sndComplete")]
    SndComplete,
    #[serde(rename = "sndError")]
    SndError {
        #[serde(rename = "sndFileError")]
        snd_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndWarning")]
    SndWarning {
        #[serde(rename = "sndFileError")]
        snd_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvWarning")]
    RcvWarning {
        #[serde(rename = "rcvFileError")]
        rcv_file_error: FileError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalid")]
    Invalid {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CIFileStatus {
    pub fn make_snd_stored() -> Self {
        Self::SndStored
    }

    pub fn make_snd_transfer(snd_progress: i64, snd_total: i64) -> Self {
        Self::SndTransfer {
            snd_progress,
            snd_total,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_cancelled() -> Self {
        Self::SndCancelled
    }

    pub fn make_snd_complete() -> Self {
        Self::SndComplete
    }

    pub fn make_snd_error(snd_file_error: FileError) -> Self {
        Self::SndError {
            snd_file_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_warning(snd_file_error: FileError) -> Self {
        Self::SndWarning {
            snd_file_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_invitation() -> Self {
        Self::RcvInvitation
    }

    pub fn make_rcv_accepted() -> Self {
        Self::RcvAccepted
    }

    pub fn make_rcv_transfer(rcv_progress: i64, rcv_total: i64) -> Self {
        Self::RcvTransfer {
            rcv_progress,
            rcv_total,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_aborted() -> Self {
        Self::RcvAborted
    }

    pub fn make_rcv_complete() -> Self {
        Self::RcvComplete
    }

    pub fn make_rcv_cancelled() -> Self {
        Self::RcvCancelled
    }

    pub fn make_rcv_error(rcv_file_error: FileError) -> Self {
        Self::RcvError {
            rcv_file_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_warning(rcv_file_error: FileError) -> Self {
        Self::RcvWarning {
            rcv_file_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_invalid(text: String) -> Self {
        Self::Invalid {
            text,
            undocumented: Default::default(),
        }
    }
}

impl CIFileStatus {
    pub fn is_snd_stored(&self) -> bool {
        matches!(self, Self::SndStored)
    }
    pub fn snd_transfer(&self) -> Option<CIFileStatusSndTransferRef<'_>> {
        if let Self::SndTransfer {
            snd_progress,
            snd_total,
            ..
        } = self
        {
            Some(CIFileStatusSndTransferRef {
                snd_progress,
                snd_total,
            })
        } else {
            None
        }
    }
    pub fn is_snd_cancelled(&self) -> bool {
        matches!(self, Self::SndCancelled)
    }
    pub fn is_snd_complete(&self) -> bool {
        matches!(self, Self::SndComplete)
    }
    pub fn snd_error(&self) -> Option<&FileError> {
        if let Self::SndError { snd_file_error, .. } = self {
            Some(snd_file_error)
        } else {
            None
        }
    }
    pub fn snd_warning(&self) -> Option<&FileError> {
        if let Self::SndWarning { snd_file_error, .. } = self {
            Some(snd_file_error)
        } else {
            None
        }
    }
    pub fn is_rcv_invitation(&self) -> bool {
        matches!(self, Self::RcvInvitation)
    }
    pub fn is_rcv_accepted(&self) -> bool {
        matches!(self, Self::RcvAccepted)
    }
    pub fn rcv_transfer(&self) -> Option<CIFileStatusRcvTransferRef<'_>> {
        if let Self::RcvTransfer {
            rcv_progress,
            rcv_total,
            ..
        } = self
        {
            Some(CIFileStatusRcvTransferRef {
                rcv_progress,
                rcv_total,
            })
        } else {
            None
        }
    }
    pub fn is_rcv_aborted(&self) -> bool {
        matches!(self, Self::RcvAborted)
    }
    pub fn is_rcv_complete(&self) -> bool {
        matches!(self, Self::RcvComplete)
    }
    pub fn is_rcv_cancelled(&self) -> bool {
        matches!(self, Self::RcvCancelled)
    }
    pub fn rcv_error(&self) -> Option<&FileError> {
        if let Self::RcvError { rcv_file_error, .. } = self {
            Some(rcv_file_error)
        } else {
            None
        }
    }
    pub fn rcv_warning(&self) -> Option<&FileError> {
        if let Self::RcvWarning { rcv_file_error, .. } = self {
            Some(rcv_file_error)
        } else {
            None
        }
    }
    pub fn invalid(&self) -> Option<&String> {
        if let Self::Invalid { text, .. } = self {
            Some(text)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct CIFileStatusSndTransferRef<'a> {
    pub snd_progress: &'a i64,
    pub snd_total: &'a i64,
}
#[derive(Clone, Copy)]
pub struct CIFileStatusRcvTransferRef<'a> {
    pub rcv_progress: &'a i64,
    pub rcv_total: &'a i64,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CIForwardedFrom {
    pub fn make_unknown() -> Self {
        Self::Unknown
    }

    pub fn make_contact(
        chat_name: String,
        msg_dir: MsgDirection,
        contact_id: Option<i64>,
        chat_item_id: Option<i64>,
    ) -> Self {
        Self::Contact {
            chat_name,
            msg_dir,
            contact_id,
            chat_item_id,
            undocumented: Default::default(),
        }
    }

    pub fn make_group(
        chat_name: String,
        msg_dir: MsgDirection,
        group_id: Option<i64>,
        chat_item_id: Option<i64>,
    ) -> Self {
        Self::Group {
            chat_name,
            msg_dir,
            group_id,
            chat_item_id,
            undocumented: Default::default(),
        }
    }
}

impl CIForwardedFrom {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    pub fn contact(&self) -> Option<CIForwardedFromContactRef<'_>> {
        if let Self::Contact {
            chat_name,
            msg_dir,
            contact_id,
            chat_item_id,
            ..
        } = self
        {
            Some(CIForwardedFromContactRef {
                chat_name,
                msg_dir,
                contact_id,
                chat_item_id,
            })
        } else {
            None
        }
    }
    pub fn group(&self) -> Option<CIForwardedFromGroupRef<'_>> {
        if let Self::Group {
            chat_name,
            msg_dir,
            group_id,
            chat_item_id,
            ..
        } = self
        {
            Some(CIForwardedFromGroupRef {
                chat_name,
                msg_dir,
                group_id,
                chat_item_id,
            })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct CIForwardedFromContactRef<'a> {
    pub chat_name: &'a String,
    pub msg_dir: &'a MsgDirection,
    pub contact_id: &'a Option<i64>,
    pub chat_item_id: &'a Option<i64>,
}
#[derive(Clone, Copy)]
pub struct CIForwardedFromGroupRef<'a> {
    pub chat_name: &'a String,
    pub msg_dir: &'a MsgDirection,
    pub group_id: &'a Option<i64>,
    pub chat_item_id: &'a Option<i64>,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "itemEdited", default)]
    pub item_edited: bool,

    #[serde(rename = "itemTimed", skip_serializing_if = "Option::is_none")]
    pub item_timed: Option<CITimed>,

    #[serde(rename = "itemLive", skip_serializing_if = "Option::is_none")]
    pub item_live: Option<bool>,

    #[serde(rename = "userMention", default)]
    pub user_mention: bool,

    #[serde(rename = "hasLink", default)]
    pub has_link: bool,

    #[serde(rename = "deletable", default)]
    pub deletable: bool,

    #[serde(rename = "editable", default)]
    pub editable: bool,

    #[serde(
        rename = "forwardedByMember",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub forwarded_by_member: Option<i64>,

    #[serde(rename = "showGroupAsSender", default)]
    pub show_group_as_sender: bool,

    #[serde(rename = "msgSigned", skip_serializing_if = "Option::is_none")]
    pub msg_signed: Option<MsgSigStatus>,

    #[serde(rename = "createdAt")]
    pub created_at: UtcTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: UtcTime,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CIReactionCount {
    #[serde(rename = "reaction")]
    pub reaction: MsgReaction,

    #[serde(rename = "userReacted", default)]
    pub user_reacted: bool,

    #[serde(
        rename = "totalReacted",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub total_reacted: i32,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndRcvd")]
    SndRcvd {
        #[serde(rename = "msgRcptStatus")]
        msg_rcpt_status: MsgReceiptStatus,

        #[serde(rename = "sndProgress")]
        snd_progress: SndCIStatusProgress,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndErrorAuth")]
    SndErrorAuth,
    #[serde(rename = "sndError")]
    SndError {
        #[serde(rename = "agentError")]
        agent_error: SndError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndWarning")]
    SndWarning {
        #[serde(rename = "agentError")]
        agent_error: SndError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvNew")]
    RcvNew,
    #[serde(rename = "rcvRead")]
    RcvRead,
    #[serde(rename = "invalid")]
    Invalid {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CIStatus {
    pub fn make_snd_new() -> Self {
        Self::SndNew
    }

    pub fn make_snd_sent(snd_progress: SndCIStatusProgress) -> Self {
        Self::SndSent {
            snd_progress,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_rcvd(
        msg_rcpt_status: MsgReceiptStatus,
        snd_progress: SndCIStatusProgress,
    ) -> Self {
        Self::SndRcvd {
            msg_rcpt_status,
            snd_progress,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_error_auth() -> Self {
        Self::SndErrorAuth
    }

    pub fn make_snd_error(agent_error: SndError) -> Self {
        Self::SndError {
            agent_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_snd_warning(agent_error: SndError) -> Self {
        Self::SndWarning {
            agent_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_new() -> Self {
        Self::RcvNew
    }

    pub fn make_rcv_read() -> Self {
        Self::RcvRead
    }

    pub fn make_invalid(text: String) -> Self {
        Self::Invalid {
            text,
            undocumented: Default::default(),
        }
    }
}

impl CIStatus {
    pub fn is_snd_new(&self) -> bool {
        matches!(self, Self::SndNew)
    }
    pub fn snd_sent(&self) -> Option<&SndCIStatusProgress> {
        if let Self::SndSent { snd_progress, .. } = self {
            Some(snd_progress)
        } else {
            None
        }
    }
    pub fn snd_rcvd(&self) -> Option<CIStatusSndRcvdRef<'_>> {
        if let Self::SndRcvd {
            msg_rcpt_status,
            snd_progress,
            ..
        } = self
        {
            Some(CIStatusSndRcvdRef {
                msg_rcpt_status,
                snd_progress,
            })
        } else {
            None
        }
    }
    pub fn is_snd_error_auth(&self) -> bool {
        matches!(self, Self::SndErrorAuth)
    }
    pub fn snd_error(&self) -> Option<&SndError> {
        if let Self::SndError { agent_error, .. } = self {
            Some(agent_error)
        } else {
            None
        }
    }
    pub fn snd_warning(&self) -> Option<&SndError> {
        if let Self::SndWarning { agent_error, .. } = self {
            Some(agent_error)
        } else {
            None
        }
    }
    pub fn is_rcv_new(&self) -> bool {
        matches!(self, Self::RcvNew)
    }
    pub fn is_rcv_read(&self) -> bool {
        matches!(self, Self::RcvRead)
    }
    pub fn invalid(&self) -> Option<&String> {
        if let Self::Invalid { text, .. } = self {
            Some(text)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct CIStatusSndRcvdRef<'a> {
    pub msg_rcpt_status: &'a MsgReceiptStatus,
    pub snd_progress: &'a SndCIStatusProgress,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CITimed {
    #[serde(rename = "ttl", deserialize_with = "deserialize_number_from_string")]
    pub ttl: i32,

    #[serde(rename = "deleteAt", skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<UtcTime>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "menu")]
    Menu {
        #[serde(rename = "label")]
        label: String,

        #[serde(rename = "commands")]
        commands: Vec<ChatBotCommand>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ChatBotCommand {
    pub fn make_command(keyword: String, label: String, params: Option<String>) -> Self {
        Self::Command {
            keyword,
            label,
            params,
            undocumented: Default::default(),
        }
    }

    pub fn make_menu(label: String, commands: Vec<ChatBotCommand>) -> Self {
        Self::Menu {
            label,
            commands,
            undocumented: Default::default(),
        }
    }
}

impl ChatBotCommand {
    pub fn command(&self) -> Option<ChatBotCommandCommandRef<'_>> {
        if let Self::Command {
            keyword,
            label,
            params,
            ..
        } = self
        {
            Some(ChatBotCommandCommandRef {
                keyword,
                label,
                params,
            })
        } else {
            None
        }
    }
    pub fn menu(&self) -> Option<ChatBotCommandMenuRef<'_>> {
        if let Self::Menu {
            label, commands, ..
        } = self
        {
            Some(ChatBotCommandMenuRef { label, commands })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct ChatBotCommandCommandRef<'a> {
    pub keyword: &'a String,
    pub label: &'a String,
    pub params: &'a Option<String>,
}
#[derive(Clone, Copy)]
pub struct ChatBotCommandMenuRef<'a> {
    pub label: &'a String,
    pub commands: &'a Vec<ChatBotCommand>,
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
        #[serde(rename = "notify", default)]
        notify: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "entity")]
    Entity {
        #[serde(rename = "notify", default)]
        notify: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "messages")]
    Messages,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CommandSyntax for ChatDeleteMode {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
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
    }
}

impl ChatDeleteMode {
    pub fn make_full(notify: bool) -> Self {
        Self::Full {
            notify,
            undocumented: Default::default(),
        }
    }

    pub fn make_entity(notify: bool) -> Self {
        Self::Entity {
            notify,
            undocumented: Default::default(),
        }
    }

    pub fn make_messages() -> Self {
        Self::Messages
    }
}

impl ChatDeleteMode {
    pub fn full(&self) -> Option<&bool> {
        if let Self::Full { notify, .. } = self {
            Some(notify)
        } else {
            None
        }
    }
    pub fn entity(&self) -> Option<&bool> {
        if let Self::Entity { notify, .. } = self {
            Some(notify)
        } else {
            None
        }
    }
    pub fn is_messages(&self) -> bool {
        matches!(self, Self::Messages)
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "groupChatScope", skip_serializing_if = "Option::is_none")]
        group_chat_scope: Option<GroupChatScopeInfo>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "local")]
    Local {
        #[serde(rename = "noteFolder")]
        note_folder: NoteFolder,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactRequest")]
    ContactRequest {
        #[serde(rename = "contactRequest")]
        contact_request: UserContactRequest,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactConnection")]
    ContactConnection {
        #[serde(rename = "contactConnection")]
        contact_connection: PendingContactConnection,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ChatInfo {
    pub fn make_direct(contact: Contact) -> Self {
        Self::Direct {
            contact,
            undocumented: Default::default(),
        }
    }

    pub fn make_group(group_info: GroupInfo, group_chat_scope: Option<GroupChatScopeInfo>) -> Self {
        Self::Group {
            group_info,
            group_chat_scope,
            undocumented: Default::default(),
        }
    }

    pub fn make_local(note_folder: NoteFolder) -> Self {
        Self::Local {
            note_folder,
            undocumented: Default::default(),
        }
    }

    pub fn make_contact_request(contact_request: UserContactRequest) -> Self {
        Self::ContactRequest {
            contact_request,
            undocumented: Default::default(),
        }
    }

    pub fn make_contact_connection(contact_connection: PendingContactConnection) -> Self {
        Self::ContactConnection {
            contact_connection,
            undocumented: Default::default(),
        }
    }
}

impl ChatInfo {
    pub fn direct(&self) -> Option<&Contact> {
        if let Self::Direct { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn group(&self) -> Option<ChatInfoGroupRef<'_>> {
        if let Self::Group {
            group_info,
            group_chat_scope,
            ..
        } = self
        {
            Some(ChatInfoGroupRef {
                group_info,
                group_chat_scope,
            })
        } else {
            None
        }
    }
    pub fn local(&self) -> Option<&NoteFolder> {
        if let Self::Local { note_folder, .. } = self {
            Some(note_folder)
        } else {
            None
        }
    }
    pub fn contact_request(&self) -> Option<&UserContactRequest> {
        if let Self::ContactRequest {
            contact_request, ..
        } = self
        {
            Some(contact_request)
        } else {
            None
        }
    }
    pub fn contact_connection(&self) -> Option<&PendingContactConnection> {
        if let Self::ContactConnection {
            contact_connection, ..
        } = self
        {
            Some(contact_connection)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct ChatInfoGroupRef<'a> {
    pub group_info: &'a GroupInfo,
    pub group_chat_scope: &'a Option<GroupChatScopeInfo>,
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
    pub mentions: BTreeMap<String, CIMention>,

    #[serde(rename = "formattedText", skip_serializing_if = "Option::is_none")]
    pub formatted_text: Option<Vec<FormattedText>>,

    #[serde(rename = "quotedItem", skip_serializing_if = "Option::is_none")]
    pub quoted_item: Option<CIQuote>,

    #[serde(rename = "reactions")]
    pub reactions: Vec<CIReactionCount>,

    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<CIFile>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

impl CommandSyntax for ChatRef {
    const COMMAND_BUF_SIZE: usize = 256;

    fn append_command_syntax(&self, buf: &mut String) {
        self.chat_type.append_command_syntax(buf);
        write!(buf, "{}", self.chat_id).unwrap();
        if let Some(chat_scope) = &self.chat_scope {
            chat_scope.append_command_syntax(buf);
        }
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

    #[serde(rename = "favorite", default)]
    pub favorite: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "unreadChat", default)]
    pub unread_chat: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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
    const COMMAND_BUF_SIZE: usize = 16;

    fn append_command_syntax(&self, buf: &mut String) {
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ClientNotice {
    #[serde(
        rename = "ttl",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub ttl: Option<i64>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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
pub struct CommentsGroupPreference {
    #[serde(rename = "enable")]
    pub enable: GroupFeatureEnabled,

    #[serde(
        rename = "duration",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub duration: Option<i32>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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
    pub mentions: BTreeMap<String, i64>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnStatus {
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
    #[serde(rename = "sndReady")]
    SndReady,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "failed")]
    Failed {
        #[serde(rename = "connError")]
        conn_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ConnStatus {
    pub fn make_new() -> Self {
        Self::New
    }

    pub fn make_prepared() -> Self {
        Self::Prepared
    }

    pub fn make_joined() -> Self {
        Self::Joined
    }

    pub fn make_requested() -> Self {
        Self::Requested
    }

    pub fn make_accepted() -> Self {
        Self::Accepted
    }

    pub fn make_snd_ready() -> Self {
        Self::SndReady
    }

    pub fn make_ready() -> Self {
        Self::Ready
    }

    pub fn make_deleted() -> Self {
        Self::Deleted
    }

    pub fn make_failed(conn_error: String) -> Self {
        Self::Failed {
            conn_error,
            undocumented: Default::default(),
        }
    }
}

impl ConnStatus {
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New)
    }
    pub fn is_prepared(&self) -> bool {
        matches!(self, Self::Prepared)
    }
    pub fn is_joined(&self) -> bool {
        matches!(self, Self::Joined)
    }
    pub fn is_requested(&self) -> bool {
        matches!(self, Self::Requested)
    }
    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }
    pub fn is_snd_ready(&self) -> bool {
        matches!(self, Self::SndReady)
    }
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }
    pub fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }
    pub fn failed(&self) -> Option<&String> {
        if let Self::Failed { conn_error, .. } = self {
            Some(conn_error)
        } else {
            None
        }
    }
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

    #[serde(rename = "viaGroupLink", default)]
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

    #[serde(rename = "contactConnInitiated", default)]
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

    #[serde(rename = "pqSupport", default)]
    pub pq_support: bool,

    #[serde(rename = "pqEncryption", default)]
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvGroupMsgConnection")]
    RcvGroupMsgConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "groupMember")]
        group_member: GroupMember,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userContactConnection")]
    UserContactConnection {
        #[serde(rename = "entityConnection")]
        entity_connection: Connection,

        #[serde(rename = "userContact")]
        user_contact: UserContact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ConnectionEntity {
    pub fn make_rcv_direct_msg_connection(
        entity_connection: Connection,
        contact: Option<Contact>,
    ) -> Self {
        Self::RcvDirectMsgConnection {
            entity_connection,
            contact,
            undocumented: Default::default(),
        }
    }

    pub fn make_rcv_group_msg_connection(
        entity_connection: Connection,
        group_info: GroupInfo,
        group_member: GroupMember,
    ) -> Self {
        Self::RcvGroupMsgConnection {
            entity_connection,
            group_info,
            group_member,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_contact_connection(
        entity_connection: Connection,
        user_contact: UserContact,
    ) -> Self {
        Self::UserContactConnection {
            entity_connection,
            user_contact,
            undocumented: Default::default(),
        }
    }
}

impl ConnectionEntity {
    pub fn rcv_direct_msg_connection(
        &self,
    ) -> Option<ConnectionEntityRcvDirectMsgConnectionRef<'_>> {
        if let Self::RcvDirectMsgConnection {
            entity_connection,
            contact,
            ..
        } = self
        {
            Some(ConnectionEntityRcvDirectMsgConnectionRef {
                entity_connection,
                contact,
            })
        } else {
            None
        }
    }
    pub fn rcv_group_msg_connection(&self) -> Option<ConnectionEntityRcvGroupMsgConnectionRef<'_>> {
        if let Self::RcvGroupMsgConnection {
            entity_connection,
            group_info,
            group_member,
            ..
        } = self
        {
            Some(ConnectionEntityRcvGroupMsgConnectionRef {
                entity_connection,
                group_info,
                group_member,
            })
        } else {
            None
        }
    }
    pub fn user_contact_connection(&self) -> Option<ConnectionEntityUserContactConnectionRef<'_>> {
        if let Self::UserContactConnection {
            entity_connection,
            user_contact,
            ..
        } = self
        {
            Some(ConnectionEntityUserContactConnectionRef {
                entity_connection,
                user_contact,
            })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct ConnectionEntityRcvDirectMsgConnectionRef<'a> {
    pub entity_connection: &'a Connection,
    pub contact: &'a Option<Contact>,
}
#[derive(Clone, Copy)]
pub struct ConnectionEntityRcvGroupMsgConnectionRef<'a> {
    pub entity_connection: &'a Connection,
    pub group_info: &'a GroupInfo,
    pub group_member: &'a GroupMember,
}
#[derive(Clone, Copy)]
pub struct ConnectionEntityUserContactConnectionRef<'a> {
    pub entity_connection: &'a Connection,
    pub user_contact: &'a UserContact,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactAddress")]
    ContactAddress {
        #[serde(rename = "contactAddressPlan")]
        contact_address_plan: ContactAddressPlan,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupLink")]
    GroupLink {
        #[serde(rename = "groupLinkPlan")]
        group_link_plan: GroupLinkPlan,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "chatError")]
        chat_error: ChatError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ConnectionPlan {
    pub fn make_invitation_link(invitation_link_plan: InvitationLinkPlan) -> Self {
        Self::InvitationLink {
            invitation_link_plan,
            undocumented: Default::default(),
        }
    }

    pub fn make_contact_address(contact_address_plan: ContactAddressPlan) -> Self {
        Self::ContactAddress {
            contact_address_plan,
            undocumented: Default::default(),
        }
    }

    pub fn make_group_link(group_link_plan: GroupLinkPlan) -> Self {
        Self::GroupLink {
            group_link_plan,
            undocumented: Default::default(),
        }
    }

    pub fn make_error(chat_error: ChatError) -> Self {
        Self::Error {
            chat_error,
            undocumented: Default::default(),
        }
    }
}

impl ConnectionPlan {
    pub fn invitation_link(&self) -> Option<&InvitationLinkPlan> {
        if let Self::InvitationLink {
            invitation_link_plan,
            ..
        } = self
        {
            Some(invitation_link_plan)
        } else {
            None
        }
    }
    pub fn contact_address(&self) -> Option<&ContactAddressPlan> {
        if let Self::ContactAddress {
            contact_address_plan,
            ..
        } = self
        {
            Some(contact_address_plan)
        } else {
            None
        }
    }
    pub fn group_link(&self) -> Option<&GroupLinkPlan> {
        if let Self::GroupLink {
            group_link_plan, ..
        } = self
        {
            Some(group_link_plan)
        } else {
            None
        }
    }
    pub fn error(&self) -> Option<&ChatError> {
        if let Self::Error { chat_error, .. } = self {
            Some(chat_error)
        } else {
            None
        }
    }
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

    #[serde(rename = "contactUsed", default)]
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

    #[serde(rename = "contactGrpInvSent", default)]
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

    #[serde(rename = "chatDeleted", default)]
    pub chat_deleted: bool,

    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<JsonObject>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContactAddressPlan {
    #[serde(rename = "ok")]
    Ok {
        #[serde(rename = "contactSLinkData_", skip_serializing_if = "Option::is_none")]
        contact_s_link_data: Option<ContactShortLinkData>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "ownLink")]
    OwnLink,
    #[serde(rename = "connectingConfirmReconnect")]
    ConnectingConfirmReconnect,
    #[serde(rename = "connectingProhibit")]
    ConnectingProhibit {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "known")]
    Known {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactViaAddress")]
    ContactViaAddress {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ContactAddressPlan {
    pub fn make_ok(contact_s_link_data: Option<ContactShortLinkData>) -> Self {
        Self::Ok {
            contact_s_link_data,
            undocumented: Default::default(),
        }
    }

    pub fn make_own_link() -> Self {
        Self::OwnLink
    }

    pub fn make_connecting_confirm_reconnect() -> Self {
        Self::ConnectingConfirmReconnect
    }

    pub fn make_connecting_prohibit(contact: Contact) -> Self {
        Self::ConnectingProhibit {
            contact,
            undocumented: Default::default(),
        }
    }

    pub fn make_known(contact: Contact) -> Self {
        Self::Known {
            contact,
            undocumented: Default::default(),
        }
    }

    pub fn make_contact_via_address(contact: Contact) -> Self {
        Self::ContactViaAddress {
            contact,
            undocumented: Default::default(),
        }
    }
}

impl ContactAddressPlan {
    pub fn ok(&self) -> Option<&Option<ContactShortLinkData>> {
        if let Self::Ok {
            contact_s_link_data,
            ..
        } = self
        {
            Some(contact_s_link_data)
        } else {
            None
        }
    }
    pub fn is_own_link(&self) -> bool {
        matches!(self, Self::OwnLink)
    }
    pub fn is_connecting_confirm_reconnect(&self) -> bool {
        matches!(self, Self::ConnectingConfirmReconnect)
    }
    pub fn connecting_prohibit(&self) -> Option<&Contact> {
        if let Self::ConnectingProhibit { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn known(&self) -> Option<&Contact> {
        if let Self::Known { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn contact_via_address(&self) -> Option<&Contact> {
        if let Self::ContactViaAddress { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactShortLinkData {
    #[serde(rename = "profile")]
    pub profile: Profile,

    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<MsgContent>,

    #[serde(rename = "business", default)]
    pub business: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "user")]
    User {
        #[serde(rename = "preference")]
        preference: SimplePreference,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ContactUserPref {
    pub fn make_contact(preference: SimplePreference) -> Self {
        Self::Contact {
            preference,
            undocumented: Default::default(),
        }
    }

    pub fn make_user(preference: SimplePreference) -> Self {
        Self::User {
            preference,
            undocumented: Default::default(),
        }
    }
}

impl ContactUserPref {
    pub fn contact(&self) -> Option<&SimplePreference> {
        if let Self::Contact { preference, .. } = self {
            Some(preference)
        } else {
            None
        }
    }
    pub fn user(&self) -> Option<&SimplePreference> {
        if let Self::User { preference, .. } = self {
            Some(preference)
        } else {
            None
        }
    }
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

impl CommandSyntax for CreatedConnLink {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        write!(buf, "{}", self.conn_full_link).unwrap();
        if let Some(conn_short_link) = &self.conn_short_link {
            buf.push(' ');
            write!(buf, "{}", conn_short_link).unwrap();
        }
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CryptoFileArgs {
    #[serde(rename = "fileKey")]
    pub file_key: String,

    #[serde(rename = "fileNonce")]
    pub file_nonce: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct DroppedMsg {
    #[serde(rename = "brokerTs")]
    pub broker_ts: UtcTime,

    #[serde(
        rename = "attempts",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub attempts: i32,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct E2EInfo {
    #[serde(rename = "pqEnabled", skip_serializing_if = "Option::is_none")]
    pub pq_enabled: Option<bool>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "fileDescrComplete", default)]
    pub file_descr_complete: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "cancelled", default)]
    pub cancelled: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "colored")]
    Colored {
        #[serde(rename = "color")]
        color: Color,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "hyperLink")]
    HyperLink {
        #[serde(rename = "showText", skip_serializing_if = "Option::is_none")]
        show_text: Option<String>,

        #[serde(rename = "linkUri")]
        link_uri: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "command")]
    Command {
        #[serde(rename = "commandStr")]
        command_str: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "mention")]
    Mention {
        #[serde(rename = "memberName")]
        member_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl Format {
    pub fn make_bold() -> Self {
        Self::Bold
    }

    pub fn make_italic() -> Self {
        Self::Italic
    }

    pub fn make_strike_through() -> Self {
        Self::StrikeThrough
    }

    pub fn make_snippet() -> Self {
        Self::Snippet
    }

    pub fn make_secret() -> Self {
        Self::Secret
    }

    pub fn make_small() -> Self {
        Self::Small
    }

    pub fn make_colored(color: Color) -> Self {
        Self::Colored {
            color,
            undocumented: Default::default(),
        }
    }

    pub fn make_uri() -> Self {
        Self::Uri
    }

    pub fn make_hyper_link(show_text: Option<String>, link_uri: String) -> Self {
        Self::HyperLink {
            show_text,
            link_uri,
            undocumented: Default::default(),
        }
    }

    pub fn make_simplex_link(
        show_text: Option<String>,
        link_type: SimplexLinkType,
        simplex_uri: String,
        smp_hosts: Vec<String>,
    ) -> Self {
        Self::SimplexLink {
            show_text,
            link_type,
            simplex_uri,
            smp_hosts,
            undocumented: Default::default(),
        }
    }

    pub fn make_command(command_str: String) -> Self {
        Self::Command {
            command_str,
            undocumented: Default::default(),
        }
    }

    pub fn make_mention(member_name: String) -> Self {
        Self::Mention {
            member_name,
            undocumented: Default::default(),
        }
    }

    pub fn make_email() -> Self {
        Self::Email
    }

    pub fn make_phone() -> Self {
        Self::Phone
    }
}

impl Format {
    pub fn is_bold(&self) -> bool {
        matches!(self, Self::Bold)
    }
    pub fn is_italic(&self) -> bool {
        matches!(self, Self::Italic)
    }
    pub fn is_strike_through(&self) -> bool {
        matches!(self, Self::StrikeThrough)
    }
    pub fn is_snippet(&self) -> bool {
        matches!(self, Self::Snippet)
    }
    pub fn is_secret(&self) -> bool {
        matches!(self, Self::Secret)
    }
    pub fn is_small(&self) -> bool {
        matches!(self, Self::Small)
    }
    pub fn colored(&self) -> Option<&Color> {
        if let Self::Colored { color, .. } = self {
            Some(color)
        } else {
            None
        }
    }
    pub fn is_uri(&self) -> bool {
        matches!(self, Self::Uri)
    }
    pub fn hyper_link(&self) -> Option<FormatHyperLinkRef<'_>> {
        if let Self::HyperLink {
            show_text,
            link_uri,
            ..
        } = self
        {
            Some(FormatHyperLinkRef {
                show_text,
                link_uri,
            })
        } else {
            None
        }
    }
    pub fn simplex_link(&self) -> Option<FormatSimplexLinkRef<'_>> {
        if let Self::SimplexLink {
            show_text,
            link_type,
            simplex_uri,
            smp_hosts,
            ..
        } = self
        {
            Some(FormatSimplexLinkRef {
                show_text,
                link_type,
                simplex_uri,
                smp_hosts,
            })
        } else {
            None
        }
    }
    pub fn command(&self) -> Option<&String> {
        if let Self::Command { command_str, .. } = self {
            Some(command_str)
        } else {
            None
        }
    }
    pub fn mention(&self) -> Option<&String> {
        if let Self::Mention { member_name, .. } = self {
            Some(member_name)
        } else {
            None
        }
    }
    pub fn is_email(&self) -> bool {
        matches!(self, Self::Email)
    }
    pub fn is_phone(&self) -> bool {
        matches!(self, Self::Phone)
    }
}
#[derive(Clone, Copy)]
pub struct FormatHyperLinkRef<'a> {
    pub show_text: &'a Option<String>,
    pub link_uri: &'a String,
}
#[derive(Clone, Copy)]
pub struct FormatSimplexLinkRef<'a> {
    pub show_text: &'a Option<String>,
    pub link_type: &'a SimplexLinkType,
    pub simplex_uri: &'a String,
    pub smp_hosts: &'a Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct FormattedText {
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "comments")]
    pub comments: CommentsGroupPreference,

    #[serde(rename = "commands")]
    pub commands: Vec<ChatBotCommand>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct Group {
    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(rename = "members")]
    pub members: Vec<GroupMember>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl CommandSyntax for GroupChatScope {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("(_support");
        match self {
            Self::MemberSupport {
                group_member_id, ..
            } => {
                if let Some(group_member_id) = group_member_id {
                    buf.push(':');
                    write!(buf, "{}", group_member_id).unwrap();
                }
            }
            Self::Undocumented(_) => {}
        }
        buf.push(')');
    }
}

impl GroupChatScope {
    pub fn make_member_support(group_member_id: Option<i64>) -> Self {
        Self::MemberSupport {
            group_member_id,
            undocumented: Default::default(),
        }
    }
}

impl GroupChatScope {
    pub fn member_support(&self) -> Option<&Option<i64>> {
        if let Self::MemberSupport {
            group_member_id, ..
        } = self
        {
            Some(group_member_id)
        } else {
            None
        }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl GroupChatScopeInfo {
    pub fn make_member_support(group_member: Option<GroupMember>) -> Self {
        Self::MemberSupport {
            group_member,
            undocumented: Default::default(),
        }
    }
}

impl GroupChatScopeInfo {
    pub fn member_support(&self) -> Option<&Option<GroupMember>> {
        if let Self::MemberSupport { group_member, .. } = self {
            Some(group_member)
        } else {
            None
        }
    }
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

    #[serde(rename = "groupDirectInvStartedConnection", default)]
    pub group_direct_inv_started_connection: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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
    #[serde(rename = "comments")]
    Comments,
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

    #[serde(rename = "useRelays", default)]
    pub use_relays: bool,

    #[serde(rename = "relayOwnStatus", skip_serializing_if = "Option::is_none")]
    pub relay_own_status: Option<RelayStatus>,

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

    #[serde(rename = "groupSummary")]
    pub group_summary: GroupSummary,

    #[serde(
        rename = "membersRequireAttention",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub members_require_attention: i32,

    #[serde(rename = "viaGroupLinkUri", skip_serializing_if = "Option::is_none")]
    pub via_group_link_uri: Option<String>,

    #[serde(rename = "groupKeys", skip_serializing_if = "Option::is_none")]
    pub group_keys: Option<GroupKeys>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupKeys {
    #[serde(rename = "publicGroupId")]
    pub public_group_id: String,

    #[serde(rename = "groupRootKey")]
    pub group_root_key: GroupRootKey,

    #[serde(rename = "memberPrivKey")]
    pub member_priv_key: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "shortLinkDataSet", default)]
    pub short_link_data_set: bool,

    #[serde(rename = "shortLinkLargeDataSet", default)]
    pub short_link_large_data_set: bool,

    #[serde(rename = "groupLinkId")]
    pub group_link_id: String,

    #[serde(rename = "acceptMemberRole")]
    pub accept_member_role: GroupMemberRole,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GroupLinkPlan {
    #[serde(rename = "ok")]
    Ok {
        #[serde(rename = "groupSLinkInfo_", skip_serializing_if = "Option::is_none")]
        group_s_link_info: Option<GroupShortLinkInfo>,

        #[serde(rename = "groupSLinkData_", skip_serializing_if = "Option::is_none")]
        group_s_link_data: Option<GroupShortLinkData>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "ownLink")]
    OwnLink {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connectingConfirmReconnect")]
    ConnectingConfirmReconnect,
    #[serde(rename = "connectingProhibit")]
    ConnectingProhibit {
        #[serde(rename = "groupInfo_", skip_serializing_if = "Option::is_none")]
        group_info: Option<GroupInfo>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "known")]
    Known {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl GroupLinkPlan {
    pub fn make_ok(
        group_s_link_info: Option<GroupShortLinkInfo>,
        group_s_link_data: Option<GroupShortLinkData>,
    ) -> Self {
        Self::Ok {
            group_s_link_info,
            group_s_link_data,
            undocumented: Default::default(),
        }
    }

    pub fn make_own_link(group_info: GroupInfo) -> Self {
        Self::OwnLink {
            group_info,
            undocumented: Default::default(),
        }
    }

    pub fn make_connecting_confirm_reconnect() -> Self {
        Self::ConnectingConfirmReconnect
    }

    pub fn make_connecting_prohibit(group_info: Option<GroupInfo>) -> Self {
        Self::ConnectingProhibit {
            group_info,
            undocumented: Default::default(),
        }
    }

    pub fn make_known(group_info: GroupInfo) -> Self {
        Self::Known {
            group_info,
            undocumented: Default::default(),
        }
    }
}

impl GroupLinkPlan {
    pub fn ok(&self) -> Option<GroupLinkPlanOkRef<'_>> {
        if let Self::Ok {
            group_s_link_info,
            group_s_link_data,
            ..
        } = self
        {
            Some(GroupLinkPlanOkRef {
                group_s_link_info,
                group_s_link_data,
            })
        } else {
            None
        }
    }
    pub fn own_link(&self) -> Option<&GroupInfo> {
        if let Self::OwnLink { group_info, .. } = self {
            Some(group_info)
        } else {
            None
        }
    }
    pub fn is_connecting_confirm_reconnect(&self) -> bool {
        matches!(self, Self::ConnectingConfirmReconnect)
    }
    pub fn connecting_prohibit(&self) -> Option<&Option<GroupInfo>> {
        if let Self::ConnectingProhibit { group_info, .. } = self {
            Some(group_info)
        } else {
            None
        }
    }
    pub fn known(&self) -> Option<&GroupInfo> {
        if let Self::Known { group_info, .. } = self {
            Some(group_info)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct GroupLinkPlanOkRef<'a> {
    pub group_s_link_info: &'a Option<GroupShortLinkInfo>,
    pub group_s_link_data: &'a Option<GroupShortLinkData>,
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

    #[serde(
        rename = "indexInGroup",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub index_in_group: i64,

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

    #[serde(rename = "blockedByAdmin", default)]
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

    #[serde(rename = "memberPubKey", skip_serializing_if = "Option::is_none")]
    pub member_pub_key: Option<String>,

    #[serde(rename = "relayLink", skip_serializing_if = "Option::is_none")]
    pub relay_link: Option<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMemberAdmission {
    #[serde(rename = "review", skip_serializing_if = "Option::is_none")]
    pub review: Option<MemberCriteria>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupMemberRole {
    #[default]
    #[serde(rename = "relay")]
    Relay,
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
    #[serde(rename = "showMessages", default)]
    pub show_messages: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<CommentsGroupPreference>,

    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<ChatBotCommand>>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "publicGroup", skip_serializing_if = "Option::is_none")]
    pub public_group: Option<PublicGroupProfile>,

    #[serde(rename = "groupPreferences", skip_serializing_if = "Option::is_none")]
    pub group_preferences: Option<GroupPreferences>,

    #[serde(rename = "memberAdmission", skip_serializing_if = "Option::is_none")]
    pub member_admission: Option<GroupMemberAdmission>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupRelay {
    #[serde(
        rename = "groupRelayId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_relay_id: i64,

    #[serde(
        rename = "groupMemberId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub group_member_id: i64,

    #[serde(rename = "userChatRelay")]
    pub user_chat_relay: UserChatRelay,

    #[serde(rename = "relayStatus")]
    pub relay_status: RelayStatus,

    #[serde(rename = "relayLink", skip_serializing_if = "Option::is_none")]
    pub relay_link: Option<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GroupRootKey {
    #[serde(rename = "private")]
    Private {
        #[serde(rename = "rootPrivKey")]
        root_priv_key: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "public")]
    Public {
        #[serde(rename = "rootPubKey")]
        root_pub_key: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl GroupRootKey {
    pub fn make_private(root_priv_key: String) -> Self {
        Self::Private {
            root_priv_key,
            undocumented: Default::default(),
        }
    }

    pub fn make_public(root_pub_key: String) -> Self {
        Self::Public {
            root_pub_key,
            undocumented: Default::default(),
        }
    }
}

impl GroupRootKey {
    pub fn private(&self) -> Option<&String> {
        if let Self::Private { root_priv_key, .. } = self {
            Some(root_priv_key)
        } else {
            None
        }
    }
    pub fn public(&self) -> Option<&String> {
        if let Self::Public { root_pub_key, .. } = self {
            Some(root_pub_key)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupShortLinkData {
    #[serde(rename = "groupProfile")]
    pub group_profile: GroupProfile,

    #[serde(rename = "publicGroupData", skip_serializing_if = "Option::is_none")]
    pub public_group_data: Option<PublicGroupData>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupShortLinkInfo {
    #[serde(rename = "direct", default)]
    pub direct: bool,

    #[serde(rename = "groupRelays")]
    pub group_relays: Vec<String>,

    #[serde(rename = "publicGroupId", skip_serializing_if = "Option::is_none")]
    pub public_group_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupSummary {
    #[serde(
        rename = "currentMembers",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub current_members: i64,

    #[serde(
        rename = "publicMemberCount",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_number_from_string",
        default
    )]
    pub public_member_count: Option<i64>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GroupType {
    #[default]
    #[serde(rename = "channel")]
    Channel,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "ownLink")]
    OwnLink,
    #[serde(rename = "connecting")]
    Connecting {
        #[serde(rename = "contact_", skip_serializing_if = "Option::is_none")]
        contact: Option<Contact>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "known")]
    Known {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl InvitationLinkPlan {
    pub fn make_ok(contact_s_link_data: Option<ContactShortLinkData>) -> Self {
        Self::Ok {
            contact_s_link_data,
            undocumented: Default::default(),
        }
    }

    pub fn make_own_link() -> Self {
        Self::OwnLink
    }

    pub fn make_connecting(contact: Option<Contact>) -> Self {
        Self::Connecting {
            contact,
            undocumented: Default::default(),
        }
    }

    pub fn make_known(contact: Contact) -> Self {
        Self::Known {
            contact,
            undocumented: Default::default(),
        }
    }
}

impl InvitationLinkPlan {
    pub fn ok(&self) -> Option<&Option<ContactShortLinkData>> {
        if let Self::Ok {
            contact_s_link_data,
            ..
        } = self
        {
            Some(contact_s_link_data)
        } else {
            None
        }
    }
    pub fn is_own_link(&self) -> bool {
        matches!(self, Self::OwnLink)
    }
    pub fn connecting(&self) -> Option<&Option<Contact>> {
        if let Self::Connecting { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn known(&self) -> Option<&Contact> {
        if let Self::Known { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "user")]
    User,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl InvitedBy {
    pub fn make_contact(by_contact_id: i64) -> Self {
        Self::Contact {
            by_contact_id,
            undocumented: Default::default(),
        }
    }

    pub fn make_user() -> Self {
        Self::User
    }

    pub fn make_unknown() -> Self {
        Self::Unknown
    }
}

impl InvitedBy {
    pub fn contact(&self) -> Option<&i64> {
        if let Self::Contact { by_contact_id, .. } = self {
            Some(by_contact_id)
        } else {
            None
        }
    }
    pub fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "unknown")]
    Unknown {
        #[serde(rename = "tag")]
        tag: String,

        #[serde(rename = "json")]
        json: JsonObject,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl LinkContent {
    pub fn make_page() -> Self {
        Self::Page
    }

    pub fn make_image() -> Self {
        Self::Image
    }

    pub fn make_video(duration: Option<i32>) -> Self {
        Self::Video {
            duration,
            undocumented: Default::default(),
        }
    }

    pub fn make_unknown(tag: String, json: JsonObject) -> Self {
        Self::Unknown {
            tag,
            json,
            undocumented: Default::default(),
        }
    }
}

impl LinkContent {
    pub fn is_page(&self) -> bool {
        matches!(self, Self::Page)
    }
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image)
    }
    pub fn video(&self) -> Option<&Option<i32>> {
        if let Self::Video { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }
    pub fn unknown(&self) -> Option<LinkContentUnknownRef<'_>> {
        if let Self::Unknown { tag, json, .. } = self {
            Some(LinkContentUnknownRef { tag, json })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct LinkContentUnknownRef<'a> {
    pub tag: &'a String,
    pub json: &'a JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(rename = "business", default)]
        business: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invitation")]
    Invitation {
        #[serde(rename = "invLink")]
        inv_link: String,

        #[serde(rename = "profile")]
        profile: Profile,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "connLink")]
        conn_link: String,

        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl MsgChatLink {
    pub fn make_contact(conn_link: String, profile: Profile, business: bool) -> Self {
        Self::Contact {
            conn_link,
            profile,
            business,
            undocumented: Default::default(),
        }
    }

    pub fn make_invitation(inv_link: String, profile: Profile) -> Self {
        Self::Invitation {
            inv_link,
            profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_group(conn_link: String, group_profile: GroupProfile) -> Self {
        Self::Group {
            conn_link,
            group_profile,
            undocumented: Default::default(),
        }
    }
}

impl MsgChatLink {
    pub fn contact(&self) -> Option<MsgChatLinkContactRef<'_>> {
        if let Self::Contact {
            conn_link,
            profile,
            business,
            ..
        } = self
        {
            Some(MsgChatLinkContactRef {
                conn_link,
                profile,
                business,
            })
        } else {
            None
        }
    }
    pub fn invitation(&self) -> Option<MsgChatLinkInvitationRef<'_>> {
        if let Self::Invitation {
            inv_link, profile, ..
        } = self
        {
            Some(MsgChatLinkInvitationRef { inv_link, profile })
        } else {
            None
        }
    }
    pub fn group(&self) -> Option<MsgChatLinkGroupRef<'_>> {
        if let Self::Group {
            conn_link,
            group_profile,
            ..
        } = self
        {
            Some(MsgChatLinkGroupRef {
                conn_link,
                group_profile,
            })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct MsgChatLinkContactRef<'a> {
    pub conn_link: &'a String,
    pub profile: &'a Profile,
    pub business: &'a bool,
}
#[derive(Clone, Copy)]
pub struct MsgChatLinkInvitationRef<'a> {
    pub inv_link: &'a String,
    pub profile: &'a Profile,
}
#[derive(Clone, Copy)]
pub struct MsgChatLinkGroupRef<'a> {
    pub conn_link: &'a String,
    pub group_profile: &'a GroupProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MsgContent {
    #[serde(rename = "text")]
    Text {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "link")]
    Link {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "preview")]
        preview: LinkPreview,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "image")]
    Image {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "image")]
        image: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "file")]
    File {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "report")]
    Report {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "reason")]
        reason: ReportReason,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chat")]
    Chat {
        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "chatLink")]
        chat_link: MsgChatLink,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "unknown")]
    Unknown {
        #[serde(rename = "tag")]
        tag: String,

        #[serde(rename = "text")]
        text: String,

        #[serde(rename = "json")]
        json: JsonObject,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl MsgContent {
    pub fn make_text(text: String) -> Self {
        Self::Text {
            text,
            undocumented: Default::default(),
        }
    }

    pub fn make_link(text: String, preview: LinkPreview) -> Self {
        Self::Link {
            text,
            preview,
            undocumented: Default::default(),
        }
    }

    pub fn make_image(text: String, image: String) -> Self {
        Self::Image {
            text,
            image,
            undocumented: Default::default(),
        }
    }

    pub fn make_video(text: String, image: String, duration: i32) -> Self {
        Self::Video {
            text,
            image,
            duration,
            undocumented: Default::default(),
        }
    }

    pub fn make_voice(text: String, duration: i32) -> Self {
        Self::Voice {
            text,
            duration,
            undocumented: Default::default(),
        }
    }

    pub fn make_file(text: String) -> Self {
        Self::File {
            text,
            undocumented: Default::default(),
        }
    }

    pub fn make_report(text: String, reason: ReportReason) -> Self {
        Self::Report {
            text,
            reason,
            undocumented: Default::default(),
        }
    }

    pub fn make_chat(text: String, chat_link: MsgChatLink) -> Self {
        Self::Chat {
            text,
            chat_link,
            undocumented: Default::default(),
        }
    }

    pub fn make_unknown(tag: String, text: String, json: JsonObject) -> Self {
        Self::Unknown {
            tag,
            text,
            json,
            undocumented: Default::default(),
        }
    }
}

impl MsgContent {
    pub fn text(&self) -> Option<&String> {
        if let Self::Text { text, .. } = self {
            Some(text)
        } else {
            None
        }
    }
    pub fn link(&self) -> Option<MsgContentLinkRef<'_>> {
        if let Self::Link { text, preview, .. } = self {
            Some(MsgContentLinkRef { text, preview })
        } else {
            None
        }
    }
    pub fn image(&self) -> Option<MsgContentImageRef<'_>> {
        if let Self::Image { text, image, .. } = self {
            Some(MsgContentImageRef { text, image })
        } else {
            None
        }
    }
    pub fn video(&self) -> Option<MsgContentVideoRef<'_>> {
        if let Self::Video {
            text,
            image,
            duration,
            ..
        } = self
        {
            Some(MsgContentVideoRef {
                text,
                image,
                duration,
            })
        } else {
            None
        }
    }
    pub fn voice(&self) -> Option<MsgContentVoiceRef<'_>> {
        if let Self::Voice { text, duration, .. } = self {
            Some(MsgContentVoiceRef { text, duration })
        } else {
            None
        }
    }
    pub fn file(&self) -> Option<&String> {
        if let Self::File { text, .. } = self {
            Some(text)
        } else {
            None
        }
    }
    pub fn report(&self) -> Option<MsgContentReportRef<'_>> {
        if let Self::Report { text, reason, .. } = self {
            Some(MsgContentReportRef { text, reason })
        } else {
            None
        }
    }
    pub fn chat(&self) -> Option<MsgContentChatRef<'_>> {
        if let Self::Chat {
            text, chat_link, ..
        } = self
        {
            Some(MsgContentChatRef { text, chat_link })
        } else {
            None
        }
    }
    pub fn unknown(&self) -> Option<MsgContentUnknownRef<'_>> {
        if let Self::Unknown {
            tag, text, json, ..
        } = self
        {
            Some(MsgContentUnknownRef { tag, text, json })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct MsgContentLinkRef<'a> {
    pub text: &'a String,
    pub preview: &'a LinkPreview,
}
#[derive(Clone, Copy)]
pub struct MsgContentImageRef<'a> {
    pub text: &'a String,
    pub image: &'a String,
}
#[derive(Clone, Copy)]
pub struct MsgContentVideoRef<'a> {
    pub text: &'a String,
    pub image: &'a String,
    pub duration: &'a i32,
}
#[derive(Clone, Copy)]
pub struct MsgContentVoiceRef<'a> {
    pub text: &'a String,
    pub duration: &'a i32,
}
#[derive(Clone, Copy)]
pub struct MsgContentReportRef<'a> {
    pub text: &'a String,
    pub reason: &'a ReportReason,
}
#[derive(Clone, Copy)]
pub struct MsgContentChatRef<'a> {
    pub text: &'a String,
    pub chat_link: &'a MsgChatLink,
}
#[derive(Clone, Copy)]
pub struct MsgContentUnknownRef<'a> {
    pub tag: &'a String,
    pub text: &'a String,
    pub json: &'a JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "unknown")]
    Unknown {
        #[serde(rename = "tag")]
        tag: String,

        #[serde(rename = "json")]
        json: JsonObject,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl MsgReaction {
    pub fn make_emoji(emoji: String) -> Self {
        Self::Emoji {
            emoji,
            undocumented: Default::default(),
        }
    }

    pub fn make_unknown(tag: String, json: JsonObject) -> Self {
        Self::Unknown {
            tag,
            json,
            undocumented: Default::default(),
        }
    }
}

impl MsgReaction {
    pub fn emoji(&self) -> Option<&String> {
        if let Self::Emoji { emoji, .. } = self {
            Some(emoji)
        } else {
            None
        }
    }
    pub fn unknown(&self) -> Option<MsgReactionUnknownRef<'_>> {
        if let Self::Unknown { tag, json, .. } = self {
            Some(MsgReactionUnknownRef { tag, json })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct MsgReactionUnknownRef<'a> {
    pub tag: &'a String,
    pub json: &'a JsonObject,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MsgSigStatus {
    #[default]
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "signedNoKey")]
    SignedNoKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewUser {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,

    #[serde(rename = "pastTimestamp", default)]
    pub past_timestamp: bool,

    #[serde(rename = "userChatRelay", default)]
    pub user_chat_relay: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "favorite", default)]
    pub favorite: bool,

    #[serde(rename = "unread", default)]
    pub unread: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "viaContactUri", default)]
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PrefEnabled {
    #[serde(rename = "forUser", default)]
    pub for_user: bool,

    #[serde(rename = "forContact", default)]
    pub for_contact: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PreparedGroup {
    #[serde(rename = "connLinkToConnect")]
    pub conn_link_to_connect: CreatedConnLink,

    #[serde(rename = "connLinkPreparedConnection", default)]
    pub conn_link_prepared_connection: bool,

    #[serde(rename = "connLinkStartedConnection", default)]
    pub conn_link_started_connection: bool,

    #[serde(rename = "welcomeSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub welcome_shared_msg_id: Option<String>,

    #[serde(rename = "requestSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub request_shared_msg_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PublicGroupData {
    #[serde(
        rename = "publicMemberCount",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub public_member_count: i64,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct PublicGroupProfile {
    #[serde(rename = "groupType")]
    pub group_type: GroupType,

    #[serde(rename = "groupLink")]
    pub group_link: String,

    #[serde(rename = "publicGroupId")]
    pub public_group_id: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "ratchetSync")]
    RatchetSync {
        #[serde(rename = "syncStatus")]
        sync_status: RatchetSyncState,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "verificationCodeReset")]
    VerificationCodeReset,
    #[serde(rename = "pqEnabled")]
    PqEnabled {
        #[serde(rename = "enabled", default)]
        enabled: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl RcvConnEvent {
    pub fn make_switch_queue(phase: SwitchPhase) -> Self {
        Self::SwitchQueue {
            phase,
            undocumented: Default::default(),
        }
    }

    pub fn make_ratchet_sync(sync_status: RatchetSyncState) -> Self {
        Self::RatchetSync {
            sync_status,
            undocumented: Default::default(),
        }
    }

    pub fn make_verification_code_reset() -> Self {
        Self::VerificationCodeReset
    }

    pub fn make_pq_enabled(enabled: bool) -> Self {
        Self::PqEnabled {
            enabled,
            undocumented: Default::default(),
        }
    }
}

impl RcvConnEvent {
    pub fn switch_queue(&self) -> Option<&SwitchPhase> {
        if let Self::SwitchQueue { phase, .. } = self {
            Some(phase)
        } else {
            None
        }
    }
    pub fn ratchet_sync(&self) -> Option<&RatchetSyncState> {
        if let Self::RatchetSync { sync_status, .. } = self {
            Some(sync_status)
        } else {
            None
        }
    }
    pub fn is_verification_code_reset(&self) -> bool {
        matches!(self, Self::VerificationCodeReset)
    }
    pub fn pq_enabled(&self) -> Option<&bool> {
        if let Self::PqEnabled { enabled, .. } = self {
            Some(enabled)
        } else {
            None
        }
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupInvLinkReceived")]
    GroupInvLinkReceived {
        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl RcvDirectEvent {
    pub fn make_contact_deleted() -> Self {
        Self::ContactDeleted
    }

    pub fn make_profile_updated(from_profile: Profile, to_profile: Profile) -> Self {
        Self::ProfileUpdated {
            from_profile,
            to_profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_group_inv_link_received(group_profile: GroupProfile) -> Self {
        Self::GroupInvLinkReceived {
            group_profile,
            undocumented: Default::default(),
        }
    }
}

impl RcvDirectEvent {
    pub fn is_contact_deleted(&self) -> bool {
        matches!(self, Self::ContactDeleted)
    }
    pub fn profile_updated(&self) -> Option<RcvDirectEventProfileUpdatedRef<'_>> {
        if let Self::ProfileUpdated {
            from_profile,
            to_profile,
            ..
        } = self
        {
            Some(RcvDirectEventProfileUpdatedRef {
                from_profile,
                to_profile,
            })
        } else {
            None
        }
    }
    pub fn group_inv_link_received(&self) -> Option<&GroupProfile> {
        if let Self::GroupInvLinkReceived { group_profile, .. } = self {
            Some(group_profile)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct RcvDirectEventProfileUpdatedRef<'a> {
    pub from_profile: &'a Profile,
    pub to_profile: &'a Profile,
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

    #[serde(rename = "fileDescrComplete", default)]
    pub file_descr_complete: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RcvFileStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "accepted")]
    Accepted {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connected")]
    Connected {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "complete")]
    Complete {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "cancelled")]
    Cancelled {
        #[serde(rename = "filePath_", skip_serializing_if = "Option::is_none")]
        file_path: Option<String>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl RcvFileStatus {
    pub fn make_new() -> Self {
        Self::New
    }

    pub fn make_accepted(file_path: String) -> Self {
        Self::Accepted {
            file_path,
            undocumented: Default::default(),
        }
    }

    pub fn make_connected(file_path: String) -> Self {
        Self::Connected {
            file_path,
            undocumented: Default::default(),
        }
    }

    pub fn make_complete(file_path: String) -> Self {
        Self::Complete {
            file_path,
            undocumented: Default::default(),
        }
    }

    pub fn make_cancelled(file_path: Option<String>) -> Self {
        Self::Cancelled {
            file_path,
            undocumented: Default::default(),
        }
    }
}

impl RcvFileStatus {
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New)
    }
    pub fn accepted(&self) -> Option<&String> {
        if let Self::Accepted { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn connected(&self) -> Option<&String> {
        if let Self::Connected { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn complete(&self) -> Option<&String> {
        if let Self::Complete { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn cancelled(&self) -> Option<&Option<String>> {
        if let Self::Cancelled { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
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

    #[serde(rename = "cancelled", default)]
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(rename = "blocked", default)]
        blocked: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userRole")]
    UserRole {
        #[serde(rename = "role")]
        role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userDeleted")]
    UserDeleted,
    #[serde(rename = "groupDeleted")]
    GroupDeleted,
    #[serde(rename = "groupUpdated")]
    GroupUpdated {
        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "newMemberPendingReview")]
    NewMemberPendingReview,
    #[serde(rename = "msgBadSignature")]
    MsgBadSignature,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl RcvGroupEvent {
    pub fn make_member_added(group_member_id: i64, profile: Profile) -> Self {
        Self::MemberAdded {
            group_member_id,
            profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_member_connected() -> Self {
        Self::MemberConnected
    }

    pub fn make_member_accepted(group_member_id: i64, profile: Profile) -> Self {
        Self::MemberAccepted {
            group_member_id,
            profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_accepted() -> Self {
        Self::UserAccepted
    }

    pub fn make_member_left() -> Self {
        Self::MemberLeft
    }

    pub fn make_member_role(group_member_id: i64, profile: Profile, role: GroupMemberRole) -> Self {
        Self::MemberRole {
            group_member_id,
            profile,
            role,
            undocumented: Default::default(),
        }
    }

    pub fn make_member_blocked(group_member_id: i64, profile: Profile, blocked: bool) -> Self {
        Self::MemberBlocked {
            group_member_id,
            profile,
            blocked,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_role(role: GroupMemberRole) -> Self {
        Self::UserRole {
            role,
            undocumented: Default::default(),
        }
    }

    pub fn make_member_deleted(group_member_id: i64, profile: Profile) -> Self {
        Self::MemberDeleted {
            group_member_id,
            profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_deleted() -> Self {
        Self::UserDeleted
    }

    pub fn make_group_deleted() -> Self {
        Self::GroupDeleted
    }

    pub fn make_group_updated(group_profile: GroupProfile) -> Self {
        Self::GroupUpdated {
            group_profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_invited_via_group_link() -> Self {
        Self::InvitedViaGroupLink
    }

    pub fn make_member_created_contact() -> Self {
        Self::MemberCreatedContact
    }

    pub fn make_member_profile_updated(from_profile: Profile, to_profile: Profile) -> Self {
        Self::MemberProfileUpdated {
            from_profile,
            to_profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_new_member_pending_review() -> Self {
        Self::NewMemberPendingReview
    }

    pub fn make_msg_bad_signature() -> Self {
        Self::MsgBadSignature
    }
}

impl RcvGroupEvent {
    pub fn member_added(&self) -> Option<RcvGroupEventMemberAddedRef<'_>> {
        if let Self::MemberAdded {
            group_member_id,
            profile,
            ..
        } = self
        {
            Some(RcvGroupEventMemberAddedRef {
                group_member_id,
                profile,
            })
        } else {
            None
        }
    }
    pub fn is_member_connected(&self) -> bool {
        matches!(self, Self::MemberConnected)
    }
    pub fn member_accepted(&self) -> Option<RcvGroupEventMemberAcceptedRef<'_>> {
        if let Self::MemberAccepted {
            group_member_id,
            profile,
            ..
        } = self
        {
            Some(RcvGroupEventMemberAcceptedRef {
                group_member_id,
                profile,
            })
        } else {
            None
        }
    }
    pub fn is_user_accepted(&self) -> bool {
        matches!(self, Self::UserAccepted)
    }
    pub fn is_member_left(&self) -> bool {
        matches!(self, Self::MemberLeft)
    }
    pub fn member_role(&self) -> Option<RcvGroupEventMemberRoleRef<'_>> {
        if let Self::MemberRole {
            group_member_id,
            profile,
            role,
            ..
        } = self
        {
            Some(RcvGroupEventMemberRoleRef {
                group_member_id,
                profile,
                role,
            })
        } else {
            None
        }
    }
    pub fn member_blocked(&self) -> Option<RcvGroupEventMemberBlockedRef<'_>> {
        if let Self::MemberBlocked {
            group_member_id,
            profile,
            blocked,
            ..
        } = self
        {
            Some(RcvGroupEventMemberBlockedRef {
                group_member_id,
                profile,
                blocked,
            })
        } else {
            None
        }
    }
    pub fn user_role(&self) -> Option<&GroupMemberRole> {
        if let Self::UserRole { role, .. } = self {
            Some(role)
        } else {
            None
        }
    }
    pub fn member_deleted(&self) -> Option<RcvGroupEventMemberDeletedRef<'_>> {
        if let Self::MemberDeleted {
            group_member_id,
            profile,
            ..
        } = self
        {
            Some(RcvGroupEventMemberDeletedRef {
                group_member_id,
                profile,
            })
        } else {
            None
        }
    }
    pub fn is_user_deleted(&self) -> bool {
        matches!(self, Self::UserDeleted)
    }
    pub fn is_group_deleted(&self) -> bool {
        matches!(self, Self::GroupDeleted)
    }
    pub fn group_updated(&self) -> Option<&GroupProfile> {
        if let Self::GroupUpdated { group_profile, .. } = self {
            Some(group_profile)
        } else {
            None
        }
    }
    pub fn is_invited_via_group_link(&self) -> bool {
        matches!(self, Self::InvitedViaGroupLink)
    }
    pub fn is_member_created_contact(&self) -> bool {
        matches!(self, Self::MemberCreatedContact)
    }
    pub fn member_profile_updated(&self) -> Option<RcvGroupEventMemberProfileUpdatedRef<'_>> {
        if let Self::MemberProfileUpdated {
            from_profile,
            to_profile,
            ..
        } = self
        {
            Some(RcvGroupEventMemberProfileUpdatedRef {
                from_profile,
                to_profile,
            })
        } else {
            None
        }
    }
    pub fn is_new_member_pending_review(&self) -> bool {
        matches!(self, Self::NewMemberPendingReview)
    }
    pub fn is_msg_bad_signature(&self) -> bool {
        matches!(self, Self::MsgBadSignature)
    }
}
#[derive(Clone, Copy)]
pub struct RcvGroupEventMemberAddedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
}
#[derive(Clone, Copy)]
pub struct RcvGroupEventMemberAcceptedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
}
#[derive(Clone, Copy)]
pub struct RcvGroupEventMemberRoleRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
    pub role: &'a GroupMemberRole,
}
#[derive(Clone, Copy)]
pub struct RcvGroupEventMemberBlockedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
    pub blocked: &'a bool,
}
#[derive(Clone, Copy)]
pub struct RcvGroupEventMemberDeletedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
}
#[derive(Clone, Copy)]
pub struct RcvGroupEventMemberProfileUpdatedRef<'a> {
    pub from_profile: &'a Profile,
    pub to_profile: &'a Profile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RelayProfile {
    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "fullName")]
    pub full_name: String,

    #[serde(rename = "shortDescr", skip_serializing_if = "Option::is_none")]
    pub short_descr: Option<String>,

    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RelayStatus {
    #[default]
    #[serde(rename = "new")]
    New,
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "active")]
    Active,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SecurityCode {
    #[serde(rename = "securityCode")]
    pub security_code: String,

    #[serde(rename = "verifiedAt")]
    pub verified_at: UtcTime,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct SimplePreference {
    #[serde(rename = "allow")]
    pub allow: FeatureAllowed,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "ratchetSync")]
    RatchetSync {
        #[serde(rename = "syncStatus")]
        sync_status: RatchetSyncState,

        #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
        member: Option<GroupMemberRef>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "pqEnabled")]
    PqEnabled {
        #[serde(rename = "enabled", default)]
        enabled: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl SndConnEvent {
    pub fn make_switch_queue(phase: SwitchPhase, member: Option<GroupMemberRef>) -> Self {
        Self::SwitchQueue {
            phase,
            member,
            undocumented: Default::default(),
        }
    }

    pub fn make_ratchet_sync(
        sync_status: RatchetSyncState,
        member: Option<GroupMemberRef>,
    ) -> Self {
        Self::RatchetSync {
            sync_status,
            member,
            undocumented: Default::default(),
        }
    }

    pub fn make_pq_enabled(enabled: bool) -> Self {
        Self::PqEnabled {
            enabled,
            undocumented: Default::default(),
        }
    }
}

impl SndConnEvent {
    pub fn switch_queue(&self) -> Option<SndConnEventSwitchQueueRef<'_>> {
        if let Self::SwitchQueue { phase, member, .. } = self {
            Some(SndConnEventSwitchQueueRef { phase, member })
        } else {
            None
        }
    }
    pub fn ratchet_sync(&self) -> Option<SndConnEventRatchetSyncRef<'_>> {
        if let Self::RatchetSync {
            sync_status,
            member,
            ..
        } = self
        {
            Some(SndConnEventRatchetSyncRef {
                sync_status,
                member,
            })
        } else {
            None
        }
    }
    pub fn pq_enabled(&self) -> Option<&bool> {
        if let Self::PqEnabled { enabled, .. } = self {
            Some(enabled)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct SndConnEventSwitchQueueRef<'a> {
    pub phase: &'a SwitchPhase,
    pub member: &'a Option<GroupMemberRef>,
}
#[derive(Clone, Copy)]
pub struct SndConnEventRatchetSyncRef<'a> {
    pub sync_status: &'a RatchetSyncState,
    pub member: &'a Option<GroupMemberRef>,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(rename = "blocked", default)]
        blocked: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userRole")]
    UserRole {
        #[serde(rename = "role")]
        role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userLeft")]
    UserLeft,
    #[serde(rename = "groupUpdated")]
    GroupUpdated {
        #[serde(rename = "groupProfile")]
        group_profile: GroupProfile,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userPendingReview")]
    UserPendingReview,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl SndGroupEvent {
    pub fn make_member_role(group_member_id: i64, profile: Profile, role: GroupMemberRole) -> Self {
        Self::MemberRole {
            group_member_id,
            profile,
            role,
            undocumented: Default::default(),
        }
    }

    pub fn make_member_blocked(group_member_id: i64, profile: Profile, blocked: bool) -> Self {
        Self::MemberBlocked {
            group_member_id,
            profile,
            blocked,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_role(role: GroupMemberRole) -> Self {
        Self::UserRole {
            role,
            undocumented: Default::default(),
        }
    }

    pub fn make_member_deleted(group_member_id: i64, profile: Profile) -> Self {
        Self::MemberDeleted {
            group_member_id,
            profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_left() -> Self {
        Self::UserLeft
    }

    pub fn make_group_updated(group_profile: GroupProfile) -> Self {
        Self::GroupUpdated {
            group_profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_member_accepted(group_member_id: i64, profile: Profile) -> Self {
        Self::MemberAccepted {
            group_member_id,
            profile,
            undocumented: Default::default(),
        }
    }

    pub fn make_user_pending_review() -> Self {
        Self::UserPendingReview
    }
}

impl SndGroupEvent {
    pub fn member_role(&self) -> Option<SndGroupEventMemberRoleRef<'_>> {
        if let Self::MemberRole {
            group_member_id,
            profile,
            role,
            ..
        } = self
        {
            Some(SndGroupEventMemberRoleRef {
                group_member_id,
                profile,
                role,
            })
        } else {
            None
        }
    }
    pub fn member_blocked(&self) -> Option<SndGroupEventMemberBlockedRef<'_>> {
        if let Self::MemberBlocked {
            group_member_id,
            profile,
            blocked,
            ..
        } = self
        {
            Some(SndGroupEventMemberBlockedRef {
                group_member_id,
                profile,
                blocked,
            })
        } else {
            None
        }
    }
    pub fn user_role(&self) -> Option<&GroupMemberRole> {
        if let Self::UserRole { role, .. } = self {
            Some(role)
        } else {
            None
        }
    }
    pub fn member_deleted(&self) -> Option<SndGroupEventMemberDeletedRef<'_>> {
        if let Self::MemberDeleted {
            group_member_id,
            profile,
            ..
        } = self
        {
            Some(SndGroupEventMemberDeletedRef {
                group_member_id,
                profile,
            })
        } else {
            None
        }
    }
    pub fn is_user_left(&self) -> bool {
        matches!(self, Self::UserLeft)
    }
    pub fn group_updated(&self) -> Option<&GroupProfile> {
        if let Self::GroupUpdated { group_profile, .. } = self {
            Some(group_profile)
        } else {
            None
        }
    }
    pub fn member_accepted(&self) -> Option<SndGroupEventMemberAcceptedRef<'_>> {
        if let Self::MemberAccepted {
            group_member_id,
            profile,
            ..
        } = self
        {
            Some(SndGroupEventMemberAcceptedRef {
                group_member_id,
                profile,
            })
        } else {
            None
        }
    }
    pub fn is_user_pending_review(&self) -> bool {
        matches!(self, Self::UserPendingReview)
    }
}
#[derive(Clone, Copy)]
pub struct SndGroupEventMemberRoleRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
    pub role: &'a GroupMemberRole,
}
#[derive(Clone, Copy)]
pub struct SndGroupEventMemberBlockedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
    pub blocked: &'a bool,
}
#[derive(Clone, Copy)]
pub struct SndGroupEventMemberDeletedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
}
#[derive(Clone, Copy)]
pub struct SndGroupEventMemberAcceptedRef<'a> {
    pub group_member_id: &'a i64,
    pub profile: &'a Profile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SubscriptionStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "removed")]
    Removed {
        #[serde(rename = "subError")]
        sub_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "noSub")]
    NoSub,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl SubscriptionStatus {
    pub fn make_active() -> Self {
        Self::Active
    }

    pub fn make_pending() -> Self {
        Self::Pending
    }

    pub fn make_removed(sub_error: String) -> Self {
        Self::Removed {
            sub_error,
            undocumented: Default::default(),
        }
    }

    pub fn make_no_sub() -> Self {
        Self::NoSub
    }
}

impl SubscriptionStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }
    pub fn removed(&self) -> Option<&String> {
        if let Self::Removed { sub_error, .. } = self {
            Some(sub_error)
        } else {
            None
        }
    }
    pub fn is_no_sub(&self) -> bool {
        matches!(self, Self::NoSub)
    }
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UIThemeEntityOverrides {
    #[serde(rename = "light", skip_serializing_if = "Option::is_none")]
    pub light: Option<UIThemeEntityOverride>,

    #[serde(rename = "dark", skip_serializing_if = "Option::is_none")]
    pub dark: Option<UIThemeEntityOverride>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UpdatedMessage {
    #[serde(rename = "msgContent")]
    pub msg_content: MsgContent,

    #[serde(rename = "mentions")]
    pub mentions: BTreeMap<String, i64>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "activeUser", default)]
    pub active_user: bool,

    #[serde(
        rename = "activeOrder",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub active_order: i64,

    #[serde(rename = "viewPwdHash", skip_serializing_if = "Option::is_none")]
    pub view_pwd_hash: Option<UserPwdHash>,

    #[serde(rename = "showNtfs", default)]
    pub show_ntfs: bool,

    #[serde(rename = "sendRcptsContacts", default)]
    pub send_rcpts_contacts: bool,

    #[serde(rename = "sendRcptsSmallGroups", default)]
    pub send_rcpts_small_groups: bool,

    #[serde(rename = "autoAcceptMemberContacts", default)]
    pub auto_accept_member_contacts: bool,

    #[serde(
        rename = "userMemberProfileUpdatedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_member_profile_updated_at: Option<UtcTime>,

    #[serde(rename = "uiThemes", skip_serializing_if = "Option::is_none")]
    pub ui_themes: Option<UIThemeEntityOverrides>,

    #[serde(rename = "userChatRelay", default)]
    pub user_chat_relay: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserChatRelay {
    #[serde(
        rename = "chatRelayId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub chat_relay_id: i64,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "relayProfile")]
    pub relay_profile: RelayProfile,

    #[serde(rename = "domains")]
    pub domains: Vec<String>,

    #[serde(rename = "preset", default)]
    pub preset: bool,

    #[serde(rename = "tested", skip_serializing_if = "Option::is_none")]
    pub tested: Option<bool>,

    #[serde(rename = "enabled", default)]
    pub enabled: bool,

    #[serde(rename = "deleted", default)]
    pub deleted: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "shortLinkDataSet", default)]
    pub short_link_data_set: bool,

    #[serde(rename = "shortLinkLargeDataSet", default)]
    pub short_link_large_data_set: bool,

    #[serde(rename = "addressSettings")]
    pub address_settings: AddressSettings,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "pqSupport", default)]
    pub pq_support: bool,

    #[serde(rename = "welcomeSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub welcome_shared_msg_id: Option<String>,

    #[serde(rename = "requestSharedMsgId", skip_serializing_if = "Option::is_none")]
    pub request_shared_msg_id: Option<String>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserPwdHash {
    #[serde(rename = "hash")]
    pub hash: String,

    #[serde(rename = "salt")]
    pub salt: String,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct XFTPRcvFile {
    #[serde(rename = "rcvFileDescription")]
    pub rcv_file_description: RcvFileDescr,

    #[serde(rename = "agentRcvFileId", skip_serializing_if = "Option::is_none")]
    pub agent_rcv_file_id: Option<String>,

    #[serde(rename = "agentRcvFileDeleted", default)]
    pub agent_rcv_file_deleted: bool,

    #[serde(rename = "userApprovedRelays", default)]
    pub user_approved_relays: bool,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
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

    #[serde(rename = "agentSndFileDeleted", default)]
    pub agent_snd_file_deleted: bool,

    #[serde(rename = "cryptoArgs", skip_serializing_if = "Option::is_none")]
    pub crypto_args: Option<CryptoFileArgs>,

    #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: JsonObject,
}
