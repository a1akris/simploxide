pub use simploxide_ws_core::{
    self as core, Error as CoreError, Event as CoreEvent, Result as CoreResult,
    tungstenite::Error as WsError,
};

#[cfg(feature = "cli")]
pub use simploxide_ws_core::cli;

use serde::Deserialize;
use simploxide_api_types::{
    Preferences, Profile,
    client_api::{ExtractResponse, WebSocketResponseShape, WebSocketResponseShapeInner},
    events::{Event, EventKind},
};
use simploxide_ws_core::RawClient;

use crate::{
    BadResponseError, ClientApi, ClientApiError, EventParser,
    bot::{BotProfileSettings, BotSettings},
};

pub type Bot = crate::bot::Bot<Client>;
pub type EventStream = crate::EventStream<CoreResult<CoreEvent>>;
pub type ClientResult<T = ()> = ::std::result::Result<T, ClientError>;

/// A wrapper over [`simploxide_core::connect`] that turns [`simploxide_core::RawClient`] into
/// [`Client`] and raw event queue into the [`EventStream`] which handle serialization/deserialization.
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
pub async fn connect<S: AsRef<str>>(uri: S) -> Result<(Client, EventStream), WsError> {
    let (raw_client, raw_event_queue) = simploxide_ws_core::connect(uri.as_ref()).await?;
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
) -> Result<(Client, EventStream), WsError> {
    loop {
        match connect(uri.as_ref()).await {
            Ok(connection) => break Ok(connection),
            Err(e) if retries_count == 0 => break Err(e),
            Err(_) => {
                retries_count -= 1;
                tokio::time::sleep(retry_delay).await
            }
        }
    }
}

impl EventParser for CoreResult<String> {
    type Error = ClientError;

    fn parse_kind(&self) -> Result<EventKind, Self::Error> {
        #[derive(Deserialize)]
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

fn parse_data<'de, 'r: 'de, D: 'de + Deserialize<'de>>(
    res: &'r CoreResult<String>,
) -> ClientResult<D> {
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

pub struct BotBuilder {
    name: String,
    port: u16,
    retry_delay: std::time::Duration,
    retries: usize,
    auto_reply: Option<String>,
    profile: Option<Profile>,
    preferences: Option<Preferences>,
    #[cfg(feature = "cli")]
    db_prefix: String,
    #[cfg(feature = "cli")]
    db_key: Option<String>,
    #[cfg(feature = "cli")]
    extra_args: Vec<std::ffi::OsString>,
}

impl BotBuilder {
    pub fn new(name: impl Into<String>, port: u16) -> Self {
        Self {
            name: name.into(),
            port,
            db_prefix: "bot".into(),
            db_key: None,
            retry_delay: std::time::Duration::from_secs(1),
            retries: 5,
            auto_reply: None,
            profile: None,
            preferences: None,
            #[cfg(feature = "cli")]
            extra_args: Vec::new(),
        }
    }

    #[cfg(feature = "cli")]
    /// Path prefix for the SimpleX database
    ///
    /// "{dir}/{prefix}" creates a {dir} with `{prefix}_agent.db` and `{prefix}_chat.db` {prefix}
    /// creates `{prefix}_agent.db` and `{prefix}_chat.db` at the current dir
    pub fn db_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.db_prefix = prefix.into();
        self
    }

    #[cfg(feature = "cli")]
    /// Database encryption key.
    pub fn db_key(mut self, key: impl Into<String>) -> Self {
        self.db_key = Some(key.into());
        self
    }

    /// Delay between connection retry attempt. Default: 1s
    pub fn connect_retry_delay(mut self, delay: std::time::Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Number of connection retry attempts. Default: 5
    pub fn retries(mut self, n: usize) -> Self {
        self.retries = n;
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

    /// Pass extra arguments to the `simplex-chat` process.
    #[cfg(feature = "cli")]
    pub fn cli_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<std::ffi::OsString>,
    {
        self.extra_args.extend(args.into_iter().map(|s| s.into()));
        self
    }

    /// Connect to an already-running `simplex-chat` instance.
    pub async fn connect(self) -> Result<(Bot, EventStream), BotInitError> {
        let url = format!("ws://127.0.0.1:{}", self.port);

        let (client, events) = retry_connect(url, self.retry_delay, self.retries)
            .await
            .map_err(BotInitError::Connect)?;

        let settings = BotSettings {
            display_name: self.name,
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

    /// Spawn `simplex-chat`, then connect and initialise.
    ///
    /// Returns `(bot, events, cli)`. The caller is responsible for calling
    /// [`cli::SimplexCli::kill`] after the bot finishes.
    #[cfg(feature = "cli")]
    pub async fn launch(mut self) -> Result<(Bot, EventStream, cli::SimplexCli), BotInitError> {
        let mut builder = cli::SimplexCli::builder(&self.name, self.port)
            .db_prefix(std::mem::take(&mut self.db_prefix));

        if let Some(ref mut key) = self.db_key {
            builder = builder.db_key(std::mem::take(key));
        }

        let cli = builder
            .args(std::mem::take(&mut self.extra_args))
            .spawn()
            .await
            .map_err(BotInitError::CliSpawn)?;

        let (bot, events) = self.connect().await?;
        Ok((bot, events, cli))
    }
}

/// Error returned by [`BotBuilder::connect`] and [`BotBuilder::launch`].
#[derive(Debug)]
pub enum BotInitError {
    Connect(WsError),
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
