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
    Undocumented(JsonObject),
}

impl ApiCreateMyAddressResponse {
    pub fn user_contact_link_created(&self) -> Option<&UserContactLinkCreatedResponse> {
        if let Self::UserContactLinkCreated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiDeleteMyAddressResponse {
    pub fn user_contact_link_deleted(&self) -> Option<&UserContactLinkDeletedResponse> {
        if let Self::UserContactLinkDeleted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiShowMyAddressResponse {
    pub fn user_contact_link(&self) -> Option<&UserContactLinkResponse> {
        if let Self::UserContactLink(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiSetProfileAddressResponse {
    pub fn user_profile_updated(&self) -> Option<&UserProfileUpdatedResponse> {
        if let Self::UserProfileUpdated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiSetAddressSettingsResponse {
    pub fn user_contact_link_updated(&self) -> Option<&UserContactLinkUpdatedResponse> {
        if let Self::UserContactLinkUpdated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiSendMessagesResponse {
    pub fn new_chat_items(&self) -> Option<&NewChatItemsResponse> {
        if let Self::NewChatItems(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiDeleteChatItemResponse {
    pub fn chat_items_deleted(&self) -> Option<&ChatItemsDeletedResponse> {
        if let Self::ChatItemsDeleted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiDeleteMemberChatItemResponse {
    pub fn chat_items_deleted(&self) -> Option<&ChatItemsDeletedResponse> {
        if let Self::ChatItemsDeleted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiChatItemReactionResponse {
    pub fn chat_item_reaction(&self) -> Option<&ChatItemReactionResponse> {
        if let Self::ChatItemReaction(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiAddMemberResponse {
    pub fn sent_group_invitation(&self) -> Option<&SentGroupInvitationResponse> {
        if let Self::SentGroupInvitation(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiJoinGroupResponse {
    pub fn user_accepted_group_sent(&self) -> Option<&UserAcceptedGroupSentResponse> {
        if let Self::UserAcceptedGroupSent(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiAcceptMemberResponse {
    pub fn member_accepted(&self) -> Option<&MemberAcceptedResponse> {
        if let Self::MemberAccepted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiMembersRoleResponse {
    pub fn members_role_user(&self) -> Option<&MembersRoleUserResponse> {
        if let Self::MembersRoleUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiBlockMembersForAllResponse {
    pub fn members_blocked_for_all_user(&self) -> Option<&MembersBlockedForAllUserResponse> {
        if let Self::MembersBlockedForAllUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiRemoveMembersResponse {
    pub fn user_deleted_members(&self) -> Option<&UserDeletedMembersResponse> {
        if let Self::UserDeletedMembers(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiLeaveGroupResponse {
    pub fn left_member_user(&self) -> Option<&LeftMemberUserResponse> {
        if let Self::LeftMemberUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiListMembersResponse {
    pub fn group_members(&self) -> Option<&GroupMembersResponse> {
        if let Self::GroupMembers(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiNewGroupResponse {
    pub fn group_created(&self) -> Option<&GroupCreatedResponse> {
        if let Self::GroupCreated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiUpdateGroupProfileResponse {
    pub fn group_updated(&self) -> Option<&GroupUpdatedResponse> {
        if let Self::GroupUpdated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiCreateGroupLinkResponse {
    pub fn group_link_created(&self) -> Option<&GroupLinkCreatedResponse> {
        if let Self::GroupLinkCreated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiGroupLinkMemberRoleResponse {
    pub fn group_link(&self) -> Option<&GroupLinkResponse> {
        if let Self::GroupLink(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiDeleteGroupLinkResponse {
    pub fn group_link_deleted(&self) -> Option<&GroupLinkDeletedResponse> {
        if let Self::GroupLinkDeleted(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiGetGroupLinkResponse {
    pub fn group_link(&self) -> Option<&GroupLinkResponse> {
        if let Self::GroupLink(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiAddContactResponse {
    pub fn invitation(&self) -> Option<&InvitationResponse> {
        if let Self::Invitation(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiConnectPlanResponse {
    pub fn connection_plan(&self) -> Option<&ConnectionPlanResponse> {
        if let Self::ConnectionPlan(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiAcceptContactResponse {
    pub fn accepting_contact_request(&self) -> Option<&AcceptingContactRequestResponse> {
        if let Self::AcceptingContactRequest(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiRejectContactResponse {
    pub fn contact_request_rejected(&self) -> Option<&ContactRequestRejectedResponse> {
        if let Self::ContactRequestRejected(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiListContactsResponse {
    pub fn contacts_list(&self) -> Option<&ContactsListResponse> {
        if let Self::ContactsList(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiListGroupsResponse {
    pub fn groups_list(&self) -> Option<&GroupsListResponse> {
        if let Self::GroupsList(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ShowActiveUserResponse {
    pub fn active_user(&self) -> Option<&ActiveUserResponse> {
        if let Self::ActiveUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl CreateActiveUserResponse {
    pub fn active_user(&self) -> Option<&ActiveUserResponse> {
        if let Self::ActiveUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ListUsersResponse {
    pub fn users_list(&self) -> Option<&UsersListResponse> {
        if let Self::UsersList(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiSetActiveUserResponse {
    pub fn active_user(&self) -> Option<&ActiveUserResponse> {
        if let Self::ActiveUser(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiDeleteUserResponse {
    pub fn cmd_ok(&self) -> Option<&CmdOkResponse> {
        if let Self::CmdOk(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
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

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
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
    Undocumented(JsonObject),
}

impl ApiSetContactPrefsResponse {
    pub fn contact_prefs_updated(&self) -> Option<&ContactPrefsUpdatedResponse> {
        if let Self::ContactPrefsUpdated(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn chat_cmd_error(&self) -> Option<&ChatCmdErrorResponse> {
        if let Self::ChatCmdError(ret) = self {
            Some(ret)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(ret) = self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct AcceptingContactRequestResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ActiveUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatCmdErrorResponse {
    #[serde(rename = "chatError")]
    pub chat_error: ChatError,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemNotChangedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ChatItemUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct CmdOkResponse {
    #[serde(rename = "user_", skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactAlreadyExistsResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactConnectionDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connection")]
    pub connection: PendingContactConnection,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contact")]
    pub contact: Contact,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct ContactsListResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contacts")]
    pub contacts: Vec<Contact>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupDeletedUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupLinkDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupMembersResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "group")]
    pub group: Group,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct GroupsListResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groups")]
    pub groups: Vec<GroupInfoSummary>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct LeftMemberUserResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "groupInfo")]
    pub group_info: GroupInfo,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct NewChatItemsResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItems")]
    pub chat_items: Vec<AChatItem>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAcceptedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "chatItem")]
    pub chat_item: AChatItem,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct RcvFileAcceptedSndCancelledResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "rcvFileTransfer")]
    pub rcv_file_transfer: RcvFileTransfer,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkCreatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "connLinkContact")]
    pub conn_link_contact: CreatedConnLink,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkDeletedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactLink")]
    pub contact_link: UserContactLink,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserContactLinkUpdatedResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "contactLink")]
    pub contact_link: UserContactLink,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UserProfileNoChangeResponse {
    #[serde(rename = "user")]
    pub user: User,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
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

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(String, into)))]
pub struct UsersListResponse {
    #[serde(rename = "users")]
    pub users: Vec<UserInfo>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bon", builder(default))]
    pub undocumented: HashMap<String, JsonObject>,
}
