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
//! # Using the FFI interface
//!
//! # How to work with this documentation?
//!
//! The [`Client`] page should become your main page and the [`events`] page should become your
//! secondary page. From these 2 pages you can reach all corners of the docs in a structured
//! manner.
//!
//!

pub mod prelude;

#[cfg(feature = "websocket")]
pub mod ws;

#[cfg(feature = "cli")]
pub use simploxide_ws_core::cli;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use simploxide_api_types::{
    self as types,
    client_api::{self, BadResponseError, ClientApi, ClientApiError},
    commands, events, responses,
    utils::CommandSyntax,
};
