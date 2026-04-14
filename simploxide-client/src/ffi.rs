pub use simploxide_ffi_core::{CallError, DbOpts, DefaultUser, InitError, WorkerConfig};

use simploxide_api_types::{
    Preferences, Profile,
    client_api::{ExtractResponse as _, FfiResponseShape},
    events::{Event, EventKind},
};
use simploxide_ffi_core::{Event as CoreEvent, RawClient, Result as CoreResult};

use std::sync::Arc;

use crate::{
    BadResponseError, ClientApi, ClientApiError, EventParser,
    bot::{BotProfileSettings, BotSettings},
};

pub type Bot = crate::bot::Bot<Client>;
pub type EventStream = crate::EventStream<CoreResult<CoreEvent>>;
pub type ClientResult<T = ()> = ::std::result::Result<T, ClientError>;

pub async fn init(
    default_user: DefaultUser,
    db_opts: DbOpts,
) -> Result<(Client, EventStream), InitError> {
    init_with_config(default_user, db_opts, WorkerConfig::default()).await
}

pub async fn init_with_config(
    default_user: DefaultUser,
    db_opts: DbOpts,
    config: WorkerConfig,
) -> Result<(Client, EventStream), InitError> {
    let (raw_client, raw_event_queue) =
        simploxide_ffi_core::init_with_config(default_user, db_opts, config).await?;
    Ok((
        Client::from(raw_client),
        EventStream::from(raw_event_queue.into_receiver()),
    ))
}

/// A cheaply clonable high-level FFI client implementing [`ClientApi`]
#[derive(Clone)]
pub struct Client {
    inner: RawClient,
}

impl From<RawClient> for Client {
    fn from(inner: RawClient) -> Self {
        Self { inner }
    }
}

/// A high level SimpleX-Chat client which provides typed API methods with automatic command
/// serialization and response deserialization.
impl Client {
    /// Initiates a graceful shutdown for the underlying web socket connection. See
    /// [`simploxide_ffi_core::RawClient::disconnect`] for details.
    pub fn disconnect(self) -> impl Future<Output = ()> {
        self.inner.disconnect()
    }
}

impl ClientApi for Client {
    type ResponseShape<'de, T>
        = FfiResponseShape<T>
    where
        T: 'de + serde::Deserialize<'de>;

    type Error = ClientError;

    async fn send_raw(&self, command: String) -> Result<String, Self::Error> {
        self.inner
            .send(command)
            .await
            .map_err(ClientError::FfiFailure)
    }
}

impl EventParser for CoreResult<CoreEvent> {
    type Error = ClientError;

    fn parse_kind(&self) -> Result<EventKind, Self::Error> {
        #[derive(serde::Deserialize)]
        struct TypeField<'a> {
            #[serde(rename = "type", borrow)]
            typ: &'a str,
        }

        match parse_data::<TypeField<'_>>(self) {
            Ok(f) => Ok(EventKind::from_type_str(f.typ)),
            Err(ClientError::BadResponse(BadResponseError::Undocumented(_))) => {
                Ok(EventKind::Undocumented)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_event(&self) -> Result<Event, Self::Error> {
        parse_data(self)
    }
}

fn parse_data<'de, 'r: 'de, D: 'de + serde::Deserialize<'de>>(
    result: &'r CoreResult<CoreEvent>,
) -> Result<D, ClientError> {
    result
        .as_ref()
        .map_err(|e| ClientError::FfiFailure(e.clone()))
        .and_then(|ev| {
            serde_json::from_str::<FfiResponseShape<D>>(ev)
                .map_err(BadResponseError::InvalidJson)
                .and_then(|shape| shape.extract_response())
                .map_err(ClientError::BadResponse)
        })
}

/// Builder for an FFI-backed [`Bot`].
pub struct BotBuilder {
    display_name: String,
    db_opts: DbOpts,
    default_user: Option<DefaultUser>,
    auto_reply: Option<String>,
    profile: Option<Profile>,
    preferences: Option<Preferences>,
    worker_config: WorkerConfig,
}

impl BotBuilder {
    /// Build a bot account (default).
    pub fn new(name: impl Into<String>, db_opts: DbOpts) -> Self {
        Self {
            display_name: name.into(),
            db_opts,
            default_user: None,
            auto_reply: None,
            profile: None,
            preferences: None,
            worker_config: WorkerConfig::default(),
        }
    }

    /// Override the default user created for empty databases.
    ///
    /// By default the default user name matches the bot name. This setting allows to create a user
    /// different from an active bot
    pub fn with_default_user(mut self, user: DefaultUser) -> Self {
        self.default_user = Some(user);
        self
    }

    /// Create public address and auto accept users
    pub fn auto_accept(mut self) -> Self {
        self.auto_reply = Some(String::default());
        self
    }

    /// Set a welcome message. This automatically creates a public address with enabled auto_accept
    pub fn auto_reply(mut self, auto_reply: impl Into<String>) -> Self {
        self.auto_reply = Some(auto_reply.into());
        self
    }

    /// Update/create the whole bot profile on launch
    pub fn with_profile(mut self, profile: Profile) -> Self {
        self.profile = Some(profile);
        self
    }

    /// Apply these preferences to the bot's profile during initialisation.
    pub fn with_preferences(mut self, prefs: Preferences) -> Self {
        self.preferences = Some(prefs);
        self
    }

    /// Set max permissible event latency. See [`WorkerConfig::max_event_latency`] for details
    pub fn max_event_latency(mut self, latency: std::time::Duration) -> Self {
        self.worker_config.max_event_latency = Some(latency);
        self
    }

    /// Set max concurrent SimpleX-Chat instances. See [`WorkerConfig::max_instances`] for details
    pub fn max_instances(mut self, instances: usize) -> Self {
        self.worker_config.max_instances = Some(instances);
        self
    }

    /// Initialise the SimpleX FFI runtime and return a ready-to-use bot.
    pub async fn launch(
        self,
    ) -> Result<(Bot, crate::EventStream<CoreResult<CoreEvent>>), BotInitError> {
        let default_user = self
            .default_user
            .unwrap_or_else(|| DefaultUser::bot(&self.display_name));

        let (client, events) = init_with_config(default_user, self.db_opts, self.worker_config)
            .await
            .map_err(BotInitError::Init)?;

        let settings = BotSettings {
            display_name: self.display_name,
            auto_reply: self.auto_reply,
            profile_settings: match (self.profile, self.preferences) {
                (Some(mut profile), Some(preferences)) => {
                    profile.preferences = Some(preferences);
                    Some(BotProfileSettings::FullProfile(profile))
                }
                (Some(profile), None) => Some(BotProfileSettings::FullProfile(profile)),
                (None, Some(preferences)) => Some(BotProfileSettings::Preferences(preferences)),
                (None, None) => None,
            },
        };

        let bot = Bot::init(client, settings).await?;
        Ok((bot, events))
    }
}

/// See [`crate::client_api::AllowUndocumentedResponses`] if you don't want to trigger an error
/// when you receive undocumeted responses(you usually receive undocumented responses when your
/// simplex-chat version is not compatible with the current simploxide-client version. Keep an eye
/// on the [Version compatability table](https://github.com/a1akris/simploxide?tab=readme-ov-file#version-compatability-table))
#[derive(Debug)]
pub enum ClientError {
    FfiFailure(Arc<CallError>),
    BadResponse(BadResponseError),
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FfiFailure(error) => Some(error),
            Self::BadResponse(error) => Some(error),
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::FfiFailure(err) => writeln!(f, "FFI error: {err}"),
            ClientError::BadResponse(err) => err.fmt(f),
        }
    }
}

impl From<BadResponseError> for ClientError {
    fn from(err: BadResponseError) -> Self {
        Self::BadResponse(err)
    }
}

impl ClientApiError for ClientError {
    fn bad_response(&self) -> Option<&BadResponseError> {
        if let Self::BadResponse(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError> {
        if let Self::BadResponse(resp) = self {
            Some(resp)
        } else {
            None
        }
    }
}

/// Error returned by [`BotBuilder::launch`].
#[derive(Debug)]
pub enum BotInitError {
    Init(InitError),
    Api(ClientError),
}

impl std::fmt::Display for BotInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init(e) => write!(f, "SimpleX FFI init failed: {e}"),
            Self::Api(e) => write!(f, "SimpleX API error during init: {e}"),
        }
    }
}

impl std::error::Error for BotInitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Init(e) => Some(e),
            Self::Api(e) => Some(e),
        }
    }
}

impl From<ClientError> for BotInitError {
    fn from(e: ClientError) -> Self {
        Self::Api(e)
    }
}
