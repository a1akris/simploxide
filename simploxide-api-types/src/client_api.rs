use crate::{commands::*, responses::*, utils::CommandSyntax, *};
use std::future::Future;
use std::sync::Arc;

/// A helper trait to handle different response wrappers
pub trait ExtractResponse<'de, T>: Deserialize<'de> {
    fn extract_response(self) -> Result<T, BadResponseError>;
}

pub trait ClientApiError: From<BadResponseError> + std::error::Error {
    /// If current error is a bad response error return a reference to it
    fn bad_response(&self) -> Option<&BadResponseError>;

    /// If current error is a bad response error return a mut reference to it
    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError>;
}

pub trait ClientApi: Sync {
    type ResponseShape<'de, T>: ExtractResponse<'de, T>
    where
        T: 'de + Deserialize<'de>;
    type Error: ClientApiError;

    fn send_raw(&self, command: String)
    -> impl Future<Output = Result<String, Self::Error>> + Send;

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
    fn api_create_my_address(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Arc<UserContactLinkCreatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiCreateMyAddress { user_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiCreateMyAddressResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_delete_my_address(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Arc<UserContactLinkDeletedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteMyAddress { user_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiDeleteMyAddressResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_show_my_address(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Arc<UserContactLinkResponse>, Self::Error>> + Send {
        async move {
            let command = ApiShowMyAddress { user_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiShowMyAddressResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_set_profile_address(
        &self,
        command: ApiSetProfileAddress,
    ) -> impl Future<Output = Result<Arc<UserProfileUpdatedResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSetProfileAddressResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_set_address_settings(
        &self,
        user_id: i64,
        settings: AddressSettings,
    ) -> impl Future<Output = Result<Arc<UserContactLinkUpdatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiSetAddressSettings { user_id, settings };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSetAddressSettingsResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_send_messages(
        &self,
        command: ApiSendMessages,
    ) -> impl Future<Output = Result<Arc<NewChatItemsResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSendMessagesResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_update_chat_item(
        &self,
        command: ApiUpdateChatItem,
    ) -> impl Future<Output = Result<ApiUpdateChatItemResponse, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiUpdateChatItemResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_delete_chat_item(
        &self,
        chat_ref: ChatRef,
        chat_item_ids: Vec<i64>,
        delete_mode: CIDeleteMode,
    ) -> impl Future<Output = Result<Arc<ChatItemsDeletedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteChatItem {
                chat_ref,
                chat_item_ids,
                delete_mode,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiDeleteChatItemResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_delete_member_chat_item(
        &self,
        group_id: i64,
        chat_item_ids: Vec<i64>,
    ) -> impl Future<Output = Result<Arc<ChatItemsDeletedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteMemberChatItem {
                group_id,
                chat_item_ids,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiDeleteMemberChatItemResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_chat_item_reaction(
        &self,
        command: ApiChatItemReaction,
    ) -> impl Future<Output = Result<Arc<ChatItemReactionResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiChatItemReactionResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn receive_file(
        &self,
        command: ReceiveFile,
    ) -> impl Future<Output = Result<ReceiveFileResponse, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ReceiveFileResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn cancel_file(
        &self,
        file_id: i64,
    ) -> impl Future<Output = Result<CancelFileResponse, Self::Error>> + Send {
        async move {
            let command = CancelFile { file_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, CancelFileResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_add_member(
        &self,
        group_id: i64,
        contact_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<SentGroupInvitationResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAddMember {
                group_id,
                contact_id,
                member_role,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiAddMemberResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_join_group(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<UserAcceptedGroupSentResponse>, Self::Error>> + Send {
        async move {
            let command = ApiJoinGroup { group_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiJoinGroupResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_accept_member(
        &self,
        group_id: i64,
        group_member_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<MemberAcceptedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAcceptMember {
                group_id,
                group_member_id,
                member_role,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiAcceptMemberResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_members_role(
        &self,
        group_id: i64,
        group_member_ids: Vec<i64>,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<MembersRoleUserResponse>, Self::Error>> + Send {
        async move {
            let command = ApiMembersRole {
                group_id,
                group_member_ids,
                member_role,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiMembersRoleResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_block_members_for_all(
        &self,
        command: ApiBlockMembersForAll,
    ) -> impl Future<Output = Result<Arc<MembersBlockedForAllUserResponse>, Self::Error>> + Send
    {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiBlockMembersForAllResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_remove_members(
        &self,
        command: ApiRemoveMembers,
    ) -> impl Future<Output = Result<Arc<UserDeletedMembersResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiRemoveMembersResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_leave_group(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<LeftMemberUserResponse>, Self::Error>> + Send {
        async move {
            let command = ApiLeaveGroup { group_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiLeaveGroupResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_list_members(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<GroupMembersResponse>, Self::Error>> + Send {
        async move {
            let command = ApiListMembers { group_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiListMembersResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_new_group(
        &self,
        command: ApiNewGroup,
    ) -> impl Future<Output = Result<Arc<GroupCreatedResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiNewGroupResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_new_public_group(
        &self,
        command: ApiNewPublicGroup,
    ) -> impl Future<Output = Result<ApiNewPublicGroupResponse, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiNewPublicGroupResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_get_group_relays(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<GroupRelaysResponse>, Self::Error>> + Send {
        async move {
            let command = ApiGetGroupRelays { group_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiGetGroupRelaysResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_update_group_profile(
        &self,
        group_id: i64,
        group_profile: GroupProfile,
    ) -> impl Future<Output = Result<Arc<GroupUpdatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiUpdateGroupProfile {
                group_id,
                group_profile,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiUpdateGroupProfileResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_create_group_link(
        &self,
        group_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<GroupLinkCreatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiCreateGroupLink {
                group_id,
                member_role,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiCreateGroupLinkResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_group_link_member_role(
        &self,
        group_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<GroupLinkResponse>, Self::Error>> + Send {
        async move {
            let command = ApiGroupLinkMemberRole {
                group_id,
                member_role,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiGroupLinkMemberRoleResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_delete_group_link(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<GroupLinkDeletedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteGroupLink { group_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiDeleteGroupLinkResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_get_group_link(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<GroupLinkResponse>, Self::Error>> + Send {
        async move {
            let command = ApiGetGroupLink { group_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiGetGroupLinkResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_add_contact(
        &self,
        command: ApiAddContact,
    ) -> impl Future<Output = Result<Arc<InvitationResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiAddContactResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_connect_plan(
        &self,
        command: ApiConnectPlan,
    ) -> impl Future<Output = Result<Arc<ConnectionPlanResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiConnectPlanResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_connect(
        &self,
        command: ApiConnect,
    ) -> impl Future<Output = Result<ApiConnectResponse, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiConnectResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn connect(
        &self,
        command: Connect,
    ) -> impl Future<Output = Result<ConnectResponse, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ConnectResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_accept_contact(
        &self,
        contact_req_id: i64,
    ) -> impl Future<Output = Result<Arc<AcceptingContactRequestResponse>, Self::Error>> + Send
    {
        async move {
            let command = ApiAcceptContact { contact_req_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiAcceptContactResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_reject_contact(
        &self,
        contact_req_id: i64,
    ) -> impl Future<Output = Result<Arc<ContactRequestRejectedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiRejectContact { contact_req_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiRejectContactResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_list_contacts(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Arc<ContactsListResponse>, Self::Error>> + Send {
        async move {
            let command = ApiListContacts { user_id };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiListContactsResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_list_groups(
        &self,
        command: ApiListGroups,
    ) -> impl Future<Output = Result<Arc<GroupsListResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiListGroupsResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_delete_chat(
        &self,
        chat_ref: ChatRef,
        chat_delete_mode: ChatDeleteMode,
    ) -> impl Future<Output = Result<ApiDeleteChatResponse, Self::Error>> + Send {
        async move {
            let command = ApiDeleteChat {
                chat_ref,
                chat_delete_mode,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiDeleteChatResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_set_group_custom_data(
        &self,
        command: ApiSetGroupCustomData,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSetGroupCustomDataResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_set_contact_custom_data(
        &self,
        command: ApiSetContactCustomData,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSetContactCustomDataResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_set_user_auto_accept_member_contacts(
        &self,
        command: ApiSetUserAutoAcceptMemberContacts,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<
                '_,
                ApiSetUserAutoAcceptMemberContactsResponse,
            > = serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn show_active_user(
        &self,
    ) -> impl Future<Output = Result<Arc<ActiveUserResponse>, Self::Error>> + Send {
        async move {
            let command = ShowActiveUser {};
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ShowActiveUserResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn create_active_user(
        &self,
        new_user: NewUser,
    ) -> impl Future<Output = Result<Arc<ActiveUserResponse>, Self::Error>> + Send {
        async move {
            let command = CreateActiveUser { new_user };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, CreateActiveUserResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn list_users(
        &self,
    ) -> impl Future<Output = Result<Arc<UsersListResponse>, Self::Error>> + Send {
        async move {
            let command = ListUsers {};
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ListUsersResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_set_active_user(
        &self,
        command: ApiSetActiveUser,
    ) -> impl Future<Output = Result<Arc<ActiveUserResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSetActiveUserResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_delete_user(
        &self,
        command: ApiDeleteUser,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiDeleteUserResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn api_update_profile(
        &self,
        user_id: i64,
        profile: Profile,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, Self::Error>> + Send {
        async move {
            let command = ApiUpdateProfile { user_id, profile };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiUpdateProfileResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_set_contact_prefs(
        &self,
        contact_id: i64,
        preferences: Preferences,
    ) -> impl Future<Output = Result<Arc<ContactPrefsUpdatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiSetContactPrefs {
                contact_id,
                preferences,
            };
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiSetContactPrefsResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
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
    fn start_chat(
        &self,
        command: StartChat,
    ) -> impl Future<Output = Result<StartChatResponse, Self::Error>> + Send {
        async move {
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, StartChatResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response)
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
    fn api_stop_chat(
        &self,
    ) -> impl Future<Output = Result<Arc<ChatStoppedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiStopChat {};
            let raw = self.send_raw(command.to_command_string()).await?;
            let response_shape: Self::ResponseShape<'_, ApiStopChatResponse> =
                serde_json::from_str(&raw).map_err(BadResponseError::InvalidJson)?;
            let response = response_shape.extract_response()?;
            Ok(response.into_inner())
        }
    }
}

/// Use this as [`ClientApi::ResponseShape`] to extract web socket responses
#[derive(Serialize, Deserialize)]
pub struct WebSocketResponseShape<T> {
    pub resp: WebSocketResponseShapeInner<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketResponseShapeInner<T> {
    Response(T),
    Error(ChatCmdError),
    Undocumented(JsonObject),
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for WebSocketResponseShape<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        self.resp.extract_response()
    }
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for WebSocketResponseShapeInner<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        match self {
            Self::Response(resp) => Ok(resp),
            Self::Error(err) => Err(BadResponseError::ChatError(
                err.into_inner().chat_error.clone(),
            )),
            Self::Undocumented(json) => Err(BadResponseError::Undocumented(json)),
        }
    }
}

/// Use this as [`ClientApi::ResponseShape`] to extract FFI responses
#[derive(Serialize, Deserialize)]
pub enum FfiResponseShape<T> {
    #[serde(rename = "result")]
    Result(T),

    #[serde(rename = "error")]
    Error(Arc<ChatError>),

    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for FfiResponseShape<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        match self {
            Self::Result(resp) => Ok(resp),
            Self::Error(err) => Err(BadResponseError::ChatError(err)),
            Self::Undocumented(json) => Err(BadResponseError::Undocumented(json)),
        }
    }
}

#[derive(Debug)]
pub enum BadResponseError {
    ChatError(Arc<ChatError>),
    InvalidJson(serde_json::Error),
    Undocumented(JsonObject),
}

impl BadResponseError {
    pub fn chat_error(&self) -> Option<&ChatError> {
        if let Self::ChatError(error) = self {
            Some(error.as_ref())
        } else {
            None
        }
    }

    pub fn invalid_json(&self) -> Option<&serde_json::Error> {
        if let Self::InvalidJson(error) = self {
            Some(error)
        } else {
            None
        }
    }

    pub fn undocumented(&self) -> Option<&JsonObject> {
        if let Self::Undocumented(error) = self {
            Some(error)
        } else {
            None
        }
    }
}

impl std::error::Error for BadResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChatError(error) => Some(error.as_ref()),
            Self::InvalidJson(error) => Some(error),
            Self::Undocumented(_) => None,
        }
    }
}

impl std::fmt::Display for BadResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatError(resp) => writeln!(f, "Bad response:\n{resp:#}"),
            Self::Undocumented(resp) => writeln!(f, "Unexpected response:\n{resp:#}"),
            Self::InvalidJson(err) => writeln!(f, "Invalid JSON:\n{err:#}"),
        }
    }
}

pub enum UndocumentedResponse<T> {
    Documented(T),
    Undocumented(JsonObject),
}

/// If you want to ~~suffer~~ handle undocumented responses you can use this extension trait
/// on client API return values which moves Undocumented from `Err` to `Ok` variant.
///
/// Example:
///
/// ```ignore
///     match client
///         .api_create_my_address(1)
///         .await
///         .allow_undocumented()?
///     {
///         UndocumentedResponse::Documented(resp) => {
///              // Process expected response...
///         }
///         UndocumentedResponse::Undocumented(resp) => {
///             // Do something with the unexpected response...
///         }
///     }
/// }
/// ```
pub trait AllowUndocumentedResponses<T, E> {
    fn allow_undocumented(self) -> Result<UndocumentedResponse<T>, E>;
}

impl<T, E> AllowUndocumentedResponses<T, E> for Result<T, E>
where
    E: ClientApiError,
{
    fn allow_undocumented(self) -> Result<UndocumentedResponse<T>, E> {
        match self {
            Ok(resp) => Ok(UndocumentedResponse::Documented(resp)),
            Err(mut e) => match e.bad_response_mut() {
                Some(BadResponseError::Undocumented(btree_map)) => Ok(
                    UndocumentedResponse::Undocumented(std::mem::take(btree_map)),
                ),
                _ => Err(e),
            },
        }
    }
}
