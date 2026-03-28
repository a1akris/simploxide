//! Request sender task that batches and buffers requests upon a heavy load.
//!
use std::sync::Arc;

use crate::{Error, RequestId, Result, WsOut};
use futures::{SinkExt as _, StreamExt as _};
use serde::Serialize;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::tungstenite::{self, Message};

type InternalRequest = (RequestId, String);
type RequestSender = mpsc::UnboundedSender<InternalRequest>;
type RequestReceiver = mpsc::UnboundedReceiver<InternalRequest>;
type InterruptSignal = oneshot::Receiver<Option<Error>>;

pub type Interrupter = oneshot::Sender<Option<Error>>;

pub fn init(ws_out: WsOut, interrupt: InterruptSignal) -> Transmitter {
    let (sender, receiver) = mpsc::unbounded_channel::<InternalRequest>();
    tokio::spawn(transmission_task(ws_out, receiver, interrupt));

    Transmitter { sender }
}

#[derive(Clone)]
pub struct Transmitter {
    sender: RequestSender,
}

impl Transmitter {
    /// Returns an error if web socket is already disconnected
    pub fn make_request(&self, corr_id: RequestId, cmd: String) -> Result {
        self.sender
            .send((corr_id, cmd))
            .map_err(|_| Arc::new(tungstenite::Error::AlreadyClosed))
    }
}

async fn transmission_task(
    mut ws_out: WsOut,
    receiver: RequestReceiver,
    interrupt: InterruptSignal,
) {
    let mut request_stream = UnboundedReceiverStream::new(receiver);

    tokio::select! {
        result = try_send_all(&mut ws_out, &mut request_stream) => {
            match result {
                // We get into this branch only when all clients are dropped and nobody called a
                // `disconnect` explicitly.
                Ok(_) => {
                    log::debug!("All requests were sent successfully");
                }
                Err(e) => {
                    // Web socket is closed if we get an error here.
                    // Drop all buffered requests.
                    error_handler(&mut request_stream, &e).await;
                }
            }
        }
        e = interrupt => {
            if let Ok(Some(err)) = e {
                // This error can only be received from the dispatcher due to a ws_out failure so
                // the web socket is dead at this point. Dropping buffered requests.
                error_handler(&mut request_stream, &err).await;
            } else {
                // The graceful shutdown branch. Closing the stream to prevent new requests from
                // being buffered and trying to send everything that was buffered before the
                // disconnect call.
                request_stream.close();
                if let Err(err) = try_send_all(&mut ws_out, &mut request_stream).await {
                    // Dropping buffered requests in case of failure
                    error_handler(&mut request_stream, &err).await;
                }
            }
        }
    }

    log::debug!("Transmission task finished");
}

/// This redirects the entire stream of requests to the web socket.
///
/// Requests are flushed only when `requests.recv().poll()` returns `Poll::Pending` or when the
/// internal websocket buffer(which is 64MB by default) is full so, effectively, requests are
/// buffered and being transmitted in batches if there is a huge supply of them.
async fn try_send_all(
    ws_out: &mut WsOut,
    requests: &mut UnboundedReceiverStream<InternalRequest>,
) -> tungstenite::Result<()> {
    let mut message_stream = requests.map(|(id, req)| Ok(into_message(id, req)));
    ws_out.send_all(&mut message_stream).await
}

/// Closes the internal request receiver and drops all buffered requests with a warning
async fn error_handler(
    request_stream: &mut UnboundedReceiverStream<InternalRequest>,
    err: impl std::fmt::Display,
) {
    request_stream.close();

    while let Some((id, req)) = request_stream.next().await {
        log::warn!("Dropping request `({id}, {req})` due to error: {err}");
    }
}

fn into_message(id: RequestId, req: String) -> Message {
    Message::text(serde_json::to_string(&Request::new(id, req)).unwrap())
}

/// A little helper for JSON serialialization
#[derive(Serialize)]
struct Request {
    #[serde(rename = "corrId")]
    corr_id: String,
    cmd: String,
}

impl Request {
    fn new(id: RequestId, cmd: String) -> Self {
        Self {
            corr_id: id.to_string(),
            cmd,
        }
    }
}
