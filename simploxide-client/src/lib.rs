//! For first-time users it's recommended to get hands-on experience by running some example bots
//! on [GitHub](https://github.com/a1akris/simploxide/tree/main/simploxide-client) before writing
//! their own.
//!
//! # How to write a SimpleX bot?
//!
//! First of all, you **must** use a tokio runtime. The current `simploxide` implementation heavily
//! depends on it.
//!
//! It's also recommended to use `simploxide_client::prelude::*` everywhere to not pollute the
//! import section.
//!
//! ##### Now to the bot
//!
//! The most common bot structure will look like this:
//!
//! 1. Initialize a web socket connection with the simplex-chat daemon(you can run simplex-chat as
//!    a daemon using the `simplex-chat -p <port>` command)
//! 1. Prequery some info and do some validations required for your bot to work: this typically
//!    includes creating the bot address, switching to the right bot user, etc
//! 1. Start an event reactor loop and process the events.
//!
//! Example:
//!
//! ```ignore
//! use simploxide_client::prelude::*;
//! use futures::stream::TryStreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Init websocket connection with SimpleX daemon
//!     let (client, mut events) = simploxide_client::connect("ws://127.0.0.1:5225").await?;
//!
//!     // Pre-query and validate stuff
//!     client.do_some_initialization().await?;
//!
//!
//!     // Implement event reactor
//!     while let Some(ev) = events.try_next().await? {
//!         match ev {
//!             Event::SomeEvent1(SomeEvent1 { data }) => {
//!                 client.process_event1(data).await?;
//!             }
//!             Event::SomeEvent2(SomeEvent2 { data }) => {
//!                 client.process_event2(data).await?;
//!                 break;
//!             }
//!             _ => (), // Ignore events you're not interested in.
//!         }
//!     }
//!
//!
//!     // (Optional) some cleanup
//!
//!
//!     Ok(())
//!
//! }
//! ```
//!
//! Note that the reactor part in the example above is very inefficient because it reacts on events
//! sequentially - not processing any events until the client responds to the current event. This
//! can be OK if your bot doesn't need to operate under a heavy-load, such reactor could also be
//! useful during the development because it is trivial to debug, but for deployment it is
//! advisable to enable full asynchronous multi-threaded event processing which can be simply
//! achieved by moving event handlers into tokio tasks:
//!
//!
//!```ignore
//!     // Implement event reactor
//!     while let Some(ev) = events.try_next().await? {
//!         let client = client.clone();
//!         match ev {
//!             Event::SomeEvent1(SomeEvent1 { data }) => {
//!                 tokio::spawn(async move {
//!                     client.process_event1(data).await?;
//!                 });
//!             }
//!             Event::SomeEvent2(SomeEvent2 { data }) => {
//!                 tokio::spawn(async move {
//!                     client.process_event2(data).await?;
//!                     client.disconnect();
//!                 });
//!             }
//!             _ => (), // Ignore events you're not interested in.
//!         }
//!     }
//!```
//!
//! Now the event loop can't be terimnated with a `break` statetement because events are
//! processed asynchronously in their own tasks. You can call `client.disconnect()` in this case to
//! initiate a graceful shutdown which will eventually end the event stream, or you can use a
//! cancellation token + tokio::select! and break the loop when the token is triggered.
//!
//! ##### Trivial use-cases
//!
//! Some applications may not need to react to events, they can act like scripts or like remote
//! controllers for the SimpleX chat instance. In this case, drop the event stream immediately to
//! prevent events from buffering and leaking memory:
//!
//!
//! ```ignore
//!     // Init websocket connection with SimpleX daemon
//!     let (client, events) = simploxide_client::connect("ws://127.0.0.1:5225").await?;
//!     drop(events);
//! ```
//!
//!
//! ##### More complicated use-cases
//!
//! Some applications may have several event loops, so the reactor could be moved into a separate
//! async task. In this case it's recommended to save the handle of the tokio task and await it
//! before the program exits to prevent data losses.
//!
//! ```ignore
//!     // Init websocket connection with SimpleX daemon
//!     let (client, events) = simploxide_client::connect("ws://127.0.0.1:5225").await?;
//!     let handle = tokio::spawn(event_reactor(events));
//!
//!
//!     //..
//!
//!     handle.await
//! ```
//!
//!
//! ##### Graceful shutdown guarantees
//!
//! When calling `client.disconnect()` it's guaranteed that all futures created before this call
//! will still receive their responses and that all futures created after this call will resolve
//! with `tungstenite::Error::AlreadyClosed`.
//!
//! Note however, that if your task sends multiple requests and you're calling
//! `client.disconnect()` from another task then it's not guaranteed that your task will get all
//! responses. In fact any future can resolve with an error:
//!
//! ```ignore
//! async fn my_handler(client: simploxide_client::Client) -> HandlerResult {
//!     let res1 = client.req1().await?;
//!     // <--------------------------------- Disconnect triggers at this point
//!     let res2 = client.req2(res1).await?; // This future will throw an error
//!     Ok(res2)
//! }
//! ```
//!
//! You will need to implement additional synchronization mechanisms if you want to ensure that all
//! handlers run to completion when client disconnects.
//!
//! To understand more about the client implementation read the [`core`] docs.
//!
//! # How to work with this documentation?
//!
//! The [`Client`] page should become your main page and the [`events`] page should become your
//! secondary page. From these 2 pages you can reach all corners of the docs in a structured
//! manner.
//!
use futures::Stream;
use simploxide_api_types::{
    JsonObject,
    client_api::ClientApi,
    client_api::{BadResponseError, ClientApiError},
    events::Event,
};
use simploxide_core::{EventQueue, EventReceiver, RawClient};
use std::{sync::Arc, task};

pub use simploxide_api_types::{
    self as types, client_api, commands, events, responses, utils::CommandSyntax,
};
pub use simploxide_core::{
    self as core, Error as CoreError, Result as CoreResult, tungstenite::Error as WsError,
};

pub mod prelude;

pub type ClientResult<T = ()> = std::result::Result<T, ClientError>;

/// A wrapper over [`simploxide_core::connect`] that turns [`simploxide_core::RawClient`] into
/// [`Client`] and raw event queue into the [`EventStream`] with automatic event deserialization.
///
/// ```ignore
/// let (client, mut events) = simploxide_client::connect("ws://127.0.0.1:5225").await?;
///
/// let current_user  = client.api_show_active_user().await?;
/// println!("{current_user:#?}");
///
/// while let Some(ev) = events.try_next().await? {
///     // Process events...
/// }
/// ```
pub async fn connect<S: AsRef<str>>(uri: S) -> Result<(Client, EventStream), WsError> {
    let (raw_client, raw_event_queue) = simploxide_core::connect(uri.as_ref()).await?;
    Ok((Client::from(raw_client), EventStream::from(raw_event_queue)))
}

pub struct EventStream(EventReceiver);

impl From<EventQueue> for EventStream {
    fn from(value: EventQueue) -> Self {
        Self(value.into_receiver())
    }
}

impl Stream for EventStream {
    type Item = CoreResult<Arc<Event>>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        self.0.poll_recv(cx).map(|opt| {
            opt.map(|res| res.map(|ev| serde_json::from_value::<Arc<Event>>(ev).unwrap()))
        })
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
    /// [`simploxide_core::RawClient::disconnect`] for details.
    pub fn disconnect(self) {
        self.inner.disconnect();
    }
}

impl ClientApi for Client {
    type Error = ClientError;

    async fn send_raw(&self, command: String) -> Result<JsonObject, Self::Error> {
        self.inner
            .send(command)
            .await
            .map_err(ClientError::WebSocketFailure)
    }
}

/// See [`core::client_api::AllowUndocumentedResponses`] if you don't want to trigger an error when
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

impl std::error::Error for ClientError {}

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
    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError> {
        if let Self::BadResponse(resp) = self {
            Some(resp)
        } else {
            None
        }
    }
}
