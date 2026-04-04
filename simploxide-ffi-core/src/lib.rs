pub use simploxide_core::SimplexVersion;
pub use simploxide_sxcrt_sys::{CallError, InitError, MigrationConfirmation};

use serde::Deserialize;
use simploxide_sxcrt_sys::SimpleXChat;

use std::{path::Path, sync::mpsc::RecvTimeoutError};

pub type Command = String;
pub type Event = String;
pub type Response = String;

type FfiResponder = tokio::sync::oneshot::Sender<Result<Response, CallError>>;

type ChatWorkerTransmitter = std::sync::mpsc::Sender<ChatWorkerCommand>;
type ChatWorkerReceiver = std::sync::mpsc::Receiver<ChatWorkerCommand>;

type EventTransmitter = tokio::sync::mpsc::UnboundedSender<Result<Event, CallError>>;
pub type EventReceiver = tokio::sync::mpsc::UnboundedReceiver<Result<Event, CallError>>;

pub async fn init(
    default_user: DefaultUser,
    db_opts: DbOpts,
) -> Result<(RawClient, RawEventQueue), simploxide_sxcrt_sys::InitError> {
    let (tx, rx) = std::sync::mpsc::channel();
    let (ev_tx, ev_rx) = tokio::sync::mpsc::unbounded_channel();
    let (responder, response) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || match chat_init(default_user, db_opts) {
        Ok(chat) => {
            let _ = responder.send(Ok((RawClient { tx }, RawEventQueue { receiver: ev_rx })));
            chat_worker(chat, rx, ev_tx);
        }
        Err(e) => {
            let _ = responder.send(Err(e));
        }
    });

    response.await.unwrap()
}

#[derive(Clone)]
pub struct RawClient {
    tx: ChatWorkerTransmitter,
}

impl RawClient {
    pub async fn send(&self, command: Command) -> Result<Response, CallError> {
        let (responder, response) = tokio::sync::oneshot::channel();

        self.tx
            .send(ChatWorkerCommand::Execute(command, responder))
            .map_err(|_| CallError::Failure)?;

        response.await.map_err(|_| CallError::Failure)?
    }

    /// Returns version of underlying SimpleX runtime
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
            .map_err(CallError::InvalidJson)?
            .result
            .version_info
            .version;

        let version = response
            .parse()
            .map_err(|_| VersionError::ParseError(response.to_owned()))?;

        Ok(version)
    }

    pub fn disconnect(self) {
        let _ = self.tx.send(ChatWorkerCommand::Disconnect);
    }
}

pub struct RawEventQueue {
    receiver: EventReceiver,
}

impl RawEventQueue {
    pub async fn next_event(&mut self) -> Option<Result<Event, CallError>> {
        self.receiver.recv().await
    }

    pub fn into_receiver(self) -> EventReceiver {
        self.receiver
    }
}

#[derive(Debug, Clone)]
pub struct DefaultUser {
    pub display_name: String,
    pub is_bot: bool,
}

impl DefaultUser {
    /// Creates regular SimpleX profile
    pub fn regular<S: Into<String>>(name: S) -> Self {
        Self {
            display_name: name.into(),
            is_bot: false,
        }
    }

    /// Creates bot profile
    pub fn bot<S: Into<String>>(name: S) -> Self {
        Self {
            display_name: name.into(),
            is_bot: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbOpts {
    pub prefix: String,
    pub key: Option<String>,
    pub migration: MigrationConfirmation,
}

impl DbOpts {
    pub fn unencrypted<P: AsRef<Path>>(db_path: P) -> Self {
        Self {
            prefix: db_path.as_ref().display().to_string(),
            key: None,
            migration: MigrationConfirmation::YesUp,
        }
    }

    pub fn encrypted<P: AsRef<Path>, K: Into<String>>(prefix: P, key: K) -> Self {
        Self {
            prefix: prefix.as_ref().display().to_string(),
            key: Some(key.into()),
            migration: MigrationConfirmation::YesUp,
        }
    }
}

#[derive(Debug)]
pub enum VersionError {
    Ffi(CallError),
    ParseError(String),
}

impl From<CallError> for VersionError {
    fn from(value: CallError) -> Self {
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

fn chat_init(default_user: DefaultUser, db_opts: DbOpts) -> Result<SimpleXChat, InitError> {
    let mut chat = SimpleXChat::init(
        db_opts.prefix,
        db_opts.key.unwrap_or_default(),
        db_opts.migration,
    )?;

    let output = chat.send_cmd("/users".to_owned())?;

    if output.contains("\"users\":[]") {
        let subject = if default_user.is_bot { "bot" } else { "user" };
        let output = chat.send_cmd(format!("/create {subject} '{}'", default_user.display_name))?;

        if !output.contains("activeUser") {
            let json = serde_json::from_str(&output).map_err(CallError::InvalidJson)?;
            return Err(InitError::DbError(json));
        }
    }

    let output = chat.send_cmd("/_start".to_owned())?;

    if !output.contains("chatStarted") {
        let json = serde_json::from_str(&output).map_err(CallError::InvalidJson)?;
        return Err(InitError::DbError(json));
    }

    Ok(chat)
}

fn chat_worker(mut chat: SimpleXChat, rx: ChatWorkerReceiver, ev_tx: EventTransmitter) {
    // TODO: Make it configurable
    const POLL_LATENCY: std::time::Duration = std::time::Duration::from_millis(250);

    let mut msg_wait_error = false;
    'outer: loop {
        loop {
            match chat.recv_msg_wait(POLL_LATENCY) {
                Ok(event) if event.is_empty() => break,
                Ok(event) => {
                    let _ = ev_tx.send(Ok(event));
                }
                Err(err) => {
                    let _ = ev_tx.send(Err(err));
                    msg_wait_error = true;
                    break 'outer;
                }
            }
        }

        loop {
            match rx.recv_timeout(POLL_LATENCY) {
                Ok(ChatWorkerCommand::Execute(command, responder)) => {
                    let output = chat.send_cmd(command);
                    let _ = responder.send(output);
                }
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) | Ok(ChatWorkerCommand::Disconnect) => {
                    break 'outer;
                }
            }
        }
    }

    loop {
        match rx.recv_timeout(POLL_LATENCY) {
            Ok(ChatWorkerCommand::Execute(_, responder)) => {
                let _ = responder.send(Err(CallError::Failure));
            }
            Ok(ChatWorkerCommand::Disconnect) => {}
            Err(_) => break,
        }
    }

    drop(rx);

    if !msg_wait_error {
        loop {
            match chat.recv_msg_wait(POLL_LATENCY) {
                Ok(event) if event.is_empty() => break,
                Ok(event) => {
                    if ev_tx.send(Ok(event)).is_err() {
                        break;
                    }
                }
                Err(err) => {
                    let _ = ev_tx.send(Err(err));
                    break;
                }
            }
        }
    }

    let _ = chat.send_cmd("/_stop".to_owned());
}

enum ChatWorkerCommand {
    Execute(Command, FfiResponder),
    Disconnect,
}
