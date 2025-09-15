//! # How to write a SimpleX bot?
//!
//! First of all, you **must** use a tokio runtime. The current `simploxide` implementation heavily
//! depends on it.
//!
//! Secondly, it's recommended to use `simploxide_client::prelude::*` if you don't want your import
//! section to explode. The prelude reexports all top-level types required for sending requests,
//! destructuring responses and matching events, you'll still need to manually import intermediary
//! types and there are a lot of them, the prelude just greately reduces the amount of the same
//! imports per file.
//!
//! ##### Now to the bot
//!
//! The most common bot structure will look like this:
//!
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
//! 1. Initialize a web socket connection with the simplex-chat daemon. You can run simplex-chat as
//!    a daemon with `simplex-chat -p <port>` command.
//! 1. Prequery some info and do some validations required for your bot to work: this typically
//!    includes getting or creating the bot address, switching to the right bot user, etc
//! 1. Start an event reactor loop and process the events.
//!
//! Everything looks simple and trivial but the reactor part in the example above is terribly
//! inefficient. It reacts on events sequentially waiting for client to respond to the first event
//! before processing the second. This can be fine if your bot doesn't need to operate under a
//! heavy-load, such reactor would also be useful during the development because it is trivial to
//! debug however, for production it's advisable to enable full asynchronous multi-threaded
//! processing that can be achieved by simply moving the event handlers into tokio tasks:
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
//! Note, that we can't terminate the event loop with a `break` statetement because the event is
//! being processed asynchronously in its own task. You can call `client.disconnect()` in this case
//! to initiate a graceful shutdown which will eventually end the event stream, but even with
//! strong guarantees the graceful shutdown provides it cannot guarantee that events, which
//! occurred before the shutdown, will be processed to completion as tasks may need to send several
//! requests to complete successfully, so if this is important for you application to process
//! events atomically you should use primitives like tokio channels and notifies to break the loop
//! without dropping the web socket connection.
//!
//!
//! ##### A simpler use case
//!
//! Some applications may not need to react on events, they can act like scripts, or like remote
//! controllers for a SimpleX chat instance. In this case, drop the event stream immediately to
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
//! ##### More complicated use case
//!
//! Some applications may have several event loops, so the reactor could be moved into a separate
//! async task. In this case it's recommended to save the handle of the tokio task and await it
//! before the program exits to prevent data losses(e.g. to ensure that client.disconnect() is called).
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
//! You can find complete examples that apply these concepts on
//! [GitHub](https://github.com/a1akris/simploxide/tree/main/simploxide-client)
//!
//! # How to work with this documentation?
//!
//! The [`Client`] page should become your main page. From there you can reach the deepest corners
//! of the docs in a structured manner. Looking at other modules is not very helpful unless you're
//! looking for something specific.
//!
//! If you need to understand how async is being implemented in the client check out the [`core`]
//! docs.
//!
use futures::{Stream, TryStreamExt as _};
use simploxide_api_types::{JsonObject, events::Event};
use simploxide_core::RawClient;
use std::sync::Arc;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub use simploxide_api_types::{
    self as types, client_api::ClientApi, commands, events, responses, utils::CommandSyntax,
};
pub use simploxide_core::{
    self as core, Error as CoreError, Result as CoreResult, tungstenite::Error as WsError,
};

pub mod prelude;

/// A wrapper over [`simploxide_core::connect`] that turns [`simploxide_core::RawClient`] into
/// [`Client`] and the event queue into the event stream with automatic event
/// deserialization.
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
pub async fn connect<S: AsRef<str>>(
    uri: S,
) -> Result<
    (
        Client,
        impl Stream<Item = Result<Arc<Event>, CoreError>> + Unpin,
    ),
    WsError,
> {
    let (inner, raw_queue) = simploxide_core::connect(uri.as_ref()).await?;
    let stream = UnboundedReceiverStream::new(raw_queue.into_receiver());

    Ok((
        Client { inner },
        stream.map_ok(|ev| serde_json::from_value::<Arc<Event>>(ev).unwrap()),
    ))
}

/// A high level simplex client that implements [`ClientApi`] which provides typed client
/// methods with automatic command serialization/response deserialization.
pub struct Client {
    inner: RawClient,
}

impl Client {
    /// Initiates a graceful shutdown for the underlying web socket connection. See
    /// [`simploxide_core::RawClient::disconnect`] for details.
    pub fn disconnect(self) {
        self.inner.disconnect();
    }
}

impl ClientApi for Client {
    type Error = CoreError;

    async fn send_raw(&self, command: String) -> Result<JsonObject, Self::Error> {
        self.inner.send(command).await
    }
}
