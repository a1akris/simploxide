//! Remote controller support.
//!
//! Allows a bot to accept an incoming remote control session from a SimpleX Desktop client,
//! giving the desktop live access to the bot's SimpleX instance.
//!
//! # Usage
//!
//! ```ignore
//! let (rc, events) = events.hook_remote_ctrl();
//!
//! tokio::spawn(async move { bot.run(events).await });
//!
//! // connection link received from admin via DM
//! bot.accept_remote_ctrl(&rc, &connection_link).await?;
//! ```

use simploxide_api_types::{
    JsonObject,
    client_api::{BadResponseError, ClientApi, ExtractResponse as _},
    events::{Event, EventKind},
};
use tokio::sync::oneshot;

use std::sync::Mutex;

use crate::{Hook, util::TypeField};

type Slot = Mutex<Option<oneshot::Sender<Result<String, JsonObject>>>>;

/// Can be obtained via [`EventStream::hook_remote_ctrl`](crate::EventStream::hook_remote_ctrl) or
/// [`RemoteCtrlHook::handle`]
pub struct CtrlHandle {
    slot: Slot,
}

impl CtrlHandle {
    pub(crate) fn new() -> Self {
        Self {
            slot: Mutex::new(None),
        }
    }

    /// Connect to a remote controller using the connection link obtained via SimpleX-Desktop ->
    /// link mobile device menu.
    ///
    /// # Deadlock warning
    ///
    /// This method awaits a `remoteCtrlSessionCode` event that only arrives when the event loop is
    /// running concurrently. Calling this method from a sequential handler blocks the event loop
    /// and deadlocks. Only call from a concurrent handler or outside the event dispatching logic
    /// entirely.
    pub async fn accept_remote_ctrl<C: ClientApi>(
        &self,
        client: &C,
        link: &str,
    ) -> Result<(), CtrlError<C::Error>> {
        let (tx, rx) = oneshot::channel();
        *self.slot.lock().unwrap() = Some(tx);

        let res = try_accept(client, link, rx).await;

        if res.is_err() {
            self.slot.lock().unwrap().take();
        }

        res
    }

    fn emit(&self, value: Result<String, JsonObject>) {
        if let Ok(mut slot) = self.slot.lock()
            && let Some(sender) = slot.take()
        {
            let _ = sender.send(value);
        }
    }
}

impl Hook for CtrlHandle {
    fn should_intercept(&self, kind: EventKind) -> bool {
        kind == EventKind::Undocumented
    }

    fn intercept_event(&self, event: Event) {
        // println!("CRC hook got event: {event:#?}");

        let Event::Undocumented(ref obj) = event else {
            return;
        };

        match obj.get("type").and_then(|v| v.as_str()) {
            Some("remoteCtrlSessionCode") => {
                match obj.get("sessionCode").and_then(|v| v.as_str()) {
                    Some(code) => self.emit(Ok(code.to_owned())),
                    None => self.emit(Err(obj.clone())),
                }
            }
            Some("remoteCtrlStopped") => {
                self.emit(Err(obj.clone()));
            }
            _ => (),
        }
    }
}

async fn try_accept<C: ClientApi>(
    client: &C,
    link: &str,
    rx: oneshot::Receiver<Result<String, JsonObject>>,
) -> Result<(), CtrlError<C::Error>> {
    let raw = client
        .send_raw(format!("/crc {link}"))
        .await
        .map_err(CtrlError::Api)?;

    check_tag::<C>(&raw, "remoteCtrlConnecting")?;

    let code = rx
        .await
        .map_err(|_| CtrlError::EventStreamClosed)?
        .map_err(|obj| CtrlError::BadResponse(BadResponseError::Undocumented(obj)))?;

    let raw = client
        .send_raw(format!("/verify remote ctrl {code}"))
        .await
        .map_err(CtrlError::Api)?;

    check_tag::<C>(&raw, "remoteCtrlConnected")?;
    Ok(())
}

fn check_tag<C: ClientApi>(raw: &str, expected: &str) -> Result<(), CtrlError<C::Error>> {
    let shape: C::ResponseShape<'_, TypeField<'_>> = serde_json::from_str(raw)
        .map_err(|e| CtrlError::BadResponse(BadResponseError::InvalidJson(e)))?;

    let tag = shape.extract_response().map_err(CtrlError::BadResponse)?;

    if tag.typ != expected {
        return Err(CtrlError::BadResponse(BadResponseError::Undocumented(
            serde_json::json!({"expected": expected, "got": tag.typ}),
        )));
    }
    Ok(())
}

#[derive(Debug)]
pub enum CtrlError<E> {
    /// The API call failed.
    Api(E),
    /// The event stream was closed before the session code event arrived.
    EventStreamClosed,
    /// The chat returned an unexpected or error response.
    BadResponse(BadResponseError),
}

impl<E: std::fmt::Display> std::fmt::Display for CtrlError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api(e) => write!(f, "{e}"),
            Self::EventStreamClosed => {
                write!(f, "event stream closed before session code arrived")
            }
            Self::BadResponse(e) => write!(f, "unexpected remote control response: {e}"),
        }
    }
}

impl<E: 'static + std::error::Error> std::error::Error for CtrlError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Api(e) => Some(e),
            Self::EventStreamClosed => None,
            Self::BadResponse(e) => Some(e),
        }
    }
}
