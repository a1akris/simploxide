//! High-level bot API.
//!
//! # Construction
//!
//! Bots are supposed to be constructed via concrete backend builders. See
//! [`ffi::BotBuilder`](crate::ffi::BotBuilder) and [`ws::BotBuilder`](crate::ws::BotBuilder).
//!
//! # Bots vs Bot Farms
//!
//! Constructing multiple bots via multiple `BotBuilder` invocations creates separate SimpleX-Chat
//! instances. For `ws` spawning multiple CLI processes is required, for `ffi` a new chat
//! controller is being created per bot under the hood. This is useful if you want to separate bot
//! states completely(e.g. different databases encrypted by different keys). To manage multiple
//! bots on the same SimpleX-Chat instance use [bot farms](farm). See
//! [`ffi::BotBuilder`](crate::ffi::BotBuilder) and [`ws::BotBuilder`](crate::ws::BotBuilder).

use simploxide_api_types::{
    AddressSettings, AutoAccept, BadgeProof, CIDeleteMode, ChatListQuery, ChatPeerType,
    ConnectionPlan, Contact, CreatedConnLink, GroupInfo, GroupMember, GroupMemberRole,
    GroupPreferences, GroupProfile, JsonObject, LocalProfile, MsgContent, NewUser,
    PaginationByTime, Preferences, Profile, SimplexDomainClaim, User, UserInfo,
    client_api::{BadResponseError, ClientApi, ClientApiError as _, UndocumentedResponse},
    commands::ApiSetActiveUser,
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

use std::sync::Arc;

use futures::{FutureExt as _, TryFutureExt as _};

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

#[cfg(feature = "farm")]
pub mod farm;

#[cfg(feature = "farm")]
pub use farm::BotFarm;

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
        UserId::from_raw(self.user_id)
    }
}

impl<C: ClientApi> Bot<C> {
    #[cfg(feature = "farm")]
    fn new(client: C, user_id: UserId) -> Self {
        Self {
            client,
            user_id: user_id.raw(),
        }
    }

    pub async fn init(client: C, settings: BotSettings) -> Result<Self, C::Error> {
        let mut users = client.users().await?;

        match settings.display_name.match_user(&mut users) {
            Some(current) => Self::init_existing(client, current, settings).await,
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

        let mut current = extract_profile(&mut user.profile);

        current.display_name = settings.display_name.current();
        let has_existing_address = current.contact_link.is_some();

        // Preserve the contact_link only when the address will remain published after init.
        // When auto_accept is None and an address exists, setup_auto_accept will call
        // delete_address(), so passing contact_link=Some here would cause a spurious
        // "set contact address" event immediately before "removed contact address".
        let keep_contact_link = settings.auto_accept.is_some() || !has_existing_address;
        let preserved_contact_link = keep_contact_link
            .then(|| current.contact_link.take())
            .flatten();

        let profile = match settings.profile_settings {
            Some(BotProfileSettings::Preferences(preferences)) => {
                current.preferences = Some(preferences);
                current.contact_link = preserved_contact_link;
                current.image = avatar.or(current.image);
                current.short_descr = settings.bio.or(current.short_descr);
                current.description = settings.description.or(current.description);
                current
            }
            Some(BotProfileSettings::FullProfile(mut new_profile)) => {
                new_profile.contact_link = preserved_contact_link;
                new_profile.image = new_profile.image.or(avatar);
                new_profile.short_descr = new_profile.short_descr.or(settings.bio);
                new_profile.description = new_profile.description.or(settings.description);
                new_profile
            }
            None => {
                let mut p = Self::default_profile(current.display_name);
                p.contact_link = preserved_contact_link;
                p.image = avatar;
                p.short_descr = settings.bio;
                p.description = settings.description;
                p
            }
        };

        bot.client
            .update_profile(UserId::from_raw(user.user_id), profile)
            .await?;

        bot.setup_auto_accept(settings.auto_accept, has_existing_address)
            .await?;

        Ok(bot)
    }

    async fn init_new(client: C, settings: BotSettings) -> Result<Self, C::Error> {
        let avatar = if let Some(preview) = settings.avatar {
            Some(preview.resolve().await)
        } else {
            None
        };

        let bot_profile = match settings.profile_settings {
            Some(BotProfileSettings::Preferences(preferences)) => {
                let mut profile = Self::default_profile(settings.display_name.current());
                profile.preferences = Some(preferences);
                profile.image = avatar;
                profile.short_descr = settings.bio;
                profile.description = settings.description;
                profile
            }
            Some(BotProfileSettings::FullProfile(mut profile)) => {
                profile.image = profile.image.or(avatar);
                profile.short_descr = profile.short_descr.or(settings.bio);
                profile.description = profile.description.or(settings.description);
                profile
            }
            None => {
                let mut profile = Self::default_profile(settings.display_name.current());
                profile.image = avatar;
                profile.short_descr = settings.bio;
                profile.description = settings.description;
                profile
            }
        };

        let response = client
            .new_user(NewUser {
                profile: Some(bot_profile),
                client_service: false,
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
            self.delete_address().await?;
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

    /// Conservative bot preferences: full-delete on, everything else off.
    pub fn default_preferences() -> Preferences {
        Preferences {
            timed_messages: preferences::timed_messages::NO,
            full_delete: preferences::YES,
            reactions: preferences::NO,
            voice: preferences::NO,
            files: preferences::NO,
            calls: preferences::NO,
            sessions: preferences::NO,
            commands: None,
            undocumented: Default::default(),
        }
    }

    /// Minimal bot profile with [`Self::default_preferences`] and `Bot` peer type.
    pub fn default_profile(name: impl Into<String>) -> Profile {
        Profile {
            display_name: name.into(),
            full_name: String::default(),
            short_descr: None,
            description: None,
            image: None,
            contact_link: None,
            contact_domain: None,
            preferences: Some(Self::default_preferences()),
            badge: None,
            peer_type: Some(ChatPeerType::Bot),
            undocumented: serde_json::Value::Null,
        }
    }

    /// Get full bot user info
    pub fn info(&self) -> impl Future<Output = Result<Arc<ActiveUserResponse>, C::Error>> {
        self.client.show_active_user()
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
    pub fn initiate_connection(
        &self,
        link: impl Into<String>,
    ) -> impl Future<Output = Result<UndocumentedResponse<ConnectResponse>, C::Error>> {
        self.client.initiate_connection(link)
    }

    /// Inspect a SimpleX target before connecting: resolves its type (name, contact address, group link,
    /// or 1-time invitation) and reports whether the bot is already connected via it.
    pub fn check_connection_plan(
        &self,
        target: impl Into<String>,
    ) -> impl Future<Output = Result<Arc<ConnectionPlanResponse>, C::Error>> {
        self.client.connection_plan(self.user_id(), target)
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
    /// with other bots. The [`connection.pcc_conn_id`](crate::types::PendingContactConnection::pcc_conn_id) can be matched with
    /// [crate::types::Connection::conn_id] to recognize the user connected by this link when handling the
    /// [crate::events::ContactConnected] event(see [crate::events::ContactConnected::contact])
    pub fn create_invitation_link(
        &self,
    ) -> impl Future<Output = Result<(String, Arc<InvitationResponse>), C::Error>> {
        self.client
            .create_invitation_link(self.user_id())
            .map_ok(|resp| (extract_address(&resp.conn_link_invitation), resp))
    }

    pub fn create_address(&self) -> impl Future<Output = Result<String, C::Error>> {
        self.client
            .create_address(self.user_id())
            .map_ok(|resp| extract_address(&resp.conn_link_contact))
    }

    /// Throws [crate::types::errors::StoreError::UserContactLinkNotFound] if bot doesn't have an address. Use
    /// [Self::get_or_create_address] to ensure that address is available
    pub fn address(&self) -> impl Future<Output = Result<String, C::Error>> {
        self.client
            .show_address(self.user_id())
            .map_ok(|resp| extract_address(&resp.contact_link.conn_link_contact))
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

    pub fn configure_address(
        &self,
        settings: AddressSettings,
    ) -> impl Future<Output = Result<(), C::Error>> {
        self.client
            .configure_address(self.user_id(), settings)
            .map(|r| r.map(drop))
    }

    /// Make address visible in bot/user profile
    pub fn publish_address(
        &self,
    ) -> impl Future<Output = Result<Arc<UserProfileUpdatedResponse>, C::Error>> {
        self.client.publish_address(self.user_id())
    }

    /// Hide address from bot/user profile
    pub fn hide_address(
        &self,
    ) -> impl Future<Output = Result<Arc<UserProfileUpdatedResponse>, C::Error>> {
        self.client.hide_address(self.user_id())
    }

    pub fn delete_address(&self) -> impl Future<Output = Result<(), C::Error>> {
        self.client
            .delete_address(self.user_id())
            .map(|r| r.map(drop))
    }

    pub fn profile(&self) -> impl Future<Output = Result<Profile, C::Error>> {
        self.client.show_active_user().map_ok(|mut resp| {
            let resp = Arc::get_mut(&mut resp).unwrap();
            extract_profile(&mut resp.user.profile)
        })
    }

    /// Fetches the current profile and applies `updater` to it before saving.
    pub async fn update_profile<F>(&self, updater: F) -> Result<ApiUpdateProfileResponse, C::Error>
    where
        F: 'static + Send + FnOnce(&mut Profile),
    {
        let mut profile = self.profile().await?;
        updater(&mut profile);
        self.client
            .update_profile(self.user_id(), profile.clone())
            .await
    }

    pub fn set_display_name(
        &self,
        name: impl Into<String>,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        let name = name.into();
        self.update_profile(move |profile| profile.display_name = name)
    }

    pub fn set_full_name(
        &self,
        full_name: impl Into<String>,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        let full_name = full_name.into();
        self.update_profile(move |profile| profile.full_name = full_name)
    }

    pub fn set_bio(
        &self,
        bio: impl Into<String>,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        let bio = bio.into();
        self.update_profile(move |profile| profile.short_descr = Some(bio))
    }

    pub fn set_description(
        &self,
        description: impl Into<String>,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        let description = description.into();
        self.update_profile(move |profile| profile.description = Some(description))
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
    pub fn set_peer_type(
        &self,
        peer_type: ChatPeerType,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        self.update_profile(move |profile| profile.peer_type = Some(peer_type))
    }

    pub fn set_badge(
        &self,
        badge: BadgeProof,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        self.update_profile(move |profile| profile.badge = Some(badge))
    }

    pub fn set_contact_domain(
        &self,
        domain: SimplexDomainClaim,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        self.update_profile(move |profile| profile.contact_domain = Some(domain))
    }

    pub fn clear_contact_domain(
        &self,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        self.update_profile(|profile| profile.contact_domain = None)
    }

    /// Set global preferences
    pub fn set_preferences(
        &self,
        preferences: Preferences,
    ) -> impl Future<Output = Result<ApiUpdateProfileResponse, C::Error>> {
        self.update_profile(move |profile| profile.preferences = Some(preferences))
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

        self.client.update_profile(self.user_id(), profile).await
    }

    /// Set preferences for particular contact
    pub fn set_contact_preferences<CID: Into<ContactId>>(
        &self,
        contact_id: CID,
        preferences: Preferences,
    ) -> impl Future<Output = Result<Arc<ContactPrefsUpdatedResponse>, C::Error>> {
        self.client.set_contact_prefs(contact_id, preferences)
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

        self.client.set_contact_prefs(contact_id, preferences).await
    }

    /// Get all contacts known to the bot(connected or not)
    pub fn contacts(&self) -> impl Future<Output = Result<Vec<Contact>, C::Error>> {
        self.client.contacts(self.user_id())
    }

    /// Get all groups known to the bot
    pub fn groups(&self) -> impl Future<Output = Result<Vec<GroupInfo>, C::Error>> {
        self.client.groups(self.user_id())
    }

    /// Accept contact request
    pub fn accept_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> impl Future<Output = Result<Arc<AcceptingContactRequestResponse>, C::Error>> {
        self.client.accept_contact(contact_request_id)
    }

    /// Reject contact request
    pub fn reject_contact<CRID: Into<ContactRequestId>>(
        &self,
        contact_request_id: CRID,
    ) -> impl Future<Output = Result<Arc<ContactRequestRejectedResponse>, C::Error>> {
        self.client.reject_contact(contact_request_id)
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
    pub fn chat_ids(&self) -> impl Future<Output = Result<impl Iterator<Item = ChatId>, C::Error>> {
        self.chat_ids_with(|_| true)
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
    ///    .deliver()
    ///    .await?;
    /// ```
    pub fn prepare_broadcast<M: MessageLike>(
        &self,
        msg: M,
    ) -> impl Future<
        Output = Result<
            MulticastBuilder<'_, impl 'static + Send + Iterator<Item = ChatId>, C, M::Kind>,
            C::Error,
        >,
    > {
        self.prepare_broadcast_with(msg, |_| true)
    }

    /// Generate a [MulticastBuilder] that is ready to send messages to chats matching the filter
    ///
    /// ```rust
    /// bot.prepare_broadcast_with("What do you think about this logo?", |chat| chat.is_direct())
    ///    .await
    ///    .with_image(Image::new("logo.jpg"))
    ///    .deliver()
    ///    .await?;
    /// ```
    pub fn prepare_broadcast_with<M, F>(
        &self,
        msg: M,
        f: F,
    ) -> impl Future<
        Output = Result<
            MulticastBuilder<'_, impl 'static + Send + Iterator<Item = ChatId>, C, M::Kind>,
            C::Error,
        >,
    >
    where
        F: 'static + Send + FnMut(&ChatId) -> bool,
        M: MessageLike,
    {
        let (msg, kind) = msg.into_builder_parts();
        self.chat_ids_with(f).map_ok(move |ids| MulticastBuilder {
            client: self.client(),
            chat_ids: ids,
            ttl: None,
            sign: false,
            msg,
            kind,
        })
    }

    pub fn update_msg<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        new_content: MsgContent,
    ) -> impl Future<Output = Result<ApiUpdateChatItemResponse, C::Error>> {
        self.client.update_message(chat_id, message_id, new_content)
    }

    pub fn delete_msg<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        mode: CIDeleteMode,
    ) -> impl Future<Output = Result<Arc<ChatItemsDeletedResponse>, C::Error>> {
        self.client.delete_message(chat_id, message_id, mode)
    }

    pub fn batch_delete_msgs<CID: Into<ChatId>, I: IntoIterator<Item = MessageId>>(
        &self,
        chat_id: CID,
        message_ids: I,
        mode: CIDeleteMode,
    ) -> impl Future<Output = Result<Arc<ChatItemsDeletedResponse>, C::Error>> {
        self.client
            .batch_delete_messages(chat_id, message_ids, mode)
    }

    /// Applies multiple reactions to a message. Returns one result per reaction.
    pub fn batch_msg_reactions<
        CID: Into<ChatId>,
        MID: Into<MessageId>,
        I: IntoIterator<Item = Reaction>,
    >(
        &self,
        chat_id: CID,
        message_id: MID,
        reactions: I,
    ) -> impl Future<Output = Vec<Result<Arc<ChatItemReactionResponse>, C::Error>>> {
        self.client
            .batch_message_reactions(chat_id, message_id, reactions)
    }

    pub fn update_msg_reaction<CID: Into<ChatId>, MID: Into<MessageId>>(
        &self,
        chat_id: CID,
        message_id: MID,
        reaction: Reaction,
    ) -> impl Future<Output = Vec<Result<Arc<ChatItemReactionResponse>, C::Error>>> {
        self.client
            .update_message_reaction(chat_id, message_id, reaction)
    }

    /// Starts background file download. Catch `RcvFile*` events to track the progress
    pub fn accept_file<FID: Into<FileId>>(&self, file_id: FID) -> AcceptFileBuilder<'_, C> {
        self.client.accept_file(file_id)
    }

    pub fn reject_file<FID: Into<FileId>>(
        &self,
        file_id: FID,
    ) -> impl Future<Output = Result<CancelFileResponse, C::Error>> {
        self.client.reject_file(file_id)
    }

    pub fn delete_chat<CID: Into<ChatId>>(
        &self,
        chat_id: CID,
        mode: DeleteMode,
    ) -> impl Future<Output = Result<ApiDeleteChatResponse, C::Error>> {
        self.client.delete_chat(chat_id, mode)
    }

    /// Create a new group. The bot's user becomes the owner.
    pub fn create_group(
        &self,
        profile: GroupProfile,
    ) -> impl Future<Output = Result<Arc<GroupCreatedResponse>, C::Error>> {
        self.client.create_group(self.user_id(), profile)
    }

    /// Create a new public group with relay members. The bot's user becomes the owner.
    /// Relay IDs can be obtained from [`Bot::default_relays`]
    pub fn create_public_group<I: IntoIterator<Item = RelayId>>(
        &self,
        relay_ids: I,
        profile: GroupProfile,
    ) -> impl Future<Output = Result<ApiNewPublicGroupResponse, C::Error>> {
        self.client
            .create_public_group(self.user_id(), relay_ids, profile)
    }

    /// Enable or disable automatically accepting contacts from group members.
    pub fn set_auto_accept_member_contacts(
        &self,
        on: bool,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, C::Error>> {
        self.client
            .set_auto_accept_member_contacts(self.user_id(), on)
    }

    /// Sends a group invitation to a contact.
    pub fn add_member<GID: Into<GroupId>, CID: Into<ContactId>>(
        &self,
        group_id: GID,
        contact_id: CID,
        role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<SentGroupInvitationResponse>, C::Error>> {
        self.client.add_member(group_id, contact_id, role)
    }

    /// Accepts a pending group invitation.
    pub fn join_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = Result<Arc<UserAcceptedGroupSentResponse>, C::Error>> {
        self.client.join_group(group_id)
    }

    /// Confirms a pending group membership request.
    pub fn accept_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<MemberAcceptedResponse>, C::Error>> {
        self.client.accept_member(group_id, member_id, role)
    }

    pub fn set_members_role<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
        role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<MembersRoleUserResponse>, C::Error>> {
        self.client.set_members_role(group_id, member_ids, role)
    }

    pub fn set_member_role<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
        role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<MembersRoleUserResponse>, C::Error>> {
        self.client.set_member_role(group_id, member_id, role)
    }

    /// Blocks members so their messages are hidden for everyone in the group.
    pub fn block_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = Result<Arc<MembersBlockedForAllUserResponse>, C::Error>> {
        self.client.block_members_for_all(group_id, member_ids)
    }

    /// Reverses a previous [`block_members_for_all`](Self::block_members_for_all).
    pub fn unblock_members_for_all<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = Result<Arc<MembersBlockedForAllUserResponse>, C::Error>> {
        self.client.unblock_members_for_all(group_id, member_ids)
    }

    /// Blocks a member so their messages are hidden for everyone in the group.
    pub fn block_member_for_all<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = Result<Arc<MembersBlockedForAllUserResponse>, C::Error>> {
        self.client.block_member_for_all(group_id, member_id)
    }

    /// Reverses a previous [`block_member_for_all`](Self::block_member_for_all).
    pub fn unblock_member_for_all<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = Result<Arc<MembersBlockedForAllUserResponse>, C::Error>> {
        self.client.unblock_member_for_all(group_id, member_id)
    }

    /// Removes members from the group, preserving their past messages.
    pub fn remove_members<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = Result<Arc<UserDeletedMembersResponse>, C::Error>> {
        self.client.remove_members(group_id, member_ids)
    }

    /// Removes members from the group and deletes their messages.
    pub fn remove_members_with_messages<GID: Into<GroupId>, I: IntoIterator<Item = MemberId>>(
        &self,
        group_id: GID,
        member_ids: I,
    ) -> impl Future<Output = Result<Arc<UserDeletedMembersResponse>, C::Error>> {
        self.client
            .remove_members_with_messages(group_id, member_ids)
    }

    /// Removes a member from the group, preserving their past messages.
    pub fn remove_member<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = Result<Arc<UserDeletedMembersResponse>, C::Error>> {
        self.client.remove_member(group_id, member_id)
    }

    /// Removes a member from the group and deletes their messages.
    pub fn remove_member_with_messages<GID: Into<GroupId>, MID: Into<MemberId>>(
        &self,
        group_id: GID,
        member_id: MID,
    ) -> impl Future<Output = Result<Arc<UserDeletedMembersResponse>, C::Error>> {
        self.client.remove_member_with_messages(group_id, member_id)
    }

    pub fn leave_group<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = Result<Arc<LeftMemberUserResponse>, C::Error>> {
        self.client.leave_group(group_id)
    }

    pub fn list_members<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = Result<Vec<GroupMember>, C::Error>> {
        self.client.list_members(group_id)
    }

    /// Deletes messages for all group members. Requires admin or owner role.
    pub fn moderate_messages<GID: Into<GroupId>, I: IntoIterator<Item = MessageId>>(
        &self,
        group_id: GID,
        message_ids: I,
    ) -> impl Future<Output = Result<Arc<ChatItemsDeletedResponse>, C::Error>> {
        self.client.moderate_messages(group_id, message_ids)
    }

    /// Deletes a message for all group members. Requires admin or owner role.
    pub fn moderate_message<GID: Into<GroupId>, MID: Into<MessageId>>(
        &self,
        group_id: GID,
        message_id: MID,
    ) -> impl Future<Output = Result<Arc<ChatItemsDeletedResponse>, C::Error>> {
        self.client.moderate_message(group_id, message_id)
    }

    pub fn update_group_profile<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        profile: GroupProfile,
    ) -> impl Future<Output = Result<Arc<GroupUpdatedResponse>, C::Error>> {
        self.client.update_group_profile(group_id, profile)
    }

    /// *WARN:* the current impl does full group scan because the Bot API doesn't expose a method to
    /// get gropu by ID.
    pub async fn update_group_profile_with<GID, F>(
        &self,
        group_id: GID,
        updater: F,
    ) -> Result<Arc<GroupUpdatedResponse>, C::Error>
    where
        GID: Into<GroupId>,
        F: FnOnce(&mut GroupProfile),
    {
        let group_id = group_id.into();
        let groups = self.groups().await?;
        let Some(group) = groups.into_iter().find(|g| g.group_id == group_id.raw()) else {
            return Err(BadResponseError::Undocumented(serde_json::json!({
                "type": "groupNotFound",
                "groupId": group_id.raw(),
            }))
            .into());
        };
        let mut profile = group.group_profile;
        updater(&mut profile);
        self.update_group_profile(group_id, profile).await
    }

    pub fn update_group_preferences<GID, F>(
        &self,
        group_id: GID,
        updater: F,
    ) -> impl Future<Output = Result<Arc<GroupUpdatedResponse>, C::Error>>
    where
        GID: Into<GroupId>,
        F: FnOnce(&mut GroupPreferences),
    {
        self.update_group_profile_with(group_id, |profile| {
            let mut prefs = extract_group_preferences(&mut profile.group_preferences);
            updater(&mut prefs);
            profile.group_preferences = Some(prefs);
        })
    }

    pub fn set_group_sign_messages<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        on: bool,
    ) -> impl Future<Output = Result<Arc<GroupUpdatedResponse>, C::Error>> {
        self.update_group_preferences(group_id, move |prefs| {
            prefs.sign_messages = if on {
                preferences::group::YES
            } else {
                preferences::group::NO
            };
        })
    }

    /// Stores arbitrary app-defined JSON on the group. Pass `None` to clear it.
    pub fn set_group_custom_data<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        data: Option<JsonObject>,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, C::Error>> {
        self.client.set_group_custom_data(group_id, data)
    }

    /// Stores arbitrary app-defined JSON on the contact. Pass `None` to clear it.
    pub fn set_contact_custom_data<CID: Into<ContactId>>(
        &self,
        contact_id: CID,
        data: Option<JsonObject>,
    ) -> impl Future<Output = Result<Arc<CmdOkResponse>, C::Error>> {
        self.client.set_contact_custom_data(contact_id, data)
    }

    pub fn create_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> impl Future<Output = Result<Arc<GroupLinkCreatedResponse>, C::Error>> {
        self.client.create_group_link(group_id, role)
    }

    /// Changes the default role assigned to members who join via the group link.
    pub fn set_group_link_role<GID: Into<GroupId>>(
        &self,
        group_id: GID,
        role: GroupMemberRole,
    ) -> impl Future<Output = GroupLinkResult<C>> {
        self.client.set_group_link_role(group_id, role)
    }

    pub fn delete_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = Result<Arc<GroupLinkDeletedResponse>, C::Error>> {
        self.client.delete_group_link(group_id)
    }

    pub fn get_group_link<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = GroupLinkResult<C>> {
        self.client.get_group_link(group_id)
    }

    pub fn get_group_relays<GID: Into<GroupId>>(
        &self,
        group_id: GID,
    ) -> impl Future<Output = GetGroupRelaysResponse<C>> {
        self.client.get_group_relays(group_id)
    }

    pub fn add_group_relays<GID: Into<GroupId>, I: IntoIterator<Item = RelayId>>(
        &self,
        group_id: GID,
        relay_ids: I,
    ) -> impl Future<Output = AddGroupRelaysResponse<C>> {
        self.client.add_group_relays(group_id, relay_ids)
    }

    pub fn add_group_relay<GID: Into<GroupId>, RID: Into<RelayId>>(
        &self,
        group_id: GID,
        relay_id: RID,
    ) -> impl Future<Output = AddGroupRelaysResponse<C>> {
        self.client.add_group_relay(group_id, relay_id)
    }

    /// Get chats with time-based pagination. Prefer this over [`Bot::contacts`] / [`Bot::groups`]
    /// for large databases as it avoids loading all records into memory at once.
    pub fn get_chats(
        &self,
        pagination: PaginationByTime,
        query: ChatListQuery,
    ) -> impl Future<Output = Result<Arc<ApiChatsResponse>, C::Error>> {
        self.client.get_chats(self.user_id(), pagination, query)
    }

    /// Get a list of default user relays
    pub fn default_relays(&self) -> impl Future<Output = Result<Vec<RelayId>, C::Error>> {
        self.client.default_relays()
    }

    /// Accept an incoming remote control session from a SimpleX Desktop client.
    ///
    /// Requires a [`CtrlHandle`](crate::remote::CtrlHandle) installed on the event
    /// stream via [`EventStream::hook_remote_control`](crate::EventStream::hook_remote_control).
    ///
    /// # Deadlock warning
    ///
    /// See [`CtrlHandle::accept_remote_ctrl`](crate::remote::CtrlHandle::accept_remote_ctrl).
    pub fn accept_remote_ctrl(
        &self,
        handle: &crate::remote::CtrlHandle,
        link: &str,
    ) -> impl Future<Output = Result<(), crate::remote::CtrlError<C::Error>>> {
        handle.accept_remote_ctrl(&self.client, link)
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
    pub display_name: BotName,
    /// If string is empty creates an auto-accepting address without a message. If string is not
    /// empty adds a welcome message to the address
    pub auto_accept: Option<String>,
    pub profile_settings: Option<BotProfileSettings>,
    pub avatar: Option<ImagePreview>,
    pub bio: Option<String>,
    pub description: Option<String>,
}

impl BotSettings {
    pub fn new(display_name: impl Into<BotName>) -> Self {
        Self {
            display_name: display_name.into(),
            auto_accept: None,
            profile_settings: None,
            avatar: None,
            bio: None,
            description: None,
        }
    }

    pub fn with_avatar(mut self, avatar: ImagePreview) -> Self {
        self.avatar = Some(avatar);
        self
    }

    pub fn with_bio(mut self, bio: impl Into<String>) -> Self {
        self.bio = Some(bio.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
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
pub enum BotName {
    Current(String),
    Rename { from: Vec<String>, to: String },
}

impl<S: Into<String>> From<S> for BotName {
    fn from(name: S) -> Self {
        BotName::Current(name.into())
    }
}

impl BotName {
    pub fn new(name: impl Into<String>) -> Self {
        Self::Current(name.into())
    }

    pub fn rename<S: Into<String>>(
        from: impl IntoIterator<Item = S>,
        to: impl Into<String>,
    ) -> Self {
        let from = from.into_iter().map(|s| s.into()).collect();
        let to = to.into();
        Self::Rename { from, to }
    }

    pub(crate) fn current(&self) -> String {
        match self {
            Self::Current(name) | Self::Rename { from: _, to: name } => name.clone(),
        }
    }

    pub(crate) fn matches_new(&self, name: &String) -> bool {
        match self {
            Self::Current(current) => current == name,
            Self::Rename { from: _, to } => to == name,
        }
    }

    pub(crate) fn matches_old(&self, name: &String) -> bool {
        match self {
            Self::Current(current) => current == name,
            Self::Rename { from, to: _ } => from.contains(name),
        }
    }

    #[cfg(feature = "farm")]
    pub(crate) fn matches(&self, name: &String) -> bool {
        match self {
            Self::Current(current) => current == name,
            Self::Rename { from, to } => to == name || from.contains(name),
        }
    }

    /// - Matches the current user name with the highest priority.
    /// - Otherwise matches the first user to rename.
    /// - Returns None if no matches were found.
    pub(crate) fn match_user<'a>(&self, users: &'a mut [UserInfo]) -> Option<&'a mut User> {
        let mut existing_user = None;

        for info in users {
            if self.matches_new(&info.user.profile.display_name) {
                existing_user = Some(&mut info.user);
                break;
            }

            if self.matches_old(&info.user.profile.display_name) {
                existing_user.get_or_insert(&mut info.user);
            }
        }

        existing_user
    }
}

// ~350 vs ~650 bytes
#[allow(clippy::large_enum_variant)]
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
        description: local.description.take(),
        image: local.image.take(),
        contact_link: local.contact_link.take(),
        contact_domain: local.contact_domain.take(),
        preferences: local.preferences.take(),
        peer_type: local.peer_type.take(),
        badge: None,
        undocumented: std::mem::take(&mut local.undocumented),
    }
}

fn extract_group_preferences(prefs: &mut Option<GroupPreferences>) -> GroupPreferences {
    match prefs.as_mut() {
        Some(p) => GroupPreferences {
            timed_messages: p.timed_messages.take(),
            direct_messages: p.direct_messages.take(),
            full_delete: p.full_delete.take(),
            reactions: p.reactions.take(),
            voice: p.voice.take(),
            files: p.files.take(),
            simplex_links: p.simplex_links.take(),
            reports: p.reports.take(),
            history: p.history.take(),
            support: p.support.take(),
            sessions: p.sessions.take(),
            comments: p.comments.take(),
            sign_messages: p.sign_messages.take(),
            commands: p.commands.take(),
            undocumented: std::mem::take(&mut p.undocumented),
        },
        None => GroupPreferences {
            timed_messages: None,
            direct_messages: None,
            full_delete: None,
            reactions: None,
            voice: None,
            files: None,
            simplex_links: None,
            reports: None,
            history: None,
            support: None,
            sessions: None,
            comments: None,
            sign_messages: None,
            commands: None,
            undocumented: Default::default(),
        },
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
