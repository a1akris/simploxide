//! A fully asynchronous raw SimpleX websocket client that provides:
//!
//! 1. Requests batching under a heavy load.
//!
//! 2. Complete asynchonisity: futures created by the same instance of a client are fully
//!    independent from each other. The event queue receives events independently from client
//!    actions.
//!
//! 3. Graceful shutdown with strong guarantees:
//!     - All futures scheduled before the `.disconnect` call are guaranteed to receive their
//!       responses. All futures scheduled after the `.disconnect` call are guaranteed to receive the
//!       [`tungstenite::Error::AlreadyClosed`] error.
//!
//!     - If the web socket connection drops due to an error all buffered responses are guaranteed
//!       to be delivered to corresponding futures. All other pending futures are guaranteed to be
//!       resolved with the web socket error.
//!
//!     - You will receive events for as long as there are futures awaiting responses. After all
//!       futures are resolved you will receive all buffered events and then the event queue will be
//!       closed.
//!
//! See [README on GitHub](https://github.com/a1akris/simploxide/tree/main/simploxide-core) for diagrams
//! demonstrating how all this works under the hood.
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
use tokio::sync::{oneshot, watch};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, client::IntoClientRequest as _},
};
use tokio_util::sync::CancellationToken;

use {router::ClientRouter, transmission::Transmitter};

pub use dispatcher::{EventQueue, EventReceiver};
pub use tokio_tungstenite::{self, tungstenite};

pub type Event = String;
pub type Response = Event;
pub type Error = Arc<tungstenite::Error>;
pub type Result<T = ()> = ::std::result::Result<T, Error>;
pub type RawEventQueue = EventQueue;

type WsOut =
    futures::stream::SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WsIn = futures::stream::SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

type ShutdownEmitter = watch::Sender<bool>;
type ShutdownSignal = watch::Receiver<bool>;

static REQUEST_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(feature = "cli")]
pub mod cli;

type RequestId = usize;
fn next_request_id() -> RequestId {
    REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}

/// Connect to the running SimpleX daemon by websocket URI.
///
/// Returns a [RawClient] for sending commands and a [RawEventQueue] that buffers incoming chat
/// events independently of client activity.
///
/// # Security
///
/// - SimpleX CLI does not support TLS URIs("wss://") and will fail at the handshake. The web
///   socket carries unencrypted unauthenticated traffic. Bind the daemon to
///   localhost(`ws://127.0.0.1:{port}`) only. Any process or host that can reach the port has full,
///   unauthenticated control over the daemon, can intercept events and execute arbitrary commands.
///
/// # Memory
///
/// The [`RawEventQueue`] is backed by an unbounded channel. If events are not consumed they
/// accumulate indefinitely. Either process events promptly or drop the queue immediately if your
/// application does not need them
///
/// # Example
///
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
    let (shutdown_tx, shutdown) = watch::channel(false);

    let (client_router, response_router) = router::init(
        dispatching_cancellator.clone(),
        transmission_interrupter,
        shutdown_tx,
    );
    let tx = transmission::init(ws_out, transmission_interrupted);
    let event_queue = dispatcher::init(ws_in, response_router, dispatching_cancellator);

    Ok((
        RawClient {
            tx,
            router: client_router,
            shutdown,
        },
        event_queue,
    ))
}

/// A lightweight cheaply clonable client capable of sending raw requests(SimpleX commands) and
/// receiving raw responses(JSON objects).
///
/// You can use the client behind a shared reference, or you can clone it, in both cases the
/// created futures will be indpenendent from each other.
#[derive(Clone)]
pub struct RawClient {
    tx: Transmitter,
    router: ClientRouter,
    shutdown: ShutdownSignal,
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

    /// Initiates a graceful shutdown and waits until it is complete. Returns only after the
    /// connection is fully closed.
    ///
    /// All futures that got scheduled before this call will still receive their responses. All
    /// futures scheduled after this call(from cloned clients) will resolve immediately with
    /// [`tungstenite::Error::AlreadyClosed`].
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
    /// If [`Self::send`] and [`Self::disconnect`] are called concurrently from different threads
    /// the outcome depends on scheduling. If `send` wins the channel lock first, it will receive a
    /// response as normal. If `disconnect` wins first, the `send` future will receive
    /// [`tungstenite::Error::AlreadyClosed`].
    ///
    /// However, in the second case the request could have already been buffered and delivered to the
    /// server by another thread while `disconnect` was executing on the current thread, meaning the
    /// send command ran even though the client received an error. Do not use `AlreadyClosed` as a
    /// proof that the command was not executed. To guarantee ordering, await all `send` futures to
    /// completion before calling `disconnect`.
    pub fn disconnect(mut self) -> impl Future<Output = ()> {
        self.router.shutdown();
        async move {
            let _ = self.shutdown.wait_for(|done| *done).await;
        }
    }
}
