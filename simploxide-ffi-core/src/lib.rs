//! A fully asynchrouns raw SimpleX client backed by the SimpleX FFI bindings(see
//! [simploxide_sxcrt_sys] for setup instructions) that provides:
//!
//! 1. Multi-instance support: run many SimpleX-Chat instances from a single process. Each instance
//!    is fully isolated and all are served by a single shared worker thread with fair round-robin
//!    scheduling and per-instance execution caps to prevent starvation.
//!
//! 2. Complete asynchonisity: futures created by the same instance of a client are fully
//!    independent from each other. The event queue receives events independently from client
//!    actions.
//!
//! 3. Graceful shutdown with strong guarantees:
//!     - All commands enqueued before [`RawClient::disconnect`] are guaranteed to execute and
//!       return their responses.
//!
//!     - All commands enqueued after [`RawClient::disconnect`] are guaranteed to return
//!       [`CallError::Failure`] without being executed.
//!
//!     - You will receive events for as long as the chat instance is active. After disconnect the
//!       remaining buffered events are delivered and then the event queue closes.
//!
//! -----
//!
//! _Current implementation heavily depends on `tokio` runtime and won't work with other
//! executors._

pub use simploxide_core::SimplexVersion;
pub use simploxide_sxcrt_sys::{CallError, InitError, MigrationConfirmation};

pub mod default;

mod worker;

use serde::Deserialize;

use std::{path::Path, sync::Arc, time::Duration};

pub type Command = String;
pub type Event = String;
pub type Response = String;

pub type Result<T = (), E = Arc<CallError>> = ::std::result::Result<T, E>;

type FfiResponder = tokio::sync::oneshot::Sender<Result<Response>>;

type CmdTransmitter = std::sync::mpsc::Sender<ChatCommand>;
type CmdReceiver = std::sync::mpsc::Receiver<ChatCommand>;

type EventTransmitter = tokio::sync::mpsc::UnboundedSender<Result<Event>>;
pub type EventReceiver = tokio::sync::mpsc::UnboundedReceiver<Result<Event>>;

type ShutdownEmitter = tokio::sync::watch::Sender<bool>;
type ShutdownSignal = tokio::sync::watch::Receiver<bool>;

/// Configuration for the shared FFI worker thread.
///
/// Applies only on the first [`init`] or [`init_with_config`] call. All subsequent calls reuse
/// the already running worker thread and ignore this parameter entirely.
#[derive(Default, Debug, Clone)]
pub struct WorkerConfig {
    /// Maximum permissible event latency. Controls how long the worker thread may sleep between
    /// polling cycles when all chats are idle. The sleep interval grows linearly from zero up to
    /// this value as idle time accumulates. Sending any command resets the interval immediately by
    /// waking the thread. Default: [`default::MAX_EVENT_LATENCY`].
    pub max_event_latency: Option<std::time::Duration>,

    /// Maximum number of chat instances the worker thread will serve simultaneously. [`init`]
    /// returns [`CallError::Failure`] when this limit is reached. Passing `0` is valid but
    /// prevents any chat from ever being created. Default: [`default::MAX_CHAT_INSTANCES`].
    pub max_instances: Option<usize>,
}

impl WorkerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_event_latency(mut self, duration: Duration) -> Self {
        self.max_event_latency = Some(duration);
        self
    }

    pub fn max_instances(mut self, max_instances: usize) -> Self {
        self.max_instances = Some(max_instances);
        self
    }
}

/// Open a SimpleX database with default [`WorkerConfig`] and start receiving events.
///
/// See [`init_with_config`] for full documentation.
pub async fn init(
    default_user: DefaultUser,
    db_opts: DbOpts,
) -> Result<(RawClient, RawEventQueue), InitError> {
    init_with_config(default_user, db_opts, WorkerConfig::default()).await
}

/// Open a SimpleX database and start receiving events.
///
/// Returns a [`RawClient`] for sending commands and a [`RawEventQueue`] that buffers incoming chat
/// events independently of client activity. Each init call creates a fully isolated instance with
/// its own client and event queue, shutting down one instance does not affect any other.
///
/// All FFI calls like event polling and command execution are running on a single shared OS
/// thread. Creating a new instance blocks this thread for the full duration of the database
/// initialisation(including migrations). All other currently active chat instances are frozen
/// during the execution of this method.
///
/// # Memory
///
/// The [`RawEventQueue`] is backed by an unbounded channel. If events are not consumed they
/// accumulate indefinitely. Either process events promptly or drop the queue immediately if your
/// application does not need them.
///
///
/// # Example
///
/// ```ignore
/// let (client, mut events) = simploxide_ffi_core::init_with_config(
///     DefaultUser::bot("MyBot"),
///     DbOpts::unencrypted("./data/mybot"),
///     WorkerConfig::new().max_instances(4),
/// ).await?;
///
/// // (Optional) Drop the event queue if you're not planning to handle events
/// drop(events)
///
/// // Get SimpleX runtime version
/// let version = client.version().await?;
/// ```
pub async fn init_with_config(
    default_user: DefaultUser,
    db_opts: DbOpts,
    config: WorkerConfig,
) -> Result<(RawClient, RawEventQueue), InitError> {
    worker::init(config).spawn_chat(default_user, db_opts).await
}

/// A lightweight cheaply clonable client for sending raw requests(SimpleX commands) and receiving
/// raw responses(JSON objects).
///
/// You can use the client behind a shared reference, or you can clone it, in both cases the
/// created futures will be indpenendent from each other.
#[derive(Clone)]
pub struct RawClient {
    tx: CmdTransmitter,
    worker: worker::Worker,
    shutdown: ShutdownSignal,
}

impl RawClient {
    /// Send a raw SimpleX command and await its response.
    ///
    /// The command is sent immediately and the returned future directly awaits the response from
    /// the worker thread.
    pub async fn send(&self, command: Command) -> Result<Response> {
        let (responder, response) = tokio::sync::oneshot::channel();

        self.tx
            .send(ChatCommand::Execute(command, responder))
            .map_err(|_| CallError::Failure)?;

        self.worker.wake();

        response.await.map_err(|_| CallError::Failure)?
    }

    /// Returns the version of the underlying SimpleX runtime.
    pub async fn version(&self) -> Result<SimplexVersion, VersionError> {
        #[derive(Deserialize)]
        struct VersionResult<'a> {
            #[serde(borrow)]
            result: VersionInfo<'a>,
        }

        #[derive(Deserialize)]
        struct VersionInfo<'a> {
            #[serde(borrow, rename = "versionInfo")]
            version_info: VersionData<'a>,
        }

        #[derive(Deserialize)]
        struct VersionData<'a> {
            #[serde(borrow)]
            version: &'a str,
        }

        let output = self.send("/v".to_owned()).await?;

        let response = serde_json::from_str::<VersionResult>(&output)
            .map_err(|e| Arc::new(CallError::InvalidJson(e)))?
            .result
            .version_info
            .version;

        let version = response
            .parse()
            .map_err(|_| VersionError::ParseError(response.to_owned()))?;

        Ok(version)
    }

    /// Initiates a graceful shutdown and waits until the database is fully closed.
    ///
    /// All futures that got scheduled before this call will still receive their responses. All
    /// futures scheduled after this call(from cloned clients) will resolve immediately with
    /// [`CallError::Failure`].
    ///
    /// If you don't care about waiting for the graceful shutdown to complete you can just drop the
    /// future, the shutdown will still be triggered
    ///
    /// ```ignore
    /// let _ = client.disconnect();
    /// ```
    ///
    /// or use [`tokio::time::timeout`] to limit the wait time
    ///
    /// ```ignore
    /// tokio::time::timeout(Duration::from_secs(5), client.disconnect())
    ///     .await
    ///     .unwrap_or_default();
    /// ```
    ///
    /// # Racing with [`Self::send`]
    ///
    /// Commands and the disconnect signal share the same FIFO channel. Whichever call enqueues
    /// first is processed first: If `disconnect` enqueues before a concurrent `send` the `send`
    /// future returns [`CallError::Failure`] and the command is guaranteed not to have been
    /// executed, if `send` enqueues before `disconnect` - the command executes normally.
    ///
    /// To guarantee ordering, await all `send` futures to completion before calling `disconnect`.
    pub fn disconnect(mut self) -> impl Future<Output = ()> {
        let _ = self.tx.send(ChatCommand::Disconnect);
        self.worker.wake();

        async move {
            let _ = self.shutdown.wait_for(|b| *b).await;
        }
    }
}

/// An event queue that buffers incoming SimpleX events independently of client activity.
///
/// Backed by an unbounded channel. If events are not consumed they accumulate indefinitely. Drop
/// the queue as soon as it is no longer needed. When dropped while a chat instance is active and
/// producing events, the Haskell-side queue is still drained continuously - events are discarded
/// in Rust and do not accumulate in the FFI layer.
pub struct RawEventQueue {
    receiver: EventReceiver,
}

impl RawEventQueue {
    /// Returns the next event from the queue, or `None` if the chat has shut down.
    pub async fn next_event(&mut self) -> Option<Result<Event>> {
        self.receiver.recv().await
    }

    /// Unwraps the queue into the underlying tokio unbounded receiver for more advanced use cases.
    pub fn into_receiver(self) -> EventReceiver {
        self.receiver
    }
}

/// The SimpleX user profile used to initialise the chat instance.
///
/// # Security
///
/// The `display_name` field is injected into a SimpleX CLI command of the form `/create {kind}
/// '{display_name}'`. It is intended to be a short, fixed, ASCII identifier chosen by the
/// application author, do not supply a user-input here to avoid command injections like:
/// "User'Name"(creates a user named "User" with bio="Name")
#[derive(Debug, Clone)]
pub struct DefaultUser {
    pub display_name: String,
    pub is_bot: bool,
}

impl DefaultUser {
    /// Creates a regular SimpleX user profile with the given display name.
    ///
    /// `name` is injected literally into `/create user '{name}'`. Use a fixed ASCII identifier;
    /// do not pass user-supplied input here.
    pub fn regular<S: Into<String>>(name: S) -> Self {
        Self {
            display_name: name.into(),
            is_bot: false,
        }
    }

    /// Creates a bot SimpleX user profile with the given display name.
    ///
    /// `name` is injected literally into `/create bot '{name}'`. Use a fixed ASCII identifier;
    /// do not pass user-supplied input here.
    pub fn bot<S: Into<String>>(name: S) -> Self {
        Self {
            display_name: name.into(),
            is_bot: true,
        }
    }
}

/// Database options for a SimpleX chat instance.
///
/// # The `prefix` field
///
/// SimpleX stores each chat instance as a set of files sharing a common path prefix. The prefix
/// is a directory path plus a filename stem: the directory part is created if absent, and the
/// stem is prepended to every database filename.
///
/// ```text
/// prefix: "data/bot" - creates ./data/bot_agent.db, ./data/bot_chat.db
/// prefix: "bot" - creates ./bot_agent.db, ./bot_chat.db
/// ```
///
/// # Warning: overlapping prefixes
///
/// Two instances whose prefixes share the same directory and stem will silently read and write the
/// same files. This will produce DB errors and may cause DB corruptions
#[derive(Debug, Clone)]
pub struct DbOpts {
    pub prefix: String,
    pub key: Option<String>,
    pub migration: MigrationConfirmation,
}

impl DbOpts {
    /// Open an unencrypted SimpleX database at the given path prefix.
    ///
    /// See [`DbOpts`] for an explanation of what `prefix` means and the overlap warning.
    pub fn unencrypted<P: AsRef<Path>>(db_path: P) -> Self {
        Self {
            prefix: db_path.as_ref().display().to_string(),
            key: None,
            migration: MigrationConfirmation::YesUp,
        }
    }

    /// Open an encrypted SimpleX database at the given path prefix with the given passphrase.
    ///
    /// See [`DbOpts`] for an explanation of what `prefix` means and the overlap warning.
    pub fn encrypted<P: AsRef<Path>, K: Into<String>>(prefix: P, key: K) -> Self {
        Self {
            prefix: prefix.as_ref().display().to_string(),
            key: Some(key.into()),
            migration: MigrationConfirmation::YesUp,
        }
    }
}

/// Error returned by [`RawClient::version`].
#[derive(Debug)]
pub enum VersionError {
    Ffi(Arc<CallError>),
    ParseError(String),
}

impl From<Arc<CallError>> for VersionError {
    fn from(value: Arc<CallError>) -> Self {
        Self::Ffi(value)
    }
}

impl std::fmt::Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ffi(e) => e.fmt(f),
            Self::ParseError(s) => {
                write!(
                    f,
                    "Cannot parse version, expected format: '<major>.<minor>.<patch>.<hotfix>', got {s:?}"
                )
            }
        }
    }
}

impl std::error::Error for VersionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ffi(e) => Some(e),
            Self::ParseError(_) => None,
        }
    }
}

enum ChatCommand {
    Execute(Command, FfiResponder),
    Disconnect,
}
