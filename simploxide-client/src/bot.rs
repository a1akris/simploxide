use simploxide_api_types::{
    AddressSettings, AutoAccept, ChatPeerType, Contact, CreatedConnLink, GroupInfo, LocalProfile,
    MsgContent, NewUser, PendingContactConnection, Preferences, Profile, User,
    client_api::{
        AllowUndocumentedResponses as _, ClientApi, ClientApiError as _, UndocumentedResponse,
    },
    commands::{ApiAddContact, ApiSetActiveUser, ApiSetProfileAddress},
    responses::{
        ApiDeleteChatResponse, ApiUpdateProfileResponse, ConnectResponse,
        ContactPrefsUpdatedResponse, NewChatItemsResponse, UserProfileUpdatedResponse,
    },
};

use std::sync::Arc;

use crate::{
    ext::{ClientApiExt as _, DeleteMode, MessageBuilder, MessageLike, MulticastBuilder},
    id::{ChatId, ContactId, UserId},
    preferences,
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
    pub async fn init(client: C, settings: BotSettings) -> Result<Self, C::Error> {
        let mut users = client.users().await?;

        match users.iter_mut().find_map(|info| {
            (info.user.profile.display_name == settings.display_name).then_some(&mut info.user)
        }) {
            Some(user) => {
                if !user.active_user {
                    client
                        .api_set_active_user(ApiSetActiveUser::new(user.user_id))
                        .await?;
                }

                let bot = Bot {
                    client,
                    user_id: user.user_id,
                };

                if let Some(auto_reply) = settings.auto_reply {
                    if user.profile.contact_link.is_none() {
                        bot.get_or_create_address().await?;
                        bot.publish_address().await?;
                    }

                    bot.configure_address(AddressSettings {
                        business_address: false,
                        auto_accept: Some(AutoAccept {
                            accept_incognito: false,
                            undocumented: Default::default(),
                        }),
                        auto_reply: (!auto_reply.is_empty())
                            .then(|| MsgContent::make_text(auto_reply)),
                        undocumented: Default::default(),
                    })
                    .await?;
                } else {
                    if user.profile.contact_link.is_some() {
                        bot.configure_address(AddressSettings {
                            business_address: false,
                            auto_accept: None,
                            auto_reply: None,
                            undocumented: Default::default(),
                        })
                        .await?;
                    }
                }

                if let Some(profile_settings) = settings.profile_settings {
                    let profile = match profile_settings {
                        BotProfileSettings::Preferences(preferences) => {
                            let mut current_profile = extract_profile(&mut user.profile);
                            current_profile.preferences = Some(preferences);
                            current_profile
                        }
                        BotProfileSettings::FullProfile(profile) => profile,
                    };

                    bot.client()
                        .api_update_profile(user.user_id, profile)
                        .await?;
                }

                Ok(bot)
            }
            None => {
                let bot_profile = match settings.profile_settings {
                    Some(BotProfileSettings::Preferences(preferences)) => {
                        let mut profile = Self::default_profile(settings.display_name.clone());
                        profile.preferences = Some(preferences);
                        profile
                    }
                    Some(BotProfileSettings::FullProfile(profile)) => profile,
                    None => Self::default_profile(settings.display_name.clone()),
                };

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

                if let Some(auto_reply) = settings.auto_reply {
                    bot.create_address().await?;
                    bot.publish_address().await?;

                    bot.configure_address(AddressSettings {
                        business_address: false,
                        auto_accept: Some(AutoAccept {
                            accept_incognito: true,
                            undocumented: Default::default(),
                        }),
                        auto_reply: Some(MsgContent::make_text(auto_reply)),
                        undocumented: Default::default(),
                    })
                    .await?;
                }

                Ok(bot)
            }
        }
    }

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
    pub async fn info(&self) -> Result<Arc<User>, C::Error> {
        let response = self.client.show_active_user().await?;
        Ok(Arc::new(response.user.clone()))
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
        self.client
            .initiate_connection(link)
            .await
            .allow_undocumented()
    }

    /// Create one-time-invitation link. Can be used for admin-access or for private connections
    /// with other bots. The [PendingContactConnection::pcc_conn_id] can be matched with
    /// [crate::types::Connection::conn_id] to recognize the user connected by this link when handling the
    /// [crate::events::ContactConnected] event(see [crate::events::ContactConnected::contact])
    pub async fn create_invitation_link(
        &self,
    ) -> Result<(String, Arc<PendingContactConnection>), C::Error> {
        let response = self
            .client
            .api_add_contact(ApiAddContact::new(self.user_id))
            .await?;

        let link = extract_address(&response.conn_link_invitation);
        let pcc = Arc::new(response.connection.clone());

        Ok((link, pcc))
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

    pub async fn update_profile<F>(&self, updater: F) -> Result<ApiUpdateProfileResponse, C::Error>
    where
        F: 'static + Send + FnOnce(&mut Profile),
    {
        let mut response = self.client().show_active_user().await?;
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
    pub async fn set_image(
        &self,
        // TODO: Make a helper type InlineImage type
        image: impl Into<String>,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        let image = image.into();
        self.update_profile(move |profile| profile.image = Some(image))
            .await
    }

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

    /// Get all contacts known to the bot(connected or not)
    pub async fn contacts(&self) -> Result<Vec<Contact>, C::Error> {
        self.client.contacts(self.user_id()).await
    }

    /// Get all groups known to the bot
    pub async fn groups(&self) -> Result<Vec<GroupInfo>, C::Error> {
        self.client.groups(self.user_id()).await
    }

    /// [ChatId] can be created from various types. See [ChatId] docs for the full list of `From`
    /// impls.
    pub fn send_msg<CID: Into<ChatId>, M: MessageLike>(
        &self,
        chat_id: CID,
        msg: M,
    ) -> MessageBuilder<'_, C> {
        self.client.send_message(chat_id.into(), msg)
    }

    /// Send the same message to multiple recepients
    pub fn multicast<I, M>(&self, chat_ids: I, msg: M) -> MulticastBuilder<'_, I, C>
    where
        I: IntoIterator<Item = ChatId>,
        M: MessageLike,
    {
        self.client.multicast_message(chat_ids, msg)
    }

    /// Broadcast the same message to all existing contacts(groups included)
    pub async fn broadcast<M>(
        &self,
        msg: M,
    ) -> Result<Vec<Result<Arc<NewChatItemsResponse>, C::Error>>, C::Error>
    where
        C: 'static,
        C::Error: 'static + Send,
        M: MessageLike,
    {
        self.bcst(msg, None, |_| true).await
    }

    /// Broadcast the same message to all existing contacts(groups included) matching the filter
    pub async fn broadcast_filter<M, F>(
        &self,
        msg: M,
        f: F,
    ) -> Result<Vec<Result<Arc<NewChatItemsResponse>, C::Error>>, C::Error>
    where
        C: 'static,
        C::Error: 'static + Send,
        M: MessageLike,
        F: FnMut(&ChatId) -> bool,
    {
        self.bcst(msg, None, f).await
    }

    /// Broadcast the same message with TTL to all existing contacts(groups included)
    pub async fn broadcast_with_ttl<M>(
        &self,
        msg: M,
        ttl: std::time::Duration,
    ) -> Result<Vec<Result<Arc<NewChatItemsResponse>, C::Error>>, C::Error>
    where
        C: 'static,
        C::Error: 'static + Send,
        M: MessageLike,
    {
        self.bcst(msg, Some(ttl), |_| true).await
    }

    /// Broadcast the same message with TTL to all existing contacts(groups included) matching the
    /// filter
    pub async fn broadcast_filter_with_ttl<M, F>(
        &self,
        msg: M,
        ttl: std::time::Duration,
        f: F,
    ) -> Result<Vec<Result<Arc<NewChatItemsResponse>, C::Error>>, C::Error>
    where
        C: 'static,
        C::Error: 'static + Send,
        M: MessageLike,
        F: FnMut(&ChatId) -> bool,
    {
        self.bcst(msg, Some(ttl), f).await
    }

    async fn bcst<F, M>(
        &self,
        msg: M,
        ttl: Option<std::time::Duration>,
        f: F,
    ) -> Result<Vec<Result<Arc<NewChatItemsResponse>, C::Error>>, C::Error>
    where
        F: FnMut(&ChatId) -> bool,
        M: MessageLike,
        C: 'static,
        C::Error: 'static + Send,
    {
        let (contacts, groups) = futures::future::try_join(self.contacts(), self.groups()).await?;

        let ids = contacts
            .iter()
            .map(ChatId::from)
            .chain(groups.iter().map(ChatId::from))
            .filter(f);

        match ttl {
            Some(ttl) => Ok(self.multicast(ids, msg).with_ttl(ttl).await),
            None => Ok(self.multicast(ids, msg).await),
        }
    }

    /// [ChatId] can be created from various types. See [ChatId] docs for the full list of `From`
    /// impls.
    pub async fn delete_chat<CID: Into<ChatId>>(
        &self,
        chat_id: CID,
        mode: DeleteMode,
    ) -> Result<ApiDeleteChatResponse, C::Error> {
        self.client.delete_chat(chat_id, mode).await
    }
}

#[derive(Debug, Clone)]
pub struct BotSettings {
    pub display_name: String,
    /// If string is empty creates an auto-accepting address without a message
    pub auto_reply: Option<String>,
    pub profile_settings: Option<BotProfileSettings>,
}

impl BotSettings {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            display_name: name.into(),
            auto_reply: None,
            profile_settings: None,
        }
    }

    /// Create a public auto-accepting address during the intialisation
    pub fn auto_accept(mut self) -> Self {
        self.auto_reply = Some(String::default());
        self
    }

    /// Create a public auto-accepting address with a welcome meesage during the intialisation
    pub fn auto_reply(mut self, reply: impl Into<String>) -> Self {
        self.auto_reply = Some(reply.into());
        self
    }

    pub fn with_profile_settings(mut self, settings: BotProfileSettings) -> Self {
        self.profile_settings = Some(settings);
        self
    }
}

#[derive(Debug, Clone)]
pub enum BotProfileSettings {
    Preferences(Preferences),
    FullProfile(Profile),
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
