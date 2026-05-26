//! The highest-level API

use simploxide_api_types::{
    AddressSettings, AutoAccept, CIDeleteMode, ChatListQuery, ChatPeerType, ConnectionPlan,
    Contact, CreatedConnLink, GroupInfo, GroupMember, GroupMemberRole, GroupProfile, JsonObject,
    LocalProfile, MsgContent, NewUser, PaginationByTime, Preferences, Profile, User,
    client_api::{ClientApi, ClientApiError as _, UndocumentedResponse},
    commands::{
        ApiAddContact, ApiConnectPlan, ApiGetChats, ApiNewGroup, ApiNewPublicGroup,
        ApiSetActiveUser, ApiSetProfileAddress, ApiSetUserAutoAcceptMemberContacts,
    },
    responses::{
        AcceptingContactRequestResponse, ActiveUserResponse, ApiChatsResponse,
        ApiDeleteChatResponse, ApiNewPublicGroupResponse, ApiUpdateChatItemResponse,
        ApiUpdateProfileResponse, CancelFileResponse, ChatItemReactionResponse,
        ChatItemsDeletedResponse, CmdOkResponse, ConnectResponse, ConnectionPlanResponse,
        ContactPrefsUpdatedResponse, ContactRequestRejectedResponse, GroupCreatedResponse,
        GroupLinkCreatedResponse, GroupLinkDeletedResponse, GroupUpdatedResponse,
        InvitationResponse, LeftMemberUserResponse, MemberAcceptedResponse,
        MembersBlockedForAllUserResponse, MembersRoleUserResponse, SentGroupInvitationResponse,
        UserAcceptedGroupSentResponse, UserDeletedMembersResponse, UserProfileUpdatedResponse,
    },
};

#[cfg(feature = "farm")]
pub mod farm;

use std::sync::Arc;

use crate::{
    ext::{
        AcceptFileBuilder, AddGroupRelaysResponse, ClientApiExt as _, DeleteMode,
        GetGroupRelaysResponse, GroupLinkResult, Reaction,
    },
    id::{
        ChatId, ContactId, ContactRequestId, FileId, GroupId, MemberId, MessageId, RelayId, UserId,
    },
    messages::{MessageBuilder, MessageLike, MulticastBuilder},
    preferences,
    preview::ImagePreview,
};

/// A cheaply cloneable handle to initialized SimpleX bot.
#[derive(Clone)]
pub struct Bot<C> {
    client: C,
    user_id: i64,
}

impl<C> Bot<C> {
    pub fn client(&self) -> &C {
        &self.client
    }

    pub fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }
}

impl<C: ClientApi> Bot<C> {
    fn new(client: C, user_id: UserId) -> Self {
        Self {
            client,
            user_id: user_id.0,
        }
    }

    pub async fn init(client: C, settings: BotSettings) -> Result<Self, C::Error> {
        let mut users = client.users().await?;

        match users.iter_mut().find_map(|info| {
            (info.user.profile.display_name == settings.display_name).then_some(&mut info.user)
        }) {
            Some(user) => Self::init_existing(client, user, settings).await,
            None => Self::init_new(client, settings).await,
        }
    }

    async fn init_existing(
        client: C,
        user: &mut User,
        settings: BotSettings,
    ) -> Result<Self, C::Error> {
        if !user.active_user {
            client
                .api_set_active_user(ApiSetActiveUser::new(user.user_id))
                .await?;
        }

        let avatar = if let Some(preview) = settings.avatar {
            Some(preview.resolve().await)
        } else {
            None
        };

        let bot = Bot {
            client,
            user_id: user.user_id,
        };

        bot.setup_auto_accept(settings.auto_accept, user.profile.contact_link.is_some())
            .await?;

        let mut profile = match settings.profile_settings {
            Some(BotProfileSettings::Preferences(preferences)) => {
                let mut current_profile = extract_profile(&mut user.profile);
                current_profile.preferences = Some(preferences);
                current_profile
            }
            Some(BotProfileSettings::FullProfile(profile)) => profile,
            None => Self::default_profile(settings.display_name),
        };
        profile.image = avatar;
        bot.client.api_update_profile(user.user_id, profile).await?;

        Ok(bot)
    }

    async fn init_new(client: C, settings: BotSettings) -> Result<Self, C::Error> {
        let avatar = if let Some(preview) = settings.avatar {
            Some(preview.resolve().await)
        } else {
            None
        };

        let mut bot_profile = match settings.profile_settings {
            Some(BotProfileSettings::Preferences(preferences)) => {
                let mut profile = Self::default_profile(settings.display_name.clone());
                profile.preferences = Some(preferences);
                profile
            }
            Some(BotProfileSettings::FullProfile(profile)) => profile,
            None => Self::default_profile(settings.display_name.clone()),
        };
        bot_profile.image = avatar;

        let response = client
            .new_user(NewUser {
                profile: Some(bot_profile),
                past_timestamp: false,
                user_chat_relay: false,
                undocumented: Default::default(),
            })
            .await?;

        let bot = Bot {
            client,
            user_id: response.user.user_id,
        };

        bot.setup_auto_accept(settings.auto_accept, false).await?;
        Ok(bot)
    }

    async fn setup_auto_accept(
        &self,
        auto_accept: Option<String>,
        has_existing_address: bool,
    ) -> Result<(), C::Error> {
        if let Some(welcome_message) = auto_accept {
            if !has_existing_address {
                self.get_or_create_address().await?;
                self.publish_address().await?;
            }

            self.configure_address(AddressSettings {
                business_address: false,
                auto_accept: Some(AutoAccept {
                    accept_incognito: false,
                    undocumented: Default::default(),
                }),
                auto_reply: (!welcome_message.is_empty())
                    .then(|| MsgContent::make_text(welcome_message)),
                undocumented: Default::default(),
            })
            .await?;
        } else if has_existing_address {
            self.configure_address(AddressSettings {
                business_address: false,
                auto_accept: None,
                auto_reply: None,
                undocumented: Default::default(),
            })
            .await?;

            self.hide_address().await?;
        }

        Ok(())
    }

    /// This method allows ot wrap or replace the underlying bot client.
    ///
    /// You can define your own clients implementing the [`ClientApi`] trait and then you can
    /// extend the bot functionalitty by implementing extension methods on `Bot<YourCustomClient>`
    /// type.
    pub fn wrap_client<W, F>(self, wrap: F) -> Bot<W>
    where
        W: ClientApi,
        F: FnOnce(C) -> W,
    {
        let new_client = wrap(self.client);

        Bot {
            client: new_client,
            user_id: self.user_id,
        }
    }

    /// Returns a minimal bot profile with conservative defaults: no files, calls, reactions, or voice.
    pub fn default_profile(name: impl Into<String>) -> Profile {
        Profile {
            display_name: name.into(),
            full_name: String::default(),
            short_descr: None,
            image: None,
            contact_link: None,
            preferences: Some(Preferences {
                timed_messages: preferences::timed_messages::NO,
                full_delete: preferences::YES,
                reactions: preferences::NO,
                voice: preferences::NO,
                files: preferences::NO,
                calls: preferences::NO,
                sessions: preferences::NO,
                commands: None,
                undocumented: Default::default(),
            }),
            peer_type: Some(ChatPeerType::Bot),
            undocumented: serde_json::Value::Null,
        }
    }

    /// Get full bot user info
    pub async fn info(&self) -> Result<Arc<ActiveUserResponse>, C::Error> {
        self.client.show_active_user().await
    }

    /// Initiates the connection sequence.
    ///
    /// - If contact is already connected returns either [UndocumentedResponse::Documented] with
    ///   [ConnectResponse::ContactAlreadyExists] or [UndocumentedResponse::Undocumented] with some
    ///   other responses(_this is an upstream mistake, SimpleX docs don't list all possible
    ///   responses for this method_).
    ///
    /// - If contact is not connected returns [UndocumentedResponse::Documented] with one of the
    ///   remaining [ConnectResponse] variants. The implementation must listen for
    ///   [crate::events::ContactConnected] or [crate::events::UserJoinedGroup] to confirm the
    ///   connection.
    pub async fn initiate_connection(
        &self,
        link: impl Into<String>,
    ) -> Result<UndocumentedResponse<ConnectResponse>, C::Error> {
        self.client.initiate_connection(link).await
    }

    /// Inspect a SimpleX link before connecting: resolves its type (contact address, group link,
    /// or 1-time invitation) and reports whether the bot is already connected via it.
    pub async fn check_connection_plan(
        &self,
        link: impl Into<String>,
    ) -> Result<Arc<ConnectionPlanResponse>, C::Error> {
        self.client
            .api_connect_plan(ApiConnectPlan {
                user_id: self.user_id,
                connection_link: Some(link.into()),
                resolve_known: true,
                link_owner_sig: None,
            })
            .await
    }

    /// Initiate a connection only if [`ConnectionPlan`] satisfies the predicate. For example, this
    /// can be used to connect strictly via one-time links:
    ///
    /// ```ignore
    /// let conn = bot.initiate_connection_if(
    ///     link,
    ///     |plan| matches!(plan, ConnectionPlan::InvitationLink { .. })
    /// ).await?;
    ///
    /// if conn.is_rejected() {
    ///     return Err("not a one-time link");
    /// }
    /// ```
    pub async fn initiate_connection_if<F: FnOnce(&ConnectionPlan) -> bool>(
        &self,
        link: impl Into<String>,
        predicate: F,
    ) -> Result<Connection, C::Error> {
        let link = link.into();
        let plan_resp = self.check_connection_plan(link.clone()).await?;

        if !predicate(&plan_resp.connection_plan) {
            return Ok(Connection::Rejected(plan_resp));
        }

        self.initiate_connection(link)
            .await
            .map(Connection::Initiated)
    }

    /// Create one-time-invitation link. Can be used for admin-access or for private connections
    /// with other bots. The [InvitationResponse::connection::pcc_conn_id] can be matched with
    /// [crate::types::Connection::conn_id] to recognize the user connected by this link when handling the
    /// [crate::events::ContactConnected] event(see [crate::events::ContactConnected::contact])
    pub async fn create_invitation_link(
        &self,
    ) -> Result<(String, Arc<InvitationResponse>), C::Error> {
        let response = self
            .client
            .api_add_contact(ApiAddContact::new(self.user_id))
            .await?;

        let link = extract_address(&response.conn_link_invitation);
        Ok((link, response))
    }

    pub async fn create_address(&self) -> Result<String, C::Error> {
        let response = self.client.api_create_my_address(self.user_id).await?;
        Ok(extract_address(&response.conn_link_contact))
    }

    /// Throws [crate::types::errors::StoreError::UserContactLinkNotFound] if bot doesn't have an address. Use
    /// [Self::get_or_create_address] to ensure that address is available
    pub async fn address(&self) -> Result<String, C::Error> {
        let response = self.client.api_show_my_address(self.user_id).await?;
        Ok(extract_address(&response.contact_link.conn_link_contact))
    }

    pub async fn get_or_create_address(&self) -> Result<String, C::Error> {
        match self.address().await {
            Ok(address) => Ok(address),
            Err(e)
                if e.bad_response()
                    .and_then(|e| {
                        e.chat_error().and_then(|e| {
                            e.error_store().map(|e| e.is_user_contact_link_not_found())
                        })
                    })
                    .unwrap_or(false) =>
            {
                self.create_address().await
            }
            Err(e) => Err(e),
        }
    }

    pub async fn configure_address(&self, settings: AddressSettings) -> Result<(), C::Error> {
        self.client
            .api_set_address_settings(self.user_id, settings)
            .await
            .map(drop)
    }

    /// Make address visible in bot/user profile
    pub async fn publish_address(&self) -> Result<Arc<UserProfileUpdatedResponse>, C::Error> {
        self.client
            .api_set_profile_address(ApiSetProfileAddress {
                user_id: self.user_id,
                enable: true,
            })
            .await
    }

    /// Hide address from bot/user profile
    pub async fn hide_address(&self) -> Result<Arc<UserProfileUpdatedResponse>, C::Error> {
        self.client
            .api_set_profile_address(ApiSetProfileAddress {
                user_id: self.user_id,
                enable: false,
            })
            .await
    }

    pub async fn delete_address(&self) -> Result<(), C::Error> {
        self.client.api_delete_my_address(self.user_id).await?;
        Ok(())
    }

    /// Fetches the current profile and applies `updater` to it before saving.
    pub async fn update_profile<F>(&self, updater: F) -> Result<ApiUpdateProfileResponse, C::Error>
    where
        F: 'static + Send + FnOnce(&mut Profile),
    {
        let mut response = self.client.show_active_user().await?;
        let response = Arc::get_mut(&mut response).unwrap();

        let mut profile = extract_profile(&mut response.user.profile);
        updater(&mut profile);

        self.client.api_update_profile(self.user_id, profile).await
    }

    pub async fn set_display_name(
        &self,
        name: impl Into<String>,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        let name = name.into();
        self.update_profile(move |profile| profile.display_name = name)
            .await
    }

    pub async fn set_full_name(
        &self,
        full_name: impl Into<String>,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        let full_name = full_name.into();
        self.update_profile(move |profile| profile.full_name = full_name)
            .await
    }

    pub async fn set_bio(
        &self,
        bio: impl Into<String>,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        let bio = bio.into();
        self.update_profile(move |profile| profile.short_descr = Some(bio))
            .await
    }

    /// Set the bot/user avatar
    pub async fn set_avatar(
        &self,
        avatar: ImagePreview,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        let image = avatar.resolve().await;
        self.update_profile(move |profile| profile.image = Some(image))
            .await
    }

    /// Set account type `Bot` or `Person`
    pub async fn set_peer_type(
        &self,
        peer_type: ChatPeerType,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        self.update_profile(move |profile| profile.peer_type = Some(peer_type))
            .await
    }

    /// Set global preferences
    pub async fn set_preferences(
        &self,
        preferences: Preferences,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        self.update_profile(move |profile| profile.preferences = Some(preferences))
            .await
    }

    /// Update global preferences via closure accepting current preferences
    pub async fn update_preferences<F>(
        &self,
        updater: F,
    ) -> Result<ApiUpdateProfileResponse, C::Error>
    where
        F: 'static + Send + FnOnce(&mut Preferences),
    {
        let mut response = self.client.show_active_user().await?;
        let response = Arc::get_mut(&mut response).unwrap();

        let mut profile = extract_profile(&mut response.user.profile);
        let mut preferences = extract_preferences(&mut profile.preferences);
        updater(&mut preferences);
        profile.preferences = Some(preferences);

        self.client.api_update_profile(self.user_id, profile).await
    }

    /// Set preferences for particular contact
    pub async fn set_contact_preferences<CID: Into<ContactId>>(
        &self,
        contact_id: CID,
        preferences: Preferences,
    ) -> Result<Arc<ContactPrefsUpdatedResponse>, C::Error> {
        self.client
            .api_set_contact_prefs(contact_id.into().0, preferences)
            .await
    }

    /// Tweak global preferences for particular contact via closure accepting current global
    /// preferences
    pub async fn tweak_preferences_for_contact<CID: Into<ContactId>, F>(
        &self,
        contact_id: CID,
        updater: F,
    ) -> Result<Arc<ContactPrefsUpdatedResponse>, C::Error>
    where
        F: 'static + Send + FnOnce(&mut Preferences),
    {
        let mut response = self.client.show_active_user().await?;
        let response = Arc::get_mut(&mut response).unwrap();

        let mut preferences = extract_preferences(&mut response.user.profile.preferences);
        updater(&mut preferences);

        self.client
            .api_set_contact_prefs(contact_id.into().0, preferences)
            .await
    }

    /// Get all contacts known to the bot(connected or not)
    pub async fn contacts(&self) -> Result<Vec<Contact>, C::Error> {
        self.client.contacts(self.user_id()).await
    }

    /// Get all groups known to the bot
    pub async fn groups(&self) -> Result<Vec<GroupInfo>, C::Error> {
        self.client.groups(self.user_id()).await
    }

    /// Accept contact request
    pub async fn accept_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> Result<Arc<AcceptingContactRequestResponse>, <C as ClientApi>::Error> {
        self.client.accept_contact(contact_request_id).await
    }

    /// Reject contact request
    pub async fn reject_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> Result<Arc<ContactRequestRejectedResponse>, <C as ClientApi>::Error> {
        self.client.reject_contact(contact_request_id).await
    }

    /// Send a message. See the [`messages`](crate::messages) module for details
    pub fn send_msg<CID: Into<ChatId>, M: MessageLike>(
        &self,
        chat_id: CID,
        msg: M,
    ) -> MessageBuilder<'_, C, M::Kind> {
        self.client.send_message(chat_id.into(), msg)
    }

    /// Send the same message to multiple recepients
    pub fn multicast<I, M>(&self, chat_ids: I, msg: M) -> MulticastBuilder<'_, I, C, M::Kind>
    where
        I: IntoIterator<Item = ChatId>,
        M: MessageLike,
    {
        self.client.multicast_message(chat_ids, msg)
    }

    /// Returns a list of all known chat IDs
    pub async fn chat_ids(&self) -> Result<impl Iterator<Item = ChatId>, C::Error> {
        self.chat_ids_with(|_| true).await
    }

    /// Returns a list of all known chat IDs matching the filter `f`.
    pub async fn chat_ids_with<F>(
        &self,
        f: F,
    ) -> Result<impl 'static + Send + Iterator<Item = ChatId>, C::Error>
    where
        F: 'static + Send + FnMut(&ChatId) -> bool,
    {
        let (contacts, groups) = futures::future::try_join(self.contacts(), self.groups()).await?;

        Ok(contacts
            .into_iter()
            .map(ChatId::from)
            .chain(groups.into_iter().map(ChatId::from))
            .filter(f))
    }

    /// Generate a [MulticastBuilder] that is ready to send messages to all known chats
    ///
    /// ```rust
    /// bot.prepare_broadcast("Hey, what's up?!")
    ///    .await
    ///    .send()
    ///    .await?;
    /// ```
    pub async fn prepare_broadcast<M: MessageLike>(
        &self,
        msg: M,
    ) -> Result<
        MulticastBuilder<'_, impl 'static + Send + Iterator<Item = ChatId>, C, M::Kind>,
        C::Error,
    > {
        self.prepare_broadcast_with(msg, |_| true).await
    }

    /// Generate a [MulticastBuilder] that is ready to send messages to chats matching the filter
    ///
    /// ```rust
    /// bot.prepare_broadcast_with("What do you think about this logo?", |chat| chat.is_direct())
    ///    .await
    ///    .with_image(Image::new("logo.jpg"))
    ///    .send()
    ///    .await?;
    /// ```
    pub async fn prepare_broadcast_with<M, F>(
        &self,
        msg: M,
        f: F,
    ) -> Result<
        MulticastBuilder<'_, impl 'static + Send + Iterator<Item = ChatId>, C, M::Kind>,
        C::Error,
    >
    where
        F: 'static + Send + FnMut(&ChatId) -> bool,
        M: MessageLike,
    {
        let ids = self.chat_ids_with(f).await?;
        let (msg, kind) = msg.into_builder_parts();

        Ok(MulticastBuilder {
            client: self.client(),
            chat_ids: ids,
            ttl: None,
            msg,
            kind,
        })
    }

    pub async fn update_msg<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        new_content: MsgContent,
    ) -> Result<ApiUpdateChatItemResponse, C::Error> {
        self.client
            .update_message(chat_id, message_id, new_content)
            .await
    }

    pub async fn delete_msg<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        mode: CIDeleteMode,
    ) -> Result<Arc<ChatItemsDeletedResponse>, C::Error> {
        self.client.delete_message(chat_id, message_id, mode).await
    }

    pub async fn batch_delete_msgs<CID: Into<ChatId>, I: IntoIterator<Item = MessageId>>(
        &self,
        chat_id: CID,
        message_ids: I,
        mode: CIDeleteMode,
    ) -> Result<Arc<ChatItemsDeletedResponse>, C::Error> {
        self.client
            .batch_delete_messages(chat_id, message_ids, mode)
            .await
    }

    /// Applies multiple reactions to a message. Returns one result per reaction.
    pub async fn batch_msg_reactions<
        CID: Into<ChatId>,
        MID: Into<MessageId>,
        I: IntoIterator<Item = Reaction>,
    >(
        &self,
        chat_id: CID,
        message_id: MID,
        reactions: I,
    ) -> Vec<Result<Arc<ChatItemReactionResponse>, C::Error>> {
        self.client
            .batch_message_reactions(chat_id, message_id, reactions)
            .await
    }

    pub async fn update_msg_reaction<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        reaction: Reaction,
    ) -> Vec<Result<Arc<ChatItemReactionResponse>, C::Error>> {
        self.client
            .update_message_reaction(chat_id, message_id, reaction)
            .await
    }

    /// Starts background file download. Catch `RcvFile*` events to track the progress
    pub fn accept_file<FID: Into<FileId>>(&self, file_id: FID) -> AcceptFileBuilder<'_, C> {
        self.client.accept_file(file_id)
    }

    pub async fn reject_file<FID: Into<FileId>>(
        &self,
        file_id: FID,
    ) -> Result<CancelFileResponse, C::Error> {
        self.client.reject_file(file_id).await
    }

    pub async fn delete_chat<CID: Into<ChatId>>(
        &self,
        chat_id: CID,
        mode: DeleteMode,
    ) -> Result<ApiDeleteChatResponse, C::Error> {
        self.client.delete_chat(chat_id, mode).await
    }

    /// Create a new group. The bot's user becomes the owner.
    pub async fn create_group(
        &self,
        profile: GroupProfile,
    ) -> Result<Arc<GroupCreatedResponse>, C::Error> {
        self.client
            .api_new_group(ApiNewGroup::new(self.user_id, profile))
            .await
    }

    /// Create a new public group with relay members. The bot's user becomes the owner.
    /// Relay IDs can be obtained from [`Bot::get_group_relays`]
    pub async fn create_public_group<I: IntoIterator<Item = RelayId>>(
        &self,
        relay_ids: I,
        profile: GroupProfile,
    ) -> Result<ApiNewPublicGroupResponse, C::Error> {
        self.client
            .api_new_public_group(ApiNewPublicGroup::new(
                self.user_id,
                relay_ids.into_iter().map(|id| id.0).collect(),
                profile,
            ))
            .await
    }

    /// Enable or disable automatically accepting contacts from group members.
    pub async fn set_auto_accept_member_contacts(
        &self,
        on: bool,
    ) -> Result<Arc<CmdOkResponse>, C::Error> {
        self.client
            .api_set_user_auto_accept_member_contacts(ApiSetUserAutoAcceptMemberContacts {
                user_id: self.user_id,
                on_off: on,
            })
            .await
    }

    /// Sends a group invitation to a contact.
    pub async fn add_member<GID: Into<GroupId>, CID: Into<ContactId>>(
        &self,
        group_id: GID,
        contact_id: CID,
        role: GroupMemberRole,
    ) -> Result<Arc<SentGroupInvitationResponse>, C::Error> {
        self.client.add_member(group_id, contact_id, role).await
    }

    /// Accepts a pending group invitation.
    pub async fn join_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> Result<Arc<UserAcceptedGroupSentResponse>, C::Error> {
        self.client.join_group(group_id).await
    }

    /// Confirms a pending group membership request.
    pub async fn accept_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> Result<Arc<MemberAcceptedResponse>, C::Error> {
        self.client.accept_member(group_id, member_id, role).await
    }

    pub async fn set_members_role<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
        role: GroupMemberRole,
    ) -> Result<Arc<MembersRoleUserResponse>, C::Error> {
        self.client
            .set_members_role(group_id, member_ids, role)
            .await
    }

    pub async fn set_member_role<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> Result<Arc<MembersRoleUserResponse>, C::Error> {
        self.client.set_member_role(group_id, member_id, role).await
    }

    /// Blocks members so their messages are hidden for everyone in the group.
    pub async fn block_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> Result<Arc<MembersBlockedForAllUserResponse>, C::Error> {
        self.client
            .block_members_for_all(group_id, member_ids)
            .await
    }

    /// Reverses a previous [`block_members_for_all`](Self::block_members_for_all).
    pub async fn unblock_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> Result<Arc<MembersBlockedForAllUserResponse>, C::Error> {
        self.client
            .unblock_members_for_all(group_id, member_ids)
            .await
    }

    /// Blocks a member so their messages are hidden for everyone in the group.
    pub async fn block_member_for_all<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> Result<Arc<MembersBlockedForAllUserResponse>, C::Error> {
        self.client.block_member_for_all(group_id, member_id).await
    }

    /// Reverses a previous [`block_member_for_all`](Self::block_member_for_all).
    pub async fn unblock_member_for_all<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> Result<Arc<MembersBlockedForAllUserResponse>, C::Error> {
        self.client
            .unblock_member_for_all(group_id, member_id)
            .await
    }

    /// Removes members from the group, preserving their past messages.
    pub async fn remove_members<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> Result<Arc<UserDeletedMembersResponse>, C::Error> {
        self.client.remove_members(group_id, member_ids).await
    }

    /// Removes members from the group and deletes their messages.
    pub async fn remove_members_with_messages<
        GID: Into<GroupId>,
        I: IntoIterator<Item = MemberId>,
    >(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> Result<Arc<UserDeletedMembersResponse>, C::Error> {
        self.client
            .remove_members_with_messages(group_id, member_ids)
            .await
    }

    /// Removes a member from the group, preserving their past messages.
    pub async fn remove_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> Result<Arc<UserDeletedMembersResponse>, C::Error> {
        self.client.remove_member(group_id, member_id).await
    }

    /// Removes a member from the group and deletes their messages.
    pub async fn remove_member_with_messages<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> Result<Arc<UserDeletedMembersResponse>, C::Error> {
        self.client
            .remove_member_with_messages(group_id, member_id)
            .await
    }

    pub async fn leave_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> Result<Arc<LeftMemberUserResponse>, C::Error> {
        self.client.leave_group(group_id).await
    }

    pub async fn list_members<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> Result<Vec<GroupMember>, C::Error> {
        self.client.list_members(group_id).await
    }

    /// Deletes messages for all group members. Requires admin or owner role.
    pub async fn moderate_messages<GID: Into<GroupId>, I: IntoIterator<Item = MessageId>>(
        &self,
        group_id: GID,
        message_ids: I,
    ) -> Result<Arc<ChatItemsDeletedResponse>, C::Error> {
        self.client.moderate_messages(group_id, message_ids).await
    }

    /// Deletes a message for all group members. Requires admin or owner role.
    pub async fn moderate_message<GID: Into<GroupId>, MID: Into<MessageId>>(
        &self,
        group_id: GID,
        message_id: MID,
    ) -> Result<Arc<ChatItemsDeletedResponse>, C::Error> {
        self.client.moderate_message(group_id, message_id).await
    }

    pub async fn update_group_profile<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        profile: GroupProfile,
    ) -> Result<Arc<GroupUpdatedResponse>, C::Error> {
        self.client.update_group_profile(group_id, profile).await
    }

    /// Stores arbitrary app-defined JSON on the group. Pass `None` to clear it.
    pub async fn set_group_custom_data<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        data: Option<JsonObject>,
    ) -> Result<Arc<CmdOkResponse>, C::Error> {
        self.client.set_group_custom_data(group_id, data).await
    }

    /// Stores arbitrary app-defined JSON on the contact. Pass `None` to clear it.
    pub async fn set_contact_custom_data<CID: Into<ContactId>>(
        &self,
        contact_id: CID,
        data: Option<JsonObject>,
    ) -> Result<Arc<CmdOkResponse>, C::Error> {
        self.client.set_contact_custom_data(contact_id, data).await
    }

    pub async fn create_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> Result<Arc<GroupLinkCreatedResponse>, C::Error> {
        self.client.create_group_link(group_id, role).await
    }

    /// Changes the default role assigned to members who join via the group link.
    pub async fn set_group_link_role<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> GroupLinkResult<C> {
        self.client.set_group_link_role(group_id, role).await
    }

    pub async fn delete_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> Result<Arc<GroupLinkDeletedResponse>, C::Error> {
        self.client.delete_group_link(group_id).await
    }

    pub async fn get_group_link<GID: Into<GroupId>>(&self, group_id: GID) -> GroupLinkResult<C> {
        self.client.get_group_link(group_id).await
    }

    pub async fn get_group_relays<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> GetGroupRelaysResponse<C> {
        self.client.get_group_relays(group_id).await
    }

    pub async fn add_group_relays<GID: Into<GroupId>, I: IntoIterator<Item = RelayId>>(
        &self,
        group_id: GID,
        relay_ids: I,
    ) -> AddGroupRelaysResponse<C> {
        self.client.add_group_relays(group_id, relay_ids).await
    }

    pub async fn add_group_relay<GID: Into<GroupId>, RID: Into<RelayId>>(
        &self,
        group_id: GID,
        relay_id: RID,
    ) -> AddGroupRelaysResponse<C> {
        self.client.add_group_relay(group_id, relay_id).await
    }

    /// Get chats with time-based pagination. Prefer this over [`Bot::contacts`] / [`Bot::groups`]
    /// for large databases as it avoids loading all records into memory at once.
    pub async fn get_chats(
        &self,
        pagination: PaginationByTime,
        query: ChatListQuery,
    ) -> Result<Arc<ApiChatsResponse>, C::Error> {
        self.client
            .api_get_chats(ApiGetChats::new(self.user_id, pagination, query))
            .await
    }
}

#[cfg(feature = "xftp")]
impl<C: crate::xftp::XftpExt> Bot<C> {
    pub fn download_file<FID: Into<FileId>>(
        &self,
        file_id: FID,
    ) -> crate::xftp::DownloadFileBuilder<'_, C> {
        self.client.download_file(file_id)
    }
}

#[cfg(feature = "websocket")]
impl crate::ws::Bot {
    pub fn shutdown(self) -> impl Future<Output = ()> {
        self.client.disconnect()
    }
}

#[cfg(feature = "ffi")]
impl crate::ffi::Bot {
    pub fn shutdown(self) -> impl Future<Output = ()> {
        self.client.disconnect()
    }
}

/// Passed to [`Bot::init`] to configure bot identity and startup behaviour.
#[derive(Debug, Clone)]
pub struct BotSettings {
    pub display_name: String,
    /// If string is empty creates an auto-accepting address without a message. If string is not
    /// empty adds a welcome message to the address
    pub auto_accept: Option<String>,
    pub profile_settings: Option<BotProfileSettings>,
    pub avatar: Option<ImagePreview>,
}

impl BotSettings {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            display_name: name.into(),
            auto_accept: None,
            profile_settings: None,
            avatar: None,
        }
    }

    pub fn with_avatar(mut self, avatar: ImagePreview) -> Self {
        self.avatar = Some(avatar);
        self
    }

    /// Create a public auto-accepting address during the intialisation
    pub fn auto_accept(mut self) -> Self {
        self.auto_accept = Some(String::default());
        self
    }

    /// Create a public auto-accepting address with a welcome meesage during the intialisation
    pub fn auto_accept_with(mut self, welcome_message: impl Into<String>) -> Self {
        self.auto_accept = Some(welcome_message.into());
        self
    }

    pub fn with_profile_settings(mut self, settings: BotProfileSettings) -> Self {
        self.profile_settings = Some(settings);
        self
    }
}

#[derive(Debug, Clone)]
pub enum BotProfileSettings {
    /// Apply only the given preferences; leave all other profile fields unchanged.
    Preferences(Preferences),
    /// Replace the entire profile.
    FullProfile(Profile),
}

pub enum Connection {
    Initiated(UndocumentedResponse<ConnectResponse>),
    Rejected(Arc<ConnectionPlanResponse>),
}

impl Connection {
    pub fn rejected(&self) -> Option<&ConnectionPlan> {
        if let Self::Rejected(resp) = self {
            Some(&resp.connection_plan)
        } else {
            None
        }
    }

    pub fn initiated(&self) -> Option<&UndocumentedResponse<ConnectResponse>> {
        if let Self::Initiated(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    pub fn is_rejected(&self) -> bool {
        self.rejected().is_some()
    }

    pub fn is_initiated(&self) -> bool {
        self.initiated().is_some()
    }
}

fn extract_address(link: &CreatedConnLink) -> String {
    link.conn_short_link
        .clone()
        .unwrap_or_else(|| link.conn_full_link.clone())
}

fn extract_profile(local: &mut LocalProfile) -> Profile {
    Profile {
        display_name: std::mem::take(&mut local.display_name),
        full_name: std::mem::take(&mut local.full_name),
        short_descr: local.short_descr.take(),
        image: local.image.take(),
        contact_link: local.contact_link.take(),
        preferences: local.preferences.take(),
        peer_type: local.peer_type.take(),
        undocumented: std::mem::take(&mut local.undocumented),
    }
}

fn extract_preferences(preferences: &mut Option<Preferences>) -> Preferences {
    match preferences.as_mut() {
        Some(prefs) => Preferences {
            timed_messages: prefs.timed_messages.take(),
            full_delete: prefs.full_delete.take(),
            reactions: prefs.reactions.take(),
            voice: prefs.voice.take(),
            files: prefs.files.take(),
            calls: prefs.calls.take(),
            sessions: prefs.sessions.take(),
            commands: prefs.commands.take(),
            undocumented: std::mem::take(&mut prefs.undocumented),
        },
        None => Preferences {
            timed_messages: None,
            full_delete: None,
            reactions: None,
            voice: None,
            files: None,
            calls: None,
            sessions: None,
            commands: None,
            undocumented: Default::default(),
        },
    }
}
