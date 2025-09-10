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

#[derive(Clone)]
pub struct RawClient {
    tx: Transmitter,
    router: ClientRouter,
}

impl RawClient {
    /// Send a raw SimpleX request which is a SimpleX CLI command.
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

    pub fn disconnect(self) {
        self.router.shutdown();
    }
}
