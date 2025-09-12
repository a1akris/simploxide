//! A fully asynchronous raw SimpleX client which provides:
//!
//! 1. Requests batching under a heavy load.
//!
//! 2. Complete asynchonisity: futures created by the same instance of a client are fully
//!    independent from each other. The event queue receives events independently from client
//!    actions.
//!
//! 3. Graceful shutdown with strong guarantees:
//!     - All futures scheduled before the `.disconnect` call are guaranteed to receive their
//!     responses. All futures scheduled after the `.disconnect` call are guaranteed to receive the
//!     [`tungstenite::Error::AlreadyClosed`] error.
//!
//!     - If the web socket connection drops due to an error all already received(buffered)
//!     responses are guaranteed to be delivered to corresponding futures. All other pending
//!     futures are guaranteed to be resolved with this web socket error.
//!
//!     - You will receive events for as long as there are futures awaiting responses. After all
//!     futures are resolved you will receive all buffered events and then the event queue will be
//!     closed.
//!
//! See [the Github link](#link) for diagrams demonstrating how all this works under the hood.
//!
//! -----
//!
//! _Current implementation heavily depends on `tokio` runtime and won't work with other
//! executors._
mod dispatcher;
mod router;
mod transmission;

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use futures::StreamExt;
use tokio::sync::oneshot;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, client::IntoClientRequest as _},
};
use tokio_util::sync::CancellationToken;

use {router::ClientRouter, transmission::Transmitter};

pub use dispatcher::EventQueue;
pub use tokio_tungstenite::{self, tungstenite};

pub type Event = serde_json::Value;
pub type Response = Event;
pub type Error = Arc<tungstenite::Error>;
pub type Result<T = ()> = ::std::result::Result<T, Error>;
pub type RawEventQueue = EventQueue;

type WsOut =
    futures::stream::SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WsIn = futures::stream::SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

static REQUEST_ID: AtomicUsize = AtomicUsize::new(0);

type RequestId = usize;
fn next_request_id() -> RequestId {
    REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}

/// Connect to the running SimpleX daemon by websocket URI str. Note that SimpleX doesn't support
/// TLS so using "wss://" will produce an error.
///
/// Returns a client that should be used to send requests and receive responses and the event queue
/// that buffers SimpleX chat events.
///
/// If you're writing a script-like app that doesn't need to process events drop the returned event
/// queue immediately to prevent the events being buffered effectively causing a memory leak.
///
/// Example:
/// ```ignore
/// let (client, events) = simploxide_core::connect("ws://127.0.0.1:5225").await?;
///
/// // (Optional) Drop the event queue if you're not planning to handle events
/// drop(events)
///
/// let current_user  = client.send("/user".to_owned()).await?;
/// println!("{}", serde_json::to_string_pretty(&current_user).unwrap());
/// ```
pub async fn connect(simplex_daemon_url: &str) -> tungstenite::Result<(RawClient, RawEventQueue)> {
    let connection_request = simplex_daemon_url.into_client_request()?;
    let (sockstream, _) = connect_async(connection_request).await?;
    let (ws_out, ws_in) = sockstream.split();

    let dispatching_cancellator = CancellationToken::new();
    let (transmission_interrupter, transmission_interrupted) = oneshot::channel();

    let (client_router, response_router) =
        router::init(dispatching_cancellator.clone(), transmission_interrupter);
    let tx = transmission::init(ws_out, transmission_interrupted);
    let event_queue = dispatcher::init(ws_in, response_router, dispatching_cancellator);

    Ok((
        RawClient {
            tx,
            router: client_router,
        },
        event_queue,
    ))
}

/// A lightweight cheaply clonable client capable of sending raw requests(SimpleX commands) and
/// getting raw responses(JSON objects).
///
/// You can use the client behind a shared reference, or you can clone it, in both cases the
/// created futures will be indpenendent from each other.
#[derive(Clone)]
pub struct RawClient {
    tx: Transmitter,
    router: ClientRouter,
}

impl RawClient {
    /// Send a raw SimpleX request that is a SimpleX CLI command.
    ///
    /// The actual request sending part always resolves immediately so the `send(..).await` call
    /// directly awaits the response.
    pub async fn send(&self, command: String) -> Result<Response> {
        let id = next_request_id();
        let (responder, response) = oneshot::channel();

        // IMPORTANT: It's crucial to book a request before sending it to the server to avoid the
        // case when the response comes before the responder registration.
        self.router.book(id, responder)?;
        self.tx.make_request(id, command)?;

        response
            .await
            .expect("Registered responders always deliver")
    }

    /// Drops the current client and initiates a graceful shutdown for all its copies and the
    /// events dispatcher task.
    pub fn disconnect(self) {
        self.router.shutdown();
    }
}
