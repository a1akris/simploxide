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

#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "websocket")]
pub mod ws;

pub mod bot;
pub mod dispatcher;
pub mod ext;
pub mod id;
pub mod prelude;

pub use simploxide_api_types::{
    self as types,
    client_api::{self, BadResponseError, ClientApi, ClientApiError},
    commands, events,
    events::{Event, EventKind},
    responses,
    utils::CommandSyntax,
};

pub use dispatcher::{DispatchChain, LocalDispatchChain};

use futures::{Stream, TryStreamExt as _};

use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// The high level event stream that embeds event filtering.
///
/// Parsing SimpleX events may be costly, they are quite large deeply nested structs with a lot of
/// [`String`] and [`std::collections::BTreeMap`] types. This stream provides filtering APIs
/// allowing to parse and propagate events the application handles and drop all other events early
/// without allocating any extra memory.
///
/// By default filters are disabled and no events are dropped. Use [`Self::set_filter`] to only
/// receive events you're interested in.
///
/// Use [`Self::into_dispatcher`] and [`Self::into_local_dispatcher`] methods to handle events
/// conveniently. Dispatchers are completely zerocost, manage filters internally, and provide a
/// high-level easy to use APIs covering the absolute majority of use cases.
pub struct EventStream<P> {
    filter: [bool; EventKind::COUNT],
    receiver: tokio::sync::mpsc::UnboundedReceiver<P>,
}

impl<P> From<tokio::sync::mpsc::UnboundedReceiver<P>> for EventStream<P> {
    fn from(receiver: tokio::sync::mpsc::UnboundedReceiver<P>) -> Self {
        Self {
            filter: [true; EventKind::COUNT],
            receiver,
        }
    }
}

impl<P> EventStream<P> {
    pub fn set_filter<I: IntoIterator<Item = EventKind>>(mut self, f: Filter<I>) -> Self {
        match f {
            Filter::Accept(kinds) => {
                self.reject_all();
                for kind in kinds {
                    self.filter[kind.as_usize()] = true;
                }
            }
            Filter::AcceptAllExcept(kinds) => {
                self.accept_all();
                for kind in kinds {
                    self.filter[kind.as_usize()] = false;
                }
            }
            Filter::AcceptAll => self.accept_all(),
        }

        self
    }

    pub fn accept(&mut self, kind: EventKind) {
        self.filter[kind.as_usize()] = true;
    }

    pub fn reject(&mut self, kind: EventKind) {
        self.filter[kind.as_usize()] = false;
    }

    pub fn accept_all(&mut self) {
        self.set_all(true);
    }

    pub fn reject_all(&mut self) {
        self.set_all(false)
    }

    fn set_all(&mut self, new: bool) {
        for old in &mut self.filter {
            *old = new;
        }
    }
}

pub enum Filter<I: IntoIterator<Item = EventKind>> {
    Accept(I),
    AcceptAll,
    AcceptAllExcept(I),
}

impl<P: EventParser> EventStream<P> {
    /// Truns stream into [Dispatcher] builder with the provided `ctx`. The `ctx` is an
    /// arbitrary type that can be used within event handlers. See [Dispatcher::on] for details
    pub fn into_dispatcher<C: 'static + Send>(self, ctx: C) -> DispatchChain<P, C> {
        DispatchChain::with_ctx(self, ctx)
    }

    /// Truns stream into [LocalDispatcher] builder with the provided `ctx`. The `ctx` is an
    /// arbitrary type that can be used within event handlers. See [LocalDispatcher::on] for details
    pub fn into_local_dispatcher<C>(self, ctx: C) -> LocalDispatchChain<P, C> {
        LocalDispatchChain::with_ctx(self, ctx)
    }

    pub async fn stream_events<E, F>(mut self, mut f: F) -> Result<Self, E>
    where
        F: AsyncFnMut(Event) -> Result<StreamEvents, E>,
        E: From<P::Error>,
    {
        while let Some(event) = self.try_next().await? {
            if let StreamEvents::Break = f(event).await? {
                break;
            }
        }

        Ok(self)
    }

    pub async fn stream_events_with_ctx<E, Ctx, F>(
        mut self,
        mut f: F,
        ctx: Ctx,
    ) -> Result<(Self, Ctx), E>
    where
        F: AsyncFnMut(Event, &Ctx) -> Result<StreamEvents, E>,
        E: From<P::Error>,
    {
        while let Some(event) = self.try_next().await? {
            if let StreamEvents::Break = f(event, &ctx).await? {
                break;
            }
        }

        Ok((self, ctx))
    }

    pub async fn stream_events_with_ctx_mut<E, Ctx, F>(
        mut self,
        mut f: F,
        mut ctx: Ctx,
    ) -> Result<(Self, Ctx), E>
    where
        F: AsyncFnMut(Event, &mut Ctx) -> Result<StreamEvents, E>,
        E: From<P::Error>,
    {
        while let Some(event) = self.try_next().await? {
            if let StreamEvents::Break = f(event, &mut ctx).await? {
                break;
            }
        }

        Ok((self, ctx))
    }

    pub async fn stream_events_with_ctx_cloned<E, Ctx, F>(
        mut self,
        f: F,
        ctx: Ctx,
    ) -> Result<(Self, Ctx), E>
    where
        Ctx: Clone,
        F: AsyncFn(Event, Ctx) -> Result<StreamEvents, E>,
        E: From<P::Error>,
    {
        while let Some(event) = self.try_next().await? {
            if let StreamEvents::Break = f(event, ctx.clone()).await? {
                break;
            }
        }

        Ok((self, ctx))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamEvents {
    Break,
    Continue,
}

impl<P: EventParser> Stream for EventStream<P> {
    type Item = Result<Event, P::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.receiver.poll_recv(cx) {
                Poll::Ready(Some(raw_event)) => {
                    let kind = match raw_event.parse_kind() {
                        Ok(kind) => kind,
                        Err(e) => break Poll::Ready(Some(Err(e))),
                    };

                    if self.filter[kind.as_usize()] {
                        break Poll::Ready(Some(raw_event.parse_event()));
                    }
                }
                Poll::Ready(None) => break Poll::Ready(None),
                Poll::Pending => break Poll::Pending,
            }
        }
    }
}

/// A helper trait meant to be implemented by raw event types
pub trait EventParser {
    type Error;

    /// Should parse kind cheaply without allocations
    fn parse_kind(&self) -> Result<EventKind, Self::Error>;

    /// Parse the whole events
    fn parse_event(&self) -> Result<Event, Self::Error>;
}

impl EventParser for Event {
    type Error = std::convert::Infallible;

    fn parse_kind(&self) -> Result<EventKind, Self::Error> {
        Ok(self.kind())
    }

    fn parse_event(&self) -> Result<Event, Self::Error> {
        // Cheap Arc Clone
        Ok(self.clone())
    }
}

/// Syntactic sugar for preferences
///
/// use like:
/// ```
/// // ...
/// preferences: Some(Preferences {
///     timed_messages: preferences::timed_messages::yes(Duration::from_hours(4)),
///     full_delete: preferences::YES,
///     reactions: preferences::ALWAYS,
///     voice: preferences::NO,
///     files: preferences::ALWAYS,
///     calls: preferences::YES,
///     sessions: preferences::NO,
///     commands: None,
///     undocumented: Default::default(),
/// }),
/// // ...
/// ```
///
pub mod preferences {
    use simploxide_api_types::{FeatureAllowed, SimplePreference};

    pub mod timed_messages {
        use super::*;
        use simploxide_api_types::TimedMessagesPreference;

        pub const TTL_MAX: std::time::Duration = std::time::Duration::from_hours(8784);

        pub fn always(ttl: std::time::Duration) -> Option<TimedMessagesPreference> {
            let clamped = std::cmp::min(ttl, TTL_MAX);

            Some(TimedMessagesPreference {
                allow: FeatureAllowed::Always,
                ttl: Some(clamped.as_secs() as i32),
                undocumented: serde_json::Value::Null,
            })
        }

        pub fn yes(ttl: std::time::Duration) -> Option<TimedMessagesPreference> {
            let clamped = std::cmp::min(ttl, TTL_MAX);

            Some(TimedMessagesPreference {
                allow: FeatureAllowed::Yes,
                ttl: Some(clamped.as_secs() as i32),
                undocumented: serde_json::Value::Null,
            })
        }

        pub const NO: Option<TimedMessagesPreference> = Some(TimedMessagesPreference {
            allow: FeatureAllowed::No,
            ttl: None,
            undocumented: serde_json::Value::Null,
        });
    }

    pub const ALWAYS: Option<SimplePreference> = Some(SimplePreference {
        allow: FeatureAllowed::Always,
        undocumented: serde_json::Value::Null,
    });

    pub const YES: Option<SimplePreference> = Some(SimplePreference {
        allow: FeatureAllowed::Yes,
        undocumented: serde_json::Value::Null,
    });

    pub const NO: Option<SimplePreference> = Some(SimplePreference {
        allow: FeatureAllowed::No,
        undocumented: serde_json::Value::Null,
    });
}
