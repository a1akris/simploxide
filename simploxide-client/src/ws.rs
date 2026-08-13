//! WebSocket backend that connects to a `simplex-chat` WebSocket server.
//!
//! Use [`BotBuilder`] to launch or connect to `simplex-chat` and get a ready-to-use [`Bot`].
//! For lower-level access, [`connect`] and [`retry_connect`] return a [`Client`] and an
//! [`EventStream`](crate::EventStream) directly.

use std::{ffi, sync::Arc};

use futures::TryFutureExt as _;
pub use simploxide_ws_core::{
    self as core, Error as CoreError, Event as CoreEvent, Result as CoreResult, SimplexVersion,
    VersionError, tungstenite::Error as WsError,
};

#[cfg(feature = "cli")]
pub use simploxide_ws_core::cli;

use serde::Deserialize;
use simploxide_api_types::{
    Preferences, Profile,
    client_api::{ExtractResponse, WebSocketResponseShape, WebSocketResponseShapeInner},
    events::{Event, EventKind},
};
use simploxide_core::{MAX_SUPPORTED_VERSION, MIN_SUPPORTED_VERSION};
use simploxide_ws_core::RawClient;

use crate::{
    BadResponseError, ClientApi, ClientApiError, EventParser,
    bot::{BotName, BotProfileSettings, BotSettings},
    id::UserId,
    preview::ImagePreview,
    util,
};

pub type EventResult = CoreResult<CoreEvent>;
pub type EventStream = crate::EventStream<EventResult>;
pub type ClientResult<T = ()> = ::std::result::Result<T, ClientError>;

#[cfg(not(feature = "xftp"))]
pub type Bot = crate::bot::Bot<Client>;

#[cfg(feature = "xftp")]
pub type Bot = crate::bot::Bot<crate::xftp::XftpClient<Client>>;

#[cfg(feature = "farm")]
pub type FarmBot = crate::bot::farm::FarmBot<Client>;

#[cfg(feature = "farm")]
pub type InitFarm = crate::bot::farm::InitFarm<Client, EventResult>;

#[cfg(feature = "farm")]
pub type RunningFarm = crate::bot::farm::RunningFarm<Client, EventResult>;

/// Connects to a `simplex-chat` WebSocket server, returning a [`Client`] and an [`EventStream`]
/// that handle serialization/deserialization of commands and events.
///
/// ```ignore
/// let (client, mut events) = simploxide_client::ws::connect("ws://127.0.0.1:5225").await?;
///
/// let current_user = client.api_show_active_user().await?;
/// println!("{current_user:#?}");
///
/// while let Some(ev) = events.try_next().await? {
///     // Process events...
/// }
/// ```
pub async fn connect<S: AsRef<str>>(uri: S) -> Result<(Client, EventStream), ConnectError> {
    let (raw_client, raw_event_queue) = simploxide_ws_core::connect(uri.as_ref()).await?;

    let version = raw_client
        .version()
        .await
        .map_err(ConnectError::VersionError)?;

    if !version.is_supported() {
        return Err(ConnectError::VersionMismatch(version));
    }

    Ok((
        Client::from(raw_client),
        EventStream::from(raw_event_queue.into_receiver()),
    ))
}

/// Like [`connect`] but retries to connect `retries_count` times before returning an error. This
/// method is needed when you run simplex-cli programmatically and don't know when WebSocket port
/// becomes available.
///
/// ```ignore
/// let port = 5225;
/// let cli = SimplexCli::spawn(port);
/// let uri = format!("ws://127.0.0.1:{port}");
///
/// let (client, mut events) = simploxide_client::retry_connect(&uri, Duration::from_secs(1), 10).await?;
///
/// //...
///
/// ```
pub async fn retry_connect<S: AsRef<str>>(
    uri: S,
    retry_delay: std::time::Duration,
    mut retries_count: usize,
) -> Result<(Client, EventStream), ConnectError> {
    loop {
        match connect(uri.as_ref()).await {
            Ok(connection) => break Ok(connection),
            Err(e) if !e.is_server() || retries_count == 0 => break Err(e),
            Err(_) => {
                retries_count -= 1;
                tokio::time::sleep(retry_delay).await
            }
        }
    }
}

impl EventParser for EventResult {
    type Error = ClientError;

    fn parse_kind(&self) -> Result<EventKind, Self::Error> {
        match parse_data::<util::TypeField<'_>>(self) {
            Ok(f) => Ok(EventKind::from_type_str(f.typ)),
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
            Err(ClientError::BadResponse(BadResponseError::Undocumented(json))) => {
                Ok(Event::Undocumented(json))
            }
            Err(e) => Err(e),
        }
    }
}

fn parse_data<'de, 'r: 'de, D: 'de + Deserialize<'de>>(res: &'r EventResult) -> ClientResult<D> {
    res.as_ref()
        .map_err(|e| ClientError::WebSocketFailure(e.clone()))
        .and_then(|ev| {
            serde_json::from_str::<EventShape<D>>(ev)
                .map_err(BadResponseError::InvalidJson)
                .and_then(|shape| shape.extract_response())
                .map_err(ClientError::BadResponse)
        })
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum EventShape<T> {
    ResponseShape(WebSocketResponseShape<T>),
    InlineShape(WebSocketResponseShapeInner<T>),
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for EventShape<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        match self {
            Self::ResponseShape(resp) => resp.extract_response(),
            Self::InlineShape(inline) => inline.extract_response(),
        }
    }
}

/// A high level SimpleX-Chat client which provides typed API methods with automatic command
/// serialization and response deserialization.
#[derive(Clone)]
pub struct Client {
    inner: RawClient,
}

impl From<RawClient> for Client {
    fn from(inner: RawClient) -> Self {
        Self { inner }
    }
}

impl Client {
    pub async fn version(&self) -> Result<SimplexVersion, VersionError> {
        tokio::time::timeout(std::time::Duration::from_secs(2), self.inner.version())
            .await
            .map_err(|_| VersionError::Timeout)
            .flatten()
    }

    /// Initiates a graceful shutdown for the underlying web socket connection. See
    /// [`simploxide_ws_core::RawClient::disconnect`] for details.
    pub fn disconnect(self) -> impl Future<Output = ()> {
        self.inner.disconnect()
    }
}

impl ClientApi for Client {
    type ResponseShape<'de, T>
        = WebSocketResponseShape<T>
    where
        T: 'de + Deserialize<'de>;

    type Error = ClientError;

    async fn send_raw(&self, command: String) -> Result<String, Self::Error> {
        self.inner
            .send(command)
            .await
            .map_err(ClientError::WebSocketFailure)
    }
}

/// See [`crate::client_api::AllowUndocumentedResponses`] if you don't want to trigger an error when
/// you receive undocumeted responses(you usually receive undocumented responses when your
/// simplex-chat server version is not compatible with the simploxide-client version. Keep an eye
/// on the
/// [Version compatability table](https://github.com/a1akris/simploxide?tab=readme-ov-file#version-compatability-table)
/// )
#[derive(Debug)]
pub enum ClientError {
    /// Critical error signalling that the web socket connection is dropped for some reason. You
    /// will have to reconnect to the SimpleX server to recover from this one.
    WebSocketFailure(CoreError),
    /// SimpleX command error or unexpected(undocumented) response.
    BadResponse(BadResponseError),
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WebSocketFailure(error) => Some(error),
            Self::BadResponse(error) => Some(error),
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::WebSocketFailure(err) => writeln!(f, "Web socket failure: {err}"),
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
pub enum ConnectError {
    /// Failure to establish the connection to the server
    Server(CoreError),
    /// Failure to get the server version
    VersionError(VersionError),
    /// Unsupported server version
    VersionMismatch(SimplexVersion),
}

impl ConnectError {
    pub fn is_server(&self) -> bool {
        matches!(self, Self::Server(_))
    }

    pub fn is_version_mismatch(&self) -> bool {
        matches!(self, Self::VersionMismatch(_))
    }
}

impl From<WsError> for ConnectError {
    fn from(value: WsError) -> Self {
        Self::Server(Arc::new(value))
    }
}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Server(error) => write!(f, "Cannot connect to the server: {error}"),
            Self::VersionError(error) => write!(f, "Cannot get the server version: {error}"),
            Self::VersionMismatch(v) => write!(
                f,
                "Version {v} is unsupported by the current client. Supported versions are {MIN_SUPPORTED_VERSION}..{MAX_SUPPORTED_VERSION}"
            ),
        }
    }
}

impl std::error::Error for ConnectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Server(error) => Some(error),
            Self::VersionError(error) => Some(error),
            Self::VersionMismatch(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct BotBuilder {
    inner: WsBotBuilder,
    settings: BotSettings,
    #[cfg(feature = "cli")]
    cli: cli::SimplexCliBuilder,
}

impl BotBuilder {
    pub fn new(name: impl Into<BotName>, port: u16) -> Self {
        let name = name.into();

        Self {
            inner: WsBotBuilder::new(port),
            #[cfg(feature = "cli")]
            cli: cli::SimplexCli::builder(name.current(), port),
            settings: BotSettings::new(name),
        }
    }

    #[cfg(feature = "cli")]
    /// Path prefix for the SimpleX database
    ///
    /// "{dir}/{prefix}" creates a {dir} with `{prefix}_agent.db` and `{prefix}_chat.db`;
    /// "{prefix}" creates `{prefix}_agent.db` and `{prefix}_chat.db` at the current dir
    pub fn db_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.cli = self.cli.db_prefix(prefix);
        self
    }

    #[cfg(feature = "cli")]
    /// Database encryption key.
    pub fn db_key(mut self, key: impl Into<String>) -> Self {
        self.cli = self.cli.db_key(key);
        self
    }

    /// Pass extra argument to the `simplex-chat` process.
    #[cfg(feature = "cli")]
    pub fn arg(mut self, arg: impl Into<ffi::OsString>) -> Self {
        self.cli = self.cli.arg(arg);
        self
    }

    /// Pass extra arguments to the `simplex-chat` process.
    #[cfg(feature = "cli")]
    pub fn args<J, S>(mut self, args: J) -> Self
    where
        J: IntoIterator<Item = S>,
        S: Into<std::ffi::OsString>,
    {
        self.cli = self.cli.args(args);
        self
    }

    /// Delay between connection retry attempt. Default: 1s
    pub fn connect_retry_delay(mut self, delay: std::time::Duration) -> Self {
        self.inner.retry_delay = delay;
        self
    }

    /// Number of connection retry attempts. Default: 5
    pub fn retries(mut self, n: usize) -> Self {
        self.inner.retries = n;
        self
    }

    /// Create public address and auto accept users
    pub fn auto_accept(mut self) -> Self {
        self.settings.auto_accept = Some(String::default());
        self
    }

    /// Set a welcome message. This automatically creates a public address with enabled auto_accept
    pub fn auto_accept_with(mut self, welcome_message: impl Into<String>) -> Self {
        self.settings.auto_accept = Some(welcome_message.into());
        self
    }

    /// Set the bot avatar during initialisation
    pub fn with_avatar(mut self, avatar: ImagePreview) -> Self {
        self.settings.avatar = Some(avatar);
        self
    }

    /// Set the bot bio (`short_descr`) during initialisation. Ignored when [`Self::with_profile`] is also set.
    pub fn with_bio(mut self, bio: impl Into<String>) -> Self {
        self.settings.bio = Some(bio.into());
        self
    }

    /// Set the bot description during initialisation. Ignored when [`Self::with_profile`] is also set.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.settings.description = Some(description.into());
        self
    }

    /// Update/create the whole bot profile on launch
    pub fn with_profile(mut self, profile: Profile) -> Self {
        self.settings = self
            .settings
            .merge_profile_settings(BotProfileSettings::FullProfile(profile));

        self
    }

    /// Apply these preferences to the bot's profile during initialisation.
    pub fn with_preferences(mut self, prefs: Preferences) -> Self {
        self.settings = self
            .settings
            .merge_profile_settings(BotProfileSettings::Preferences(prefs));

        self
    }

    /// Connect to an already-running `simplex-chat` instance.
    pub fn connect(self) -> impl Future<Output = Result<(Bot, EventStream), BotInitError>> {
        Self::connect_inner(self.inner, self.settings)
    }

    async fn connect_inner(
        inner: WsBotBuilder,
        settings: BotSettings,
    ) -> Result<(Bot, EventStream), BotInitError> {
        let (client, events) = inner.into_connection().await?;

        #[cfg(feature = "xftp")]
        let (client, events) = events.hook_xftp(client);

        let bot = Bot::init(client, settings).await?;

        let mut events = events;
        events.set_owner(bot.user_id());

        Ok((bot, events))
    }

    /// Spawn `simplex-chat` CLI process, then connect and initialise.
    ///
    /// Returns `(bot, events, cli)`. The caller is responsible for calling
    /// [`cli::SimplexCli::kill`] after the bot finishes.
    #[cfg(feature = "cli")]
    pub async fn launch(self) -> Result<(Bot, EventStream, cli::SimplexCli), BotInitError> {
        let cli = gracefully_spawn_cli(self.cli).await?;
        let (bot, events) = Self::connect_inner(self.inner, self.settings).await?;

        Ok((bot, events, cli))
    }
}

#[cfg(feature = "farm")]
#[derive(Clone)]
pub struct BotFarmBuilder {
    name: String,
    inner: WsBotBuilder,
    #[cfg(feature = "cli")]
    cli: cli::SimplexCliBuilder,
}

#[cfg(feature = "farm")]
impl BotFarmBuilder {
    pub fn new(name: impl Into<String>, port: u16) -> Self {
        let name = name.into();

        Self {
            #[cfg(feature = "cli")]
            cli: cli::SimplexCli::builder(name.clone(), port),
            inner: WsBotBuilder::new(port),
            name,
        }
    }

    #[cfg(feature = "cli")]
    /// Path prefix for the SimpleX database
    ///
    /// "{dir}/{prefix}" creates a {dir} with `{prefix}_agent.db` and `{prefix}_chat.db`;
    /// "{prefix}" creates `{prefix}_agent.db` and `{prefix}_chat.db` at the current dir
    pub fn db_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.cli = self.cli.db_prefix(prefix);
        self
    }

    #[cfg(feature = "cli")]
    /// Database encryption key.
    pub fn db_key(mut self, key: impl Into<String>) -> Self {
        self.cli = self.cli.db_key(key);
        self
    }

    /// Pass extra argument to the `simplex-chat` process.
    #[cfg(feature = "cli")]
    pub fn arg(mut self, arg: impl Into<ffi::OsString>) -> Self {
        self.cli = self.cli.arg(arg);
        self
    }

    /// Pass extra arguments to the `simplex-chat` process.
    #[cfg(feature = "cli")]
    pub fn args<J, S>(mut self, args: J) -> Self
    where
        J: IntoIterator<Item = S>,
        S: Into<std::ffi::OsString>,
    {
        self.cli = self.cli.args(args);
        self
    }

    /// Delay between connection retry attempt. Default: 1s
    pub fn connect_retry_delay(mut self, delay: std::time::Duration) -> Self {
        self.inner.retry_delay = delay;
        self
    }

    /// Number of connection retry attempts. Default: 5
    pub fn retries(mut self, n: usize) -> Self {
        self.inner.retries = n;
        self
    }

    /// Connect to an already-running `simplex-chat` instance.
    pub fn connect(self) -> impl Future<Output = Result<InitFarm, BotInitError>> {
        Self::connect_inner(self.name, self.inner)
    }

    async fn connect_inner(name: String, inner: WsBotBuilder) -> Result<InitFarm, BotInitError> {
        let (client, events) = inner.into_connection().await?;
        let farm = crate::bot::BotFarm::init(name, client, events).await?;
        Ok(farm)
    }

    #[cfg(feature = "cli")]
    /// Spawn `simplex-chat`, then connect and initialise.
    ///
    /// Returns `(farm, cli)`. The caller is responsible for calling
    /// [`cli::SimplexCli::kill`] after the farm finishes.
    pub async fn launch(self) -> Result<(InitFarm, cli::SimplexCli), BotInitError> {
        let cli = gracefully_spawn_cli(self.cli).await?;
        let farm = Self::connect_inner(self.name, self.inner).await?;
        Ok((farm, cli))
    }
}

#[derive(Clone)]
struct WsBotBuilder {
    port: u16,
    retry_delay: std::time::Duration,
    retries: usize,
}

impl WsBotBuilder {
    fn new(port: u16) -> Self {
        Self {
            port,
            retry_delay: std::time::Duration::from_secs(1),
            retries: 5,
        }
    }

    fn into_connection(self) -> impl Future<Output = Result<(Client, EventStream), BotInitError>> {
        let url = format!("ws://127.0.0.1:{}", self.port);
        retry_connect(url, self.retry_delay, self.retries).map_err(BotInitError::Connect)
    }
}

#[cfg(feature = "cli")]
async fn gracefully_spawn_cli(
    cli: cli::SimplexCliBuilder,
) -> Result<cli::SimplexCli, BotInitError> {
    let port = cli.port();

    match cli.spawn().await {
        Ok(cli) => Ok(cli),
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("(Address already in use)") {
                Ok(cli::SimplexCli::external(port))
            } else {
                Err(BotInitError::CliSpawn(e))
            }
        }
    }
}

/// Error returned by [`BotBuilder::connect`] and [`BotBuilder::launch`].
#[derive(Debug)]
pub enum BotInitError {
    Connect(ConnectError),
    Api(ClientError),
    #[cfg(feature = "cli")]
    CliSpawn(std::io::Error),
}

impl std::fmt::Display for BotInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "cli")]
            Self::CliSpawn(e) => write!(f, "failed to spawn simplex-chat: {e}"),
            Self::Connect(e) => write!(f, "websocket connection failed: {e}"),
            Self::Api(e) => write!(f, "SimpleX API error during init: {e}"),
        }
    }
}

impl std::error::Error for BotInitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "cli")]
            Self::CliSpawn(e) => Some(e),
            Self::Connect(e) => Some(e),
            Self::Api(e) => Some(e),
        }
    }
}

impl From<ClientError> for BotInitError {
    fn from(e: ClientError) -> Self {
        Self::Api(e)
    }
}
