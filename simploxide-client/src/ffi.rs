//! FFI backend that embeds the SimpleX-Chat library in-process via native Rust bindings.
//!
//! Use [`BotBuilder`] to initialise the FFI runtime and get a ready-to-use [`Bot`].
//! For lower-level access, [`init`] and [`init_with_config`] return a [`Client`] and an
//! [`EventStream`](crate::EventStream) directly.
//!
//! Requires AGPL-3.0 and additional build configuration. See `simploxide-sxcrt-sys`.

pub use simploxide_ffi_core::{
    CallError, DbOpts, DefaultUser, InitError as CoreInitError, SimplexVersion, VersionError,
    WorkerConfig,
};

use simploxide_api_types::{
    Preferences, Profile,
    client_api::{ExtractResponse as _, FfiResponseShape},
    events::{Event, EventKind},
};
use simploxide_core::{MAX_SUPPORTED_VERSION, MIN_SUPPORTED_VERSION};
use simploxide_ffi_core::{Event as CoreEvent, RawClient, Result as CoreResult};

use std::sync::Arc;

use crate::{
    BadResponseError, ClientApi, ClientApiError, EventParser,
    bot::{BotProfileSettings, BotSettings},
    id::UserId,
    preview::ImagePreview,
    util,
};

#[cfg(not(feature = "xftp"))]
pub type Bot = crate::bot::Bot<Client>;

#[cfg(feature = "xftp")]
pub type Bot = crate::bot::Bot<crate::xftp::XftpClient<Client>>;

#[cfg(feature = "farm")]
pub type FarmBot = crate::bot::farm::FarmBot<Client>;

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

    let version = raw_client
        .version()
        .await
        .map_err(InitError::VersionError)?;

    if !version.is_supported() {
        return Err(InitError::VersionMismatch(version));
    }

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
    pub fn version(&self) -> impl Future<Output = Result<SimplexVersion, VersionError>> {
        self.inner.version()
    }

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
        match parse_data::<util::TypeField<'_>>(self) {
            Ok(f) => Ok(EventKind::from_type_str(f.typ)),
            // FFI chat error shapes are the same for events and responses which confuses the parser therefore
            // chat errors must be handled manually
            Err(ClientError::BadResponse(BadResponseError::ChatError(_))) => {
                Ok(EventKind::ChatError)
            }
            Err(ClientError::BadResponse(BadResponseError::Undocumented(_))) => {
                Ok(EventKind::Undocumented)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_user_id(&self) -> Result<Option<UserId>, Self::Error> {
        match parse_data::<util::UserField>(self) {
            Ok(f) => Ok(UserId::try_from(f.user.user_id).ok()),
            Err(ClientError::BadResponse(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn parse_event(&self) -> Result<Event, Self::Error> {
        match parse_data(self) {
            Ok(ev) => Ok(ev),
            // FFI chat error shapes are the same for events and responses which confuses the parser therefore
            // chat errors must be handled manually
            Err(ClientError::BadResponse(BadResponseError::ChatError(err))) => Ok(
                Event::ChatError(Arc::new(simploxide_api_types::events::ChatError {
                    chat_error: err.as_ref().clone(),
                    undocumented: Default::default(),
                })),
            ),
            Err(ClientError::BadResponse(BadResponseError::Undocumented(json))) => {
                Ok(Event::Undocumented(json))
            }
            Err(e) => Err(e),
        }
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
#[derive(Clone)]
pub struct BotBuilder {
    display_name: String,
    db_opts: DbOpts,
    default_user: Option<DefaultUser>,
    auto_accept: Option<String>,
    profile: Option<Profile>,
    preferences: Option<Preferences>,
    avatar: Option<ImagePreview>,
    worker_config: WorkerConfig,
}

impl BotBuilder {
    /// Build a bot account (default).
    pub fn new(name: impl Into<String>, db_opts: DbOpts) -> Self {
        Self {
            display_name: name.into(),
            db_opts,
            default_user: None,
            auto_accept: None,
            profile: None,
            preferences: None,
            avatar: None,
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
        self.auto_accept = Some(String::default());
        self
    }

    /// [Self::auto_accept] with a welcome message
    pub fn auto_accept_with(mut self, welcome_message: impl Into<String>) -> Self {
        self.auto_accept = Some(welcome_message.into());
        self
    }

    /// Set the bot avatar during initialisation
    pub fn with_avatar(mut self, avatar: ImagePreview) -> Self {
        self.avatar = Some(avatar);
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

        #[cfg(feature = "xftp")]
        let (client, events) = events.hook_xftp(client);

        let settings = BotSettings {
            display_name: self.display_name,
            auto_accept: self.auto_accept,
            profile_settings: match (self.profile, self.preferences) {
                (Some(mut profile), Some(preferences)) => {
                    profile.preferences = Some(preferences);
                    Some(BotProfileSettings::FullProfile(profile))
                }
                (Some(profile), None) => Some(BotProfileSettings::FullProfile(profile)),
                (None, Some(preferences)) => Some(BotProfileSettings::Preferences(preferences)),
                (None, None) => None,
            },
            avatar: self.avatar,
        };

        let bot = Bot::init(client, settings).await?;

        let mut events = events;
        events.set_owner(bot.user_id());

        Ok((bot, events))
    }
}

#[cfg(feature = "farm")]
#[derive(Clone)]
pub struct BotFarmBuilder {
    display_name: String,
    db_opts: DbOpts,
    default_user: Option<DefaultUser>,
    worker_config: WorkerConfig,
}

#[cfg(feature = "farm")]
impl BotFarmBuilder {
    pub fn new(name: impl Into<String>, db_opts: DbOpts) -> Self {
        Self {
            display_name: name.into(),
            db_opts,
            default_user: None,
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

    /// Initialise the SimpleX FFI runtime and return a farm
    pub async fn launch(
        self,
    ) -> Result<
        crate::bot::BotFarm<crate::bot::farm::Init<Client, CoreResult<CoreEvent>>>,
        BotInitError,
    > {
        let default_user = self
            .default_user
            .unwrap_or_else(|| DefaultUser::bot(&self.display_name));

        let (client, events) = init_with_config(default_user, self.db_opts, self.worker_config)
            .await
            .map_err(BotInitError::Init)?;

        let bot = crate::bot::BotFarm::init(self.display_name, client, events).await?;
        Ok(bot)
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

#[derive(Debug)]
pub enum InitError {
    /// Failure to init the FFI instance
    Ffi(CoreInitError),
    /// Failure to get the backend version
    VersionError(VersionError),
    /// Unsupported backend version
    VersionMismatch(SimplexVersion),
}

impl InitError {
    pub fn is_ffi(&self) -> bool {
        matches!(self, Self::Ffi(_))
    }

    pub fn is_version_mismatch(&self) -> bool {
        matches!(self, Self::VersionMismatch(_))
    }
}

impl From<CoreInitError> for InitError {
    fn from(value: CoreInitError) -> Self {
        Self::Ffi(value)
    }
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ffi(error) => write!(f, "Cannot initialize the FFI backend: {error}"),
            Self::VersionError(error) => write!(f, "Cannot get FFI version {error}"),
            Self::VersionMismatch(v) => write!(
                f,
                "Version {v} is unsupported by the current client. Supported versions are {MIN_SUPPORTED_VERSION}..{MAX_SUPPORTED_VERSION}"
            ),
        }
    }
}

impl std::error::Error for InitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ffi(error) => Some(error),
            Self::VersionError(error) => Some(error),
            Self::VersionMismatch(_) => None,
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
