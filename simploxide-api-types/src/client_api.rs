use crate::{commands::*, responses::*, utils::CommandSyntax, *};
use std::future::Future;
use std::sync::Arc;

pub trait ClientApi: Sync {
    type Error;

    fn send_raw(
        &self,
        command: String,
    ) -> impl Future<Output = Result<JsonObject, Self::Error>> + Send;

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
    ) -> impl Future<Output = Result<Arc<ApiCreateMyAddressResponse>, Self::Error>> + Send {
        async move {
            let command = ApiCreateMyAddress { user_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiDeleteMyAddressResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteMyAddress { user_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiShowMyAddressResponse>, Self::Error>> + Send {
        async move {
            let command = ApiShowMyAddress { user_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiSetProfileAddressResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiSetAddressSettingsResponse>, Self::Error>> + Send {
        async move {
            let command = ApiSetAddressSettings { user_id, settings };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiSendMessagesResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiUpdateChatItemResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiDeleteChatItemResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteChatItem {
                chat_ref,
                chat_item_ids,
                delete_mode,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiDeleteMemberChatItemResponse>, Self::Error>> + Send
    {
        async move {
            let command = ApiDeleteMemberChatItem {
                group_id,
                chat_item_ids,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiChatItemReactionResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ReceiveFileResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<CancelFileResponse>, Self::Error>> + Send {
        async move {
            let command = CancelFile { file_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn api_add_member(
        &self,
        group_id: i64,
        contact_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<ApiAddMemberResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAddMember {
                group_id,
                contact_id,
                member_role,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiJoinGroupResponse>, Self::Error>> + Send {
        async move {
            let command = ApiJoinGroup { group_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn api_accept_member(
        &self,
        group_id: i64,
        group_member_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<ApiAcceptMemberResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAcceptMember {
                group_id,
                group_member_id,
                member_role,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn api_members_role(
        &self,
        group_id: i64,
        group_member_ids: Vec<i64>,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<ApiMembersRoleResponse>, Self::Error>> + Send {
        async move {
            let command = ApiMembersRole {
                group_id,
                group_member_ids,
                member_role,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiBlockMembersForAllResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiRemoveMembersResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiLeaveGroupResponse>, Self::Error>> + Send {
        async move {
            let command = ApiLeaveGroup { group_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiListMembersResponse>, Self::Error>> + Send {
        async move {
            let command = ApiListMembers { group_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiNewGroupResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiUpdateGroupProfileResponse>, Self::Error>> + Send {
        async move {
            let command = ApiUpdateGroupProfile {
                group_id,
                group_profile,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn api_create_group_link(
        &self,
        group_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<ApiCreateGroupLinkResponse>, Self::Error>> + Send {
        async move {
            let command = ApiCreateGroupLink {
                group_id,
                member_role,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn api_group_link_member_role(
        &self,
        group_id: i64,
        member_role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<ApiGroupLinkMemberRoleResponse>, Self::Error>> + Send {
        async move {
            let command = ApiGroupLinkMemberRole {
                group_id,
                member_role,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiDeleteGroupLinkResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteGroupLink { group_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiGetGroupLinkResponse>, Self::Error>> + Send {
        async move {
            let command = ApiGetGroupLink { group_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiAddContactResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiConnectPlanResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
        }
    }

    /// ### Connection commands
    ///
    /// These commands may be used to create connections. Most bots do not need to use them - bot users will connect via bot address with auto-accept enabled.
    ///
    /// ----
    ///
    /// Connect via SimpleX link. The link can be 1-time invitation link, contact address or group link
    ///
    /// *Network usage*: interactive.
    ///
    /// *Syntax:*
    ///
    /// ```
    /// /_connect <userId> <str(connLink_)>
    /// ```
    fn api_connect(
        &self,
        command: ApiConnect,
    ) -> impl Future<Output = Result<Arc<ApiConnectResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiAcceptContactResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAcceptContact { contact_req_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiRejectContactResponse>, Self::Error>> + Send {
        async move {
            let command = ApiRejectContact { contact_req_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiListContactsResponse>, Self::Error>> + Send {
        async move {
            let command = ApiListContacts { user_id };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiListGroupsResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiDeleteChatResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteChat {
                chat_ref,
                chat_delete_mode,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn show_active_user(
        &self,
    ) -> impl Future<Output = Result<Arc<ShowActiveUserResponse>, Self::Error>> + Send {
        async move {
            let command = ShowActiveUser {};
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn create_active_user(
        &self,
        new_user: NewUser,
    ) -> impl Future<Output = Result<Arc<CreateActiveUserResponse>, Self::Error>> + Send {
        async move {
            let command = CreateActiveUser { new_user };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn list_users(
        &self,
    ) -> impl Future<Output = Result<Arc<ListUsersResponse>, Self::Error>> + Send {
        async move {
            let command = ListUsers {};
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    fn api_set_active_user(
        &self,
        command: ApiSetActiveUser,
    ) -> impl Future<Output = Result<Arc<ApiSetActiveUserResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiDeleteUserResponse>, Self::Error>> + Send {
        async move {
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiUpdateProfileResponse>, Self::Error>> + Send {
        async move {
            let command = ApiUpdateProfile { user_id, profile };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
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
    ) -> impl Future<Output = Result<Arc<ApiSetContactPrefsResponse>, Self::Error>> + Send {
        async move {
            let command = ApiSetContactPrefs {
                contact_id,
                preferences,
            };
            let response = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            Ok(serde_json::from_value(response).unwrap())
        }
    }
}
