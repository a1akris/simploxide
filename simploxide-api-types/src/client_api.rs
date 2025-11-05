use crate::{commands::*, responses::*, utils::CommandSyntax, *};
use std::future::Future;
use std::sync::Arc;

pub trait ClientApiError: From<BadResponseError> + std::error::Error {
    /// If current error is a bad response error return a mut reference to it!
    ///
    /// Required for [`AllowUndocumentedResponses`] impl.
    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError>;
}

pub trait ClientApi: Sync {
    type Error: ClientApiError;

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
    ) -> impl Future<Output = Result<Arc<UserContactLinkCreatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiCreateMyAddress { user_id };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiCreateMyAddressResponse::UserContactLinkCreated(resp) => Ok(Arc::new(resp)),
                ApiCreateMyAddressResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiCreateMyAddressResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiDeleteMyAddressResponse::UserContactLinkDeleted(resp) => Ok(Arc::new(resp)),
                ApiDeleteMyAddressResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiDeleteMyAddressResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiShowMyAddressResponse::UserContactLink(resp) => Ok(Arc::new(resp)),
                ApiShowMyAddressResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiShowMyAddressResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiSetProfileAddressResponse::UserProfileUpdated(resp) => Ok(Arc::new(resp)),
                ApiSetProfileAddressResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiSetProfileAddressResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
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
    fn api_set_address_settings(
        &self,
        user_id: i64,
        settings: AddressSettings,
    ) -> impl Future<Output = Result<Arc<UserContactLinkUpdatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiSetAddressSettings { user_id, settings };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiSetAddressSettingsResponse::UserContactLinkUpdated(resp) => Ok(Arc::new(resp)),
                ApiSetAddressSettingsResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiSetAddressSettingsResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiSendMessagesResponse::NewChatItems(resp) => Ok(Arc::new(resp)),
                ApiSendMessagesResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiSendMessagesResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<ApiUpdateChatItemResponses, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiUpdateChatItemResponse::ChatItemUpdated(resp) => {
                    Ok(ApiUpdateChatItemResponses::ChatItemUpdated(Arc::new(resp)))
                }
                ApiUpdateChatItemResponse::ChatItemNotChanged(resp) => Ok(
                    ApiUpdateChatItemResponses::ChatItemNotChanged(Arc::new(resp)),
                ),
                ApiUpdateChatItemResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiUpdateChatItemResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiDeleteChatItemResponse::ChatItemsDeleted(resp) => Ok(Arc::new(resp)),
                ApiDeleteChatItemResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiDeleteChatItemResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiDeleteMemberChatItemResponse::ChatItemsDeleted(resp) => Ok(Arc::new(resp)),
                ApiDeleteMemberChatItemResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiDeleteMemberChatItemResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiChatItemReactionResponse::ChatItemReaction(resp) => Ok(Arc::new(resp)),
                ApiChatItemReactionResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiChatItemReactionResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
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
    fn receive_file(
        &self,
        command: ReceiveFile,
    ) -> impl Future<Output = Result<ReceiveFileResponses, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ReceiveFileResponse::RcvFileAccepted(resp) => {
                    Ok(ReceiveFileResponses::RcvFileAccepted(Arc::new(resp)))
                }
                ReceiveFileResponse::RcvFileAcceptedSndCancelled(resp) => Ok(
                    ReceiveFileResponses::RcvFileAcceptedSndCancelled(Arc::new(resp)),
                ),
                ReceiveFileResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ReceiveFileResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
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
    fn cancel_file(
        &self,
        file_id: i64,
    ) -> impl Future<Output = Result<CancelFileResponses, Self::Error>> + Send {
        async move {
            let command = CancelFile { file_id };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                CancelFileResponse::SndFileCancelled(resp) => {
                    Ok(CancelFileResponses::SndFileCancelled(Arc::new(resp)))
                }
                CancelFileResponse::RcvFileCancelled(resp) => {
                    Ok(CancelFileResponses::RcvFileCancelled(Arc::new(resp)))
                }
                CancelFileResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                CancelFileResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<SentGroupInvitationResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAddMember {
                group_id,
                contact_id,
                member_role,
            };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiAddMemberResponse::SentGroupInvitation(resp) => Ok(Arc::new(resp)),
                ApiAddMemberResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiAddMemberResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    fn api_join_group(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<UserAcceptedGroupSentResponse>, Self::Error>> + Send {
        async move {
            let command = ApiJoinGroup { group_id };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiJoinGroupResponse::UserAcceptedGroupSent(resp) => Ok(Arc::new(resp)),
                ApiJoinGroupResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiJoinGroupResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<MemberAcceptedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiAcceptMember {
                group_id,
                group_member_id,
                member_role,
            };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiAcceptMemberResponse::MemberAccepted(resp) => Ok(Arc::new(resp)),
                ApiAcceptMemberResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiAcceptMemberResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    /// /_member role #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...] observer|author|member|moderator|admin|owner
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiMembersRoleResponse::MembersRoleUser(resp) => Ok(Arc::new(resp)),
                ApiMembersRoleResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiMembersRoleResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    fn api_block_members_for_all(
        &self,
        command: ApiBlockMembersForAll,
    ) -> impl Future<Output = Result<Arc<MembersBlockedForAllUserResponse>, Self::Error>> + Send
    {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiBlockMembersForAllResponse::MembersBlockedForAllUser(resp) => Ok(Arc::new(resp)),
                ApiBlockMembersForAllResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiBlockMembersForAllResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiRemoveMembersResponse::UserDeletedMembers(resp) => Ok(Arc::new(resp)),
                ApiRemoveMembersResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiRemoveMembersResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiLeaveGroupResponse::LeftMemberUser(resp) => Ok(Arc::new(resp)),
                ApiLeaveGroupResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiLeaveGroupResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiListMembersResponse::GroupMembers(resp) => Ok(Arc::new(resp)),
                ApiListMembersResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiListMembersResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiNewGroupResponse::GroupCreated(resp) => Ok(Arc::new(resp)),
                ApiNewGroupResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiNewGroupResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiUpdateGroupProfileResponse::GroupUpdated(resp) => Ok(Arc::new(resp)),
                ApiUpdateGroupProfileResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiUpdateGroupProfileResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<GroupLinkCreatedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiCreateGroupLink {
                group_id,
                member_role,
            };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiCreateGroupLinkResponse::GroupLinkCreated(resp) => Ok(Arc::new(resp)),
                ApiCreateGroupLinkResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiCreateGroupLinkResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    /// /_set link role #<groupId> observer|author|member|moderator|admin|owner
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiGroupLinkMemberRoleResponse::GroupLink(resp) => Ok(Arc::new(resp)),
                ApiGroupLinkMemberRoleResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiGroupLinkMemberRoleResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    fn api_delete_group_link(
        &self,
        group_id: i64,
    ) -> impl Future<Output = Result<Arc<GroupLinkDeletedResponse>, Self::Error>> + Send {
        async move {
            let command = ApiDeleteGroupLink { group_id };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiDeleteGroupLinkResponse::GroupLinkDeleted(resp) => Ok(Arc::new(resp)),
                ApiDeleteGroupLinkResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiDeleteGroupLinkResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiGetGroupLinkResponse::GroupLink(resp) => Ok(Arc::new(resp)),
                ApiGetGroupLinkResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiGetGroupLinkResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiAddContactResponse::Invitation(resp) => Ok(Arc::new(resp)),
                ApiAddContactResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiAddContactResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
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
    fn api_connect_plan(
        &self,
        command: ApiConnectPlan,
    ) -> impl Future<Output = Result<Arc<ConnectionPlanResponse>, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiConnectPlanResponse::ConnectionPlan(resp) => Ok(Arc::new(resp)),
                ApiConnectPlanResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiConnectPlanResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
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
    fn api_connect(
        &self,
        command: ApiConnect,
    ) -> impl Future<Output = Result<ApiConnectResponses, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiConnectResponse::SentConfirmation(resp) => {
                    Ok(ApiConnectResponses::SentConfirmation(Arc::new(resp)))
                }
                ApiConnectResponse::ContactAlreadyExists(resp) => {
                    Ok(ApiConnectResponses::ContactAlreadyExists(Arc::new(resp)))
                }
                ApiConnectResponse::SentInvitation(resp) => {
                    Ok(ApiConnectResponses::SentInvitation(Arc::new(resp)))
                }
                ApiConnectResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiConnectResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
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
    fn connect(
        &self,
        command: Connect,
    ) -> impl Future<Output = Result<ConnectResponses, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ConnectResponse::SentConfirmation(resp) => {
                    Ok(ConnectResponses::SentConfirmation(Arc::new(resp)))
                }
                ConnectResponse::ContactAlreadyExists(resp) => {
                    Ok(ConnectResponses::ContactAlreadyExists(Arc::new(resp)))
                }
                ConnectResponse::SentInvitation(resp) => {
                    Ok(ConnectResponses::SentInvitation(Arc::new(resp)))
                }
                ConnectResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ConnectResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
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
    fn api_accept_contact(
        &self,
        contact_req_id: i64,
    ) -> impl Future<Output = Result<Arc<AcceptingContactRequestResponse>, Self::Error>> + Send
    {
        async move {
            let command = ApiAcceptContact { contact_req_id };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiAcceptContactResponse::AcceptingContactRequest(resp) => Ok(Arc::new(resp)),
                ApiAcceptContactResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiAcceptContactResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiRejectContactResponse::ContactRequestRejected(resp) => Ok(Arc::new(resp)),
                ApiRejectContactResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiRejectContactResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiListContactsResponse::ContactsList(resp) => Ok(Arc::new(resp)),
                ApiListContactsResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiListContactsResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiListGroupsResponse::GroupsList(resp) => Ok(Arc::new(resp)),
                ApiListGroupsResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiListGroupsResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<ApiDeleteChatResponses, Self::Error>> + Send {
        async move {
            let command = ApiDeleteChat {
                chat_ref,
                chat_delete_mode,
            };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiDeleteChatResponse::ContactDeleted(resp) => {
                    Ok(ApiDeleteChatResponses::ContactDeleted(Arc::new(resp)))
                }
                ApiDeleteChatResponse::ContactConnectionDeleted(resp) => Ok(
                    ApiDeleteChatResponses::ContactConnectionDeleted(Arc::new(resp)),
                ),
                ApiDeleteChatResponse::GroupDeletedUser(resp) => {
                    Ok(ApiDeleteChatResponses::GroupDeletedUser(Arc::new(resp)))
                }
                ApiDeleteChatResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiDeleteChatResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<ActiveUserResponse>, Self::Error>> + Send {
        async move {
            let command = ShowActiveUser {};
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ShowActiveUserResponse::ActiveUser(resp) => Ok(Arc::new(resp)),
                ShowActiveUserResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ShowActiveUserResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<ActiveUserResponse>, Self::Error>> + Send {
        async move {
            let command = CreateActiveUser { new_user };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                CreateActiveUserResponse::ActiveUser(resp) => Ok(Arc::new(resp)),
                CreateActiveUserResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                CreateActiveUserResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<UsersListResponse>, Self::Error>> + Send {
        async move {
            let command = ListUsers {};
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ListUsersResponse::UsersList(resp) => Ok(Arc::new(resp)),
                ListUsersResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ListUsersResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    ) -> impl Future<Output = Result<Arc<ActiveUserResponse>, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiSetActiveUserResponse::ActiveUser(resp) => Ok(Arc::new(resp)),
                ApiSetActiveUserResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiSetActiveUserResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    fn api_delete_user(
        &self,
        command: ApiDeleteUser,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, Self::Error>> + Send {
        async move {
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiDeleteUserResponse::CmdOk(resp) => Ok(Arc::new(resp)),
                ApiDeleteUserResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiDeleteUserResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
    fn api_update_profile(
        &self,
        user_id: i64,
        profile: Profile,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponses, Self::Error>> + Send {
        async move {
            let command = ApiUpdateProfile { user_id, profile };
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiUpdateProfileResponse::UserProfileUpdated(resp) => Ok(
                    ApiUpdateProfileResponses::UserProfileUpdated(Arc::new(resp)),
                ),
                ApiUpdateProfileResponse::UserProfileNoChange(resp) => Ok(
                    ApiUpdateProfileResponses::UserProfileNoChange(Arc::new(resp)),
                ),
                ApiUpdateProfileResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiUpdateProfileResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
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
            let json = self.send_raw(command.interpret()).await?;
            // Safe to unwrap because unrecognized JSON goes to undocumented variant
            let response = serde_json::from_value(json).unwrap();
            match response {
                ApiSetContactPrefsResponse::ContactPrefsUpdated(resp) => Ok(Arc::new(resp)),
                ApiSetContactPrefsResponse::ChatCmdError(resp) => {
                    Err(BadResponseError::ChatCmdError(Arc::new(resp)).into())
                }
                ApiSetContactPrefsResponse::Undocumented(resp) => {
                    Err(BadResponseError::Undocumented(resp).into())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiUpdateChatItemResponses {
    /// ChatItemUpdated: Message updated.
    #[serde(rename = "chatItemUpdated")]
    ChatItemUpdated(Arc<ChatItemUpdatedResponse>),
    /// ChatItemNotChanged: Message not changed.
    #[serde(rename = "chatItemNotChanged")]
    ChatItemNotChanged(Arc<ChatItemNotChangedResponse>),
}

impl ApiUpdateChatItemResponses {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReceiveFileResponses {
    /// RcvFileAccepted: File accepted to be received.
    #[serde(rename = "rcvFileAccepted")]
    RcvFileAccepted(Arc<RcvFileAcceptedResponse>),
    /// RcvFileAcceptedSndCancelled: File accepted, but no longer sent.
    #[serde(rename = "rcvFileAcceptedSndCancelled")]
    RcvFileAcceptedSndCancelled(Arc<RcvFileAcceptedSndCancelledResponse>),
}

impl ReceiveFileResponses {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CancelFileResponses {
    /// SndFileCancelled: Cancelled sending file.
    #[serde(rename = "sndFileCancelled")]
    SndFileCancelled(Arc<SndFileCancelledResponse>),
    /// RcvFileCancelled: Cancelled receiving file.
    #[serde(rename = "rcvFileCancelled")]
    RcvFileCancelled(Arc<RcvFileCancelledResponse>),
}

impl CancelFileResponses {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiConnectResponses {
    /// SentConfirmation: Confirmation sent to one-time invitation.
    #[serde(rename = "sentConfirmation")]
    SentConfirmation(Arc<SentConfirmationResponse>),
    /// ContactAlreadyExists: Contact already exists.
    #[serde(rename = "contactAlreadyExists")]
    ContactAlreadyExists(Arc<ContactAlreadyExistsResponse>),
    /// SentInvitation: Invitation sent to contact address.
    #[serde(rename = "sentInvitation")]
    SentInvitation(Arc<SentInvitationResponse>),
}

impl ApiConnectResponses {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectResponses {
    /// SentConfirmation: Confirmation sent to one-time invitation.
    #[serde(rename = "sentConfirmation")]
    SentConfirmation(Arc<SentConfirmationResponse>),
    /// ContactAlreadyExists: Contact already exists.
    #[serde(rename = "contactAlreadyExists")]
    ContactAlreadyExists(Arc<ContactAlreadyExistsResponse>),
    /// SentInvitation: Invitation sent to contact address.
    #[serde(rename = "sentInvitation")]
    SentInvitation(Arc<SentInvitationResponse>),
}

impl ConnectResponses {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiDeleteChatResponses {
    /// ContactDeleted: Contact deleted.
    #[serde(rename = "contactDeleted")]
    ContactDeleted(Arc<ContactDeletedResponse>),
    /// ContactConnectionDeleted: Connection deleted.
    #[serde(rename = "contactConnectionDeleted")]
    ContactConnectionDeleted(Arc<ContactConnectionDeletedResponse>),
    /// GroupDeletedUser: User deleted group.
    #[serde(rename = "groupDeletedUser")]
    GroupDeletedUser(Arc<GroupDeletedUserResponse>),
}

impl ApiDeleteChatResponses {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiUpdateProfileResponses {
    /// UserProfileUpdated: User profile updated.
    #[serde(rename = "userProfileUpdated")]
    UserProfileUpdated(Arc<UserProfileUpdatedResponse>),
    /// UserProfileNoChange: User profile was not changed.
    #[serde(rename = "userProfileNoChange")]
    UserProfileNoChange(Arc<UserProfileNoChangeResponse>),
}

impl ApiUpdateProfileResponses {
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
}

#[derive(Debug)]
pub enum BadResponseError {
    ChatCmdError(Arc<ChatCmdErrorResponse>),
    Undocumented(BTreeMap<String, JsonObject>),
}

impl std::error::Error for BadResponseError {}

impl std::fmt::Display for BadResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatCmdError(resp) => writeln!(
                f,
                "Bad server response:\n{}",
                serde_json::to_string_pretty(resp).unwrap()
            ),
            Self::Undocumented(resp) => writeln!(
                f,
                "Unexpected server response:\n{}",
                serde_json::to_string_pretty(resp).unwrap()
            ),
        }
    }
}

pub enum UndocumentedResponse<T> {
    Documented(T),
    Undocumented(BTreeMap<String, JsonObject>),
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
