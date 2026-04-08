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

use futures::Stream;
use simploxide_api_types::events::EventData;
#[cfg(feature = "cli")]
pub use simploxide_ws_core::cli;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use simploxide_api_types::{
    self as types,
    client_api::{self, BadResponseError, ClientApi, ClientApiError},
    commands, events,
    events::{Event, EventKind},
    responses,
    utils::CommandSyntax,
};

use futures::TryStreamExt as _;
use std::{
    pin::Pin,
    sync::Arc,
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

impl<P: EventParser> EventStream<P> {
    /// Truns stream into [Dispatcher] builder with the provided `ctx`. The `ctx` is an
    /// arbitrary type that can be used within event handlers. See [Dispatcher::on] for details
    pub fn into_dispatcher<C: 'static + Send>(self, ctx: C) -> DispatchChain<P, C> {
        DispatchChain { events: self, ctx }
    }

    /// Truns stream into [LocalDispatcher] builder with the provided `ctx`. The `ctx` is an
    /// arbitrary type that can be used within event handlers. See [LocalDispatcher::on] for details
    pub fn into_local_dispatcher<C>(self, ctx: C) -> LocalDispatchChain<P, C> {
        LocalDispatchChain { events: self, ctx }
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

pub enum Filter<I: IntoIterator<Item = EventKind>> {
    Accept(I),
    AcceptAll,
    AcceptAllExcept(I),
}

/// A helper trait meant to be implemented by raw event types
pub trait EventParser {
    type Error;

    /// Should parse kind cheaply without allocations
    fn parse_kind(&self) -> Result<EventKind, Self::Error>;

    /// Parse the whole events
    fn parse_event(&self) -> Result<Event, Self::Error>;
}

/// [LocalDispatcher] builder
pub struct LocalDispatchChain<P, Ctx> {
    events: EventStream<P>,
    ctx: Ctx,
}

impl<P, Ctx> LocalDispatchChain<P, Ctx>
where
    P: EventParser,
{
    pub fn with_ctx(events: EventStream<P>, ctx: Ctx) -> Self {
        Self { ctx, events }
    }

    /// Same as [LocalDispatcher::on] but accepts [events::Event] as the first argument.
    ///
    /// Allows to deal with unhandled events. This is mostly useful for debug logging. It's better
    /// to remove this method in production to avoid full parsing of unimportant events.
    pub fn fallback<E, F>(mut self, f: F) -> LocalDispatcher<P, Ctx, Fallback<F>>
    where
        F: AsyncFnMut(Event, &mut Ctx) -> Result<StreamEvents, E>,
    {
        self.events.accept_all();

        LocalDispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Fallback { f },
        }
    }

    /// See [LocalDispatcher::on]
    pub fn on<Ev, E, F>(mut self, f: F) -> LocalDispatcher<P, Ctx, Match<Ev, F>>
    where
        Ev: EventData,
        F: AsyncFnMut(Arc<Ev>, &mut Ctx) -> Result<StreamEvents, E>,
    {
        self.events.reject_all();
        self.events.accept(Ev::KIND);

        LocalDispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Match {
                f,
                _phantom: std::marker::PhantomData,
            },
        }
    }
}

/// This type builds _mostly_ compile-time known dispatch chain in its parameter `D`. The chain is
/// then unrolled in [`Self::dispatch`] method
///
/// _Knowing the full dispatch chain at compile-time requires stabilization of the
/// `async_fn_traits` feature, in partcular, the implementation must be able to refer to
/// [AsyncFnMut::CallRefFuture]. Right now all handlers are pinboxed to resolve the future type._
pub struct LocalDispatcher<P, Ctx, D> {
    events: EventStream<P>,
    ctx: Ctx,
    chain: D,
}

impl<P, Ctx, D> LocalDispatcher<P, Ctx, D>
where
    D: DispatchEvent<Ctx>,
{
    /// Register an event handler
    ///
    /// - The handler signature is `AsyncFnMut(ev: Arc<{EventDataType}>, ctx: &mut Ctx) -> Result<StreamEvents, {ErrorType}>;`
    /// - `Ctx` is whatever type you pass when creating the dispatcher with
    ///   `events.into_local_dispatcher(ctx)`. It will be passed by a mutable reference into all handlers
    /// - `{EventDataType}` is one of the structs defined in [events]. Events are dispatched
    ///   statically so this type links the handler to event. When handlers with the same `{EventDataType}` are set
    ///   multiple times the latest set call overrides previous ones.
    /// - `{ErrorType}` can be arbitrary but all handlers must share it and it must implement `From<ClientError>`.
    ///
    /// ## Usage
    ///
    /// - Set async fn
    ///
    /// ```rust
    /// events.into_local_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on(contact_connected)
    ///     .dispatch()
    ///     .await;
    ///
    /// async fn contact_connected(
    ///     ev: Arc<ContactConnected>,
    ///     ctx: &mut ws::Client
    /// ) -> ClientResult<StreamEvents> {
    ///     // ...
    /// }
    /// ```
    ///
    /// - Set async closure by fully qualifying types
    ///
    /// ```rust
    /// events.into_local_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on::<ContactConnected, _>(async |ev, &mut client| {
    ///         //...
    ///     })
    ///     .dispatch()
    ///     .await;
    /// ```
    ///
    /// - Set async closure by specifying closure argument type
    ///
    /// ```rust
    /// events.into_local_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on(async |ev: Arc<ContactConnected>, &mut client| {
    ///         //...
    ///     })
    ///     .dispatch()
    ///     .await;
    /// ```
    pub fn on<Ev, F>(mut self, f: F) -> LocalDispatcher<P, Ctx, Intercept<Match<Ev, F>, D>>
    where
        Ev: EventData,
        F: AsyncFnMut(Arc<Ev>, &mut Ctx) -> Result<StreamEvents, D::Error>,
    {
        self.events.accept(Ev::KIND);
        LocalDispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Intercept {
                d1: Match {
                    f,
                    _phantom: std::marker::PhantomData,
                },
                d2: self.chain,
            },
        }
    }

    /// Dispatch events sequentially. Handlers block the event stream allowing them to exclusively
    /// mutate `Ctx` by the `&mut` reference. Returning [StreamEvents::Break] stops the dispatcher
    /// and returns the event stream and the `ctx` for further processing. The returned
    /// [EventStream]'s filters are reset([EventStream::accept_all]).
    ///
    /// This method returns `Future: !Send`, it should be used either with the [`tokio::task::LocalSet`],
    /// on the tokio main thread, or with a single-threaded runtime.
    pub async fn dispatch(self) -> Result<(EventStream<P>, Ctx), D::Error>
    where
        P: EventParser,
        D::Error: From<P::Error>,
    {
        let Self {
            ctx,
            events,
            mut chain,
        } = self;

        let (mut stream, ctx) = events.stream_events_with_ctx_mut(async move |ev, ctx| {
            let Ok(handler) = chain.dispatch_event(ev, ctx) else {
                unreachable!("EventStream filters set by on/fallback methods drop events without handlers during parsing");
            };

            handler.await

        }, ctx).await?;

        stream.accept_all();
        Ok((stream, ctx))
    }
}

/// Dispatcher builder
pub struct DispatchChain<P, Ctx> {
    events: EventStream<P>,
    ctx: Ctx,
}

impl<P, Ctx> DispatchChain<P, Ctx>
where
    P: EventParser,
{
    pub fn with_ctx(events: EventStream<P>, ctx: Ctx) -> Self {
        Self { ctx, events }
    }

    /// Same as [Dispatcher::on] but accepts [events::Event] as the first argument.
    ///
    /// Allows to deal with unhandled events. This is mostly useful for debug logging. It's better
    /// to remove this method in production to avoid full parsing of unimportant events.
    pub fn fallback<E, F, Fut>(mut self, f: F) -> Dispatcher<P, Ctx, Fallback<F>>
    where
        Ctx: 'static + Send,
        E: 'static + Send + From<P::Error>,
        F: Fn(Event, Ctx) -> Fut,
        Fut: 'static + Send + std::future::Future<Output = Result<StreamEvents, E>>,
    {
        self.events.accept_all();

        Dispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Fallback { f },
        }
    }

    /// See [Dispatcher::on]
    pub fn on<Ev, E, F, Fut>(mut self, f: F) -> Dispatcher<P, Ctx, Match<Ev, F>>
    where
        Ctx: 'static + Send,
        E: 'static + Send,
        Ev: 'static + EventData,
        F: Fn(Arc<Ev>, Ctx) -> Fut,
        Fut: 'static + Send + std::future::Future<Output = Result<StreamEvents, E>>,
    {
        self.events.reject_all();
        self.events.accept(Ev::KIND);

        Dispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Match {
                f,
                _phantom: std::marker::PhantomData,
            },
        }
    }
}

/// This type builds fully compile-time known dispatch chain in its parameter `D`. The chain is
/// then unrolled in [`Self::dispatch`] and [`Self::sequential_dispatch`] methods
pub struct Dispatcher<P, Ctx, D> {
    events: EventStream<P>,
    ctx: Ctx,
    chain: D,
}

impl<P, Ctx, D> Dispatcher<P, Ctx, D>
where
    P: 'static + EventParser,
    Ctx: 'static + Send + Clone,
    D: ConcurrentDispatchEvent<Ctx>,
    D::Error: From<P::Error>,
{
    /// Register an event handler
    ///
    /// - The handler signature is `AsyncFn(ev: Arc<{EventDataType}>, ctx: Ctx) -> Result<StreamEvents, {ErrorType}>;`
    /// - `Ctx` is whatever type you pass when creating the dispatcher with `events.into_dispatcher(ctx)`. It will be cloned into all handlers
    /// - `{EventDataType}` is one of the structs defined in [events]. Events are dispatched
    ///   statically so this type links the handler to event. When handlers with the same `{EventDataType}` are set
    ///   multiple times the latest set call overrides previous ones.
    /// - `{ErrorType}` can be arbitrary but all handlers must share it and it must implement `From<ClientError>`.
    ///
    /// ## Usage
    ///
    /// - Set async fn
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on(contact_connected)
    ///     .dispatch()
    ///     .await;
    ///
    /// async fn contact_connected(
    ///     ev: Arc<ContactConnected>,
    ///     ctx: ws::Client
    /// ) -> ClientResult<StreamEvents> {
    ///     // ...
    /// }
    /// ```
    ///
    /// - Set async closure by fully qualifying types
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on::<ContactConnected, _, _>(async |ev, client| {
    ///         //...
    ///     })
    ///     .dispatch()
    ///     .await;
    /// ```
    ///
    /// - Set async closure by specifying closure argument type
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on(async |ev: Arc<ContactConnected>, client| {
    ///         //...
    ///     })
    ///     .dispatch()
    ///     .await;
    /// ```
    pub fn on<Ev, F, Fut>(mut self, f: F) -> Dispatcher<P, Ctx, Intercept<Match<Ev, F>, D>>
    where
        Ev: 'static + EventData,
        F: Fn(Arc<Ev>, Ctx) -> Fut,
        Fut: 'static + Send + std::future::Future<Output = Result<StreamEvents, D::Error>>,
    {
        self.events.accept(Ev::KIND);
        Dispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Intercept {
                d1: Match {
                    f,
                    _phantom: std::marker::PhantomData,
                },
                d2: self.chain,
            },
        }
    }

    /// *WARNING:* This is not a concurrent dispatch!
    ///
    /// This method is the equivalent of [LocalDispatcher::dispatch] as it executes handlers
    /// sequentially one by one, the difference is it produces a `'static + Send` future. The
    /// method exists to cover niche use cases when handler execution order matters but dispatching
    /// happens as part of another `'static + Send` Future(e.g. within a tokio::task).
    pub async fn sequential_dispatch(self) -> Result<(EventStream<P>, Ctx), D::Error> {
        let ctx = self.ctx;
        let events = self.events;
        let chain = self.chain;

        let (mut stream, ctx) = events.stream_events_with_ctx_cloned(async move |ev, ctx| {
            let Ok(handler) = chain.concurrent_dispatch_event(ev, ctx) else {
                unreachable!("EventStream filtering set by on/fallback methods drops events without handlers before parsing them");
            };

            handler.await

        }, ctx).await?;

        stream.accept_all();
        Ok((stream, ctx))
    }

    /// Spawns handlers as tokio tasks. Handlers will execute and resolve in arbitrary order.
    /// [StreamEvents::Break] eventually stops the dispatcher but it doesn't stop spawning new
    /// event handlers until it is observed so assumptions about the resulting Ctx state must not
    /// rely on [StreamEvents::Break]. The returned [EventStream]'s filters are reset([EventStream::accept_all]).
    ///
    /// # Errors and panics
    ///
    /// If one of the handlers returns an error or panics the dispatcher stops processing events,
    /// waits for all other spawned handlers to finish and returns the first observed error or
    /// continues to panic with the first observed panic. If you want a method that returns all
    /// observed erros and panics - fill an issue.
    pub async fn dispatch(self) -> Result<(EventStream<P>, Ctx), D::Error> {
        let chain = std::sync::Arc::new(self.chain);
        let ctx = self.ctx;

        let mut events = self.events;
        let mut join_set: tokio::task::JoinSet<Result<StreamEvents, D::Error>> =
            tokio::task::JoinSet::new();
        let (cancellator, cancellation) = tokio::sync::oneshot::channel::<()>();

        // A dummy task to avoid busy select! loop when JoinSet is empty.
        // Should be manually cancelled
        join_set.spawn(async move {
            let _ = cancellation.await;
            Ok(StreamEvents::Continue)
        });

        let mut result: Result<Result<StreamEvents, D::Error>, tokio::task::JoinError> = loop {
            tokio::select! {
                result = events.try_next() => match result {
                    Ok(Some(event)) => {
                        let Ok(handler) = chain.concurrent_dispatch_event(event, ctx.clone()) else {
                            unreachable!(
                                "EventStream filtering set by on and fallback methods drops events without handlers before parsing them"
                            );
                        };

                        join_set.spawn(handler);
                    }
                    Ok(None) => {
                        break Ok(Ok(StreamEvents::Break));
                    }
                    Err(e) => {
                        break Ok(Err(e.into()));
                    }
                },

                result = join_set.join_next() => match result {
                    Some(Ok(Ok(StreamEvents::Continue)))=> continue,
                    Some(Ok(Ok(StreamEvents::Break))) => break Ok(Ok(StreamEvents::Break)),
                    Some(err) => break err,
                    None => unreachable!("Dummy task must be running while we're tokio select! loop")
                }
            }
        };

        let _ = cancellator.send(());
        while let Some(res) = join_set.join_next().await {
            result = result.and(res);
        }

        match result {
            Ok(inner) => inner.map(move |_| {
                events.accept_all();
                (events, ctx)
            }),
            Err(e) => std::panic::resume_unwind(e.into_panic()),
        }
    }
}

pub trait DispatchEvent<Ctx> {
    type Error;
    type Future<'s>: std::future::Future<Output = Result<StreamEvents, Self::Error>>
    where
        Self: 's,
        Ctx: 's;

    fn dispatch_event<'s>(
        &'s mut self,
        ev: Event,
        ctx: &'s mut Ctx,
    ) -> Result<Self::Future<'s>, (Event, &'s mut Ctx)>;
}

pub trait ConcurrentDispatchEvent<Ctx>
where
    Ctx: 'static + Send,
{
    type Error: 'static + Send;
    type Future: 'static + Send + std::future::Future<Output = Result<StreamEvents, Self::Error>>;

    fn concurrent_dispatch_event(&self, ev: Event, ctx: Ctx) -> Result<Self::Future, (Event, Ctx)>;
}

pub struct Fallback<F> {
    f: F,
}

impl<Ctx, E, F> DispatchEvent<Ctx> for Fallback<F>
where
    F: AsyncFnMut(Event, &mut Ctx) -> Result<StreamEvents, E>,
{
    type Error = E;
    // TODO: Wait for `async_fn_traits` stabilization and use AsyncFnMut::CallRefFuture<'s> here
    type Future<'s>
        = Pin<Box<dyn 's + std::future::Future<Output = Result<StreamEvents, E>>>>
    where
        Self: 's,
        Ctx: 's;

    fn dispatch_event<'s>(
        &'s mut self,
        ev: Event,
        ctx: &'s mut Ctx,
    ) -> Result<Self::Future<'s>, (Event, &'s mut Ctx)> {
        Ok(Box::pin((self.f)(ev, ctx)))
    }
}

impl<Ctx, E, F, Fut> ConcurrentDispatchEvent<Ctx> for Fallback<F>
where
    Ctx: 'static + Send,
    E: 'static + Send,
    // TODO: Wait for `async_fn_traits` stabilization and use AsyncFn with for<'a>
    // CallRefFuture<'a>: 'static + Send instead
    F: Fn(Event, Ctx) -> Fut,
    Fut: 'static + Send + std::future::Future<Output = Result<StreamEvents, E>>,
{
    type Error = E;
    type Future = Fut;

    fn concurrent_dispatch_event(&self, ev: Event, ctx: Ctx) -> Result<Self::Future, (Event, Ctx)> {
        Ok((self.f)(ev, ctx))
    }
}

pub struct Match<Ev, F> {
    f: F,
    _phantom: ::std::marker::PhantomData<Ev>,
}

impl<Ctx, Ev, E, F> DispatchEvent<Ctx> for Match<Ev, F>
where
    Ev: EventData,
    F: AsyncFnMut(Arc<Ev>, &mut Ctx) -> Result<StreamEvents, E>,
{
    type Error = E;
    // TODO: Wait for `async_fn_traits` stabilization and use AsyncFnMut::CallRefFuture<'s> here
    type Future<'s>
        = Pin<Box<dyn 's + std::future::Future<Output = Result<StreamEvents, E>>>>
    where
        Self: 's,
        Ctx: 's;

    fn dispatch_event<'s>(
        &'s mut self,
        ev: Event,
        ctx: &'s mut Ctx,
    ) -> Result<Self::Future<'s>, (Event, &'s mut Ctx)> {
        match Ev::from_event(ev) {
            Ok(ev) => Ok(Box::pin((self.f)(ev, ctx))),
            Err(ev) => Err((ev, ctx)),
        }
    }
}

impl<Ctx, Ev, E, F, Fut> ConcurrentDispatchEvent<Ctx> for Match<Ev, F>
where
    Ctx: 'static + Send,
    Ev: 'static + EventData,
    E: 'static + Send,
    // TODO: Wait for `async_fn_traits` stabilization and use AsyncFn with for<'a>
    // CallRefFuture<'a>: 'static instead
    F: Fn(Arc<Ev>, Ctx) -> Fut,
    Fut: 'static + Send + std::future::Future<Output = Result<StreamEvents, E>>,
{
    type Error = E;
    type Future = Fut;

    fn concurrent_dispatch_event(&self, ev: Event, ctx: Ctx) -> Result<Self::Future, (Event, Ctx)> {
        match Ev::from_event(ev) {
            Ok(ev) => Ok((self.f)(ev, ctx)),
            Err(ev) => Err((ev, ctx)),
        }
    }
}

pub struct Intercept<D1, D2> {
    d1: D1,
    d2: D2,
}

impl<Ctx, D1, D2> DispatchEvent<Ctx> for Intercept<D1, D2>
where
    D1: DispatchEvent<Ctx>,
    D2: DispatchEvent<Ctx, Error = D1::Error>,
{
    type Error = D1::Error;
    type Future<'s>
        = futures::future::Either<D1::Future<'s>, D2::Future<'s>>
    where
        Self: 's,
        Ctx: 's;

    fn dispatch_event<'s>(
        &'s mut self,
        ev: Event,
        ctx: &'s mut Ctx,
    ) -> Result<Self::Future<'s>, (Event, &'s mut Ctx)> {
        self.d1
            .dispatch_event(ev, ctx)
            .map(futures::future::Either::Left)
            .or_else(|(ev, ctx)| {
                self.d2
                    .dispatch_event(ev, ctx)
                    .map(futures::future::Either::Right)
            })
    }
}

impl<Ctx, D1, D2> ConcurrentDispatchEvent<Ctx> for Intercept<D1, D2>
where
    Ctx: 'static + Send,
    D1: ConcurrentDispatchEvent<Ctx>,
    D2: ConcurrentDispatchEvent<Ctx, Error = D1::Error>,
{
    type Error = D1::Error;
    type Future = futures::future::Either<D1::Future, D2::Future>;

    fn concurrent_dispatch_event(&self, ev: Event, ctx: Ctx) -> Result<Self::Future, (Event, Ctx)> {
        self.d1
            .concurrent_dispatch_event(ev, ctx)
            .map(futures::future::Either::Left)
            .or_else(|(ev, ctx)| {
                self.d2
                    .concurrent_dispatch_event(ev, ctx)
                    .map(futures::future::Either::Right)
            })
    }
}
