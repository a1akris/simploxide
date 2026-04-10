use simploxide_api_types::{
    AddressSettings, AutoAccept, ChatPeerType, CreatedConnLink, LocalProfile, MsgContent, NewUser,
    Preferences, Profile,
    client_api::{ClientApi, ClientApiError as _},
    commands::{ApiSetActiveUser, ApiSetProfileAddress},
    responses::{ApiUpdateProfileResponse, UserProfileUpdatedResponse},
};

use std::sync::Arc;

use crate::{
    ext::{ClientApiExt as _, MessageBuilder, MessageLike},
    id::{ChatId, UserId},
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
        let mut response = client.list_users().await?;
        let response = Arc::get_mut(&mut response).unwrap();

        match response.users.iter_mut().find_map(|info| {
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

    pub async fn create_address(&self) -> Result<String, C::Error> {
        let response = self.client.api_create_my_address(self.user_id).await?;
        Ok(extract_address(&response.conn_link_contact))
    }

    /// Throws [StoreError::UserContactLinkNotFound] if bot doesn't have an address. Use
    /// [get_or_create_address] to ensure that address is available
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

    pub async fn set_preferences(
        &self,
        preferences: Preferences,
    ) -> Result<ApiUpdateProfileResponse, C::Error> {
        self.update_profile(move |profile| profile.preferences = Some(preferences))
            .await
    }

    pub fn send_msg<M: MessageLike>(&self, chat_id: ChatId, msg: M) -> MessageBuilder<'_, C> {
        self.client().send_message(chat_id, msg)
    }
}

#[derive(Debug, Clone)]
pub struct BotSettings {
    pub display_name: String,
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

    pub fn auto_accept(mut self, reply: impl Into<String>) -> Self {
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
