//! Zero-cost[^1] type-safe event dispatchers handling events mostly at compile time(event matching logic should
//! reduce to a jump table used in a loop, the implementation doesn't use runtime maps or
//! virtual calls giving the compiler full information required to do optimizations). See
//! [`DispatchChain`] for a quick start
//!
//! [^1]: _Sequential dispatchers erase types with `Box<dyn Future>` making them non-zerocost, this
//! will be fixed after `async_fn_traits` stabilization, today's Rust lacks expressive power to
//! bind Future lifetimes correctly. For sequential scenarios reading events as a stream can
//! produce more efficient assembly_

use futures::TryStreamExt as _;
use simploxide_api_types::events::{Event, EventData};
#[cfg(feature = "cancellation")]
use tokio_util::sync::CancellationToken;

use std::{future::Future, pin::Pin, sync::Arc};

use crate::{EventParser, EventStream, StreamEvents};

/// [`Dispatcher`] builder. Obtained from [`EventStream::into_dispatcher`].
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

    /// The sequential version of [Self::fallback]
    pub fn seq_fallback<E, F>(mut self, f: F) -> Dispatcher<P, Ctx, Fallback<F>>
    where
        F: AsyncFnMut(Event, &mut Ctx) -> Result<StreamEvents, E>,
    {
        self.events.accept_all();
        Dispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Fallback { f },
        }
    }

    /// Accepts all events([events::Event]) unhandled by regular handlers.
    /// It is mostly useful for debug logging. Prefer to remove this call in production because it
    /// forces full parsing of all event types when by default only the events with set up handlers
    /// get parsed.
    pub fn fallback<E, F, Fut>(mut self, f: F) -> Dispatcher<P, Ctx, Fallback<F>>
    where
        Ctx: 'static + Send,
        E: 'static + Send + From<P::Error>,
        F: Fn(Event, Ctx) -> Fut,
        Fut: 'static + Send + Future<Output = Result<StreamEvents, E>>,
    {
        self.events.accept_all();
        Dispatcher {
            events: self.events,
            ctx: self.ctx,
            chain: Fallback { f },
        }
    }

    /// Register the sequential handler. This call determines the [`Dispatcher`] type. You won't be
    /// able to mix `seq` with `on`,  after calling this all handlers must be `seq`. See
    /// [`Dispatcher::seq`] for full docs.
    pub fn seq<Ev, E, F>(mut self, f: F) -> Dispatcher<P, Ctx, Match<Ev, F>>
    where
        Ev: EventData,
        F: AsyncFnMut(Arc<Ev>, &mut Ctx) -> Result<StreamEvents, E>,
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

    /// Register the concurrent handler. This call determines the [`Dispatcher`] type. You won't be
    /// able to mix `seq` with `on`,  after calling this all handlers must be `on`. See
    /// [`Dispatcher::on`] for full docs. Concurrent handlers require `Ctx: 'static + Send +
    /// Clone`. `Ctx` is getting cloned into each handler, ensure that `impl Clone for Ctx` is
    /// cheap. If you need to mutate the `Ctx` you will have to use `Arc<Mutex>` or similar.
    pub fn on<Ev, E, F, Fut>(mut self, f: F) -> Dispatcher<P, Ctx, Match<Ev, F>>
    where
        Ctx: 'static + Send,
        E: 'static + Send,
        Ev: 'static + EventData,
        F: Fn(Arc<Ev>, Ctx) -> Fut,
        Fut: 'static + Send + Future<Output = Result<StreamEvents, E>>,
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

/// Builds a compile-time dispatch chain in its type parameter `D`.
/// The chain is unrolled in [`Self::sequential_dispatch`] or [`Self::dispatch`].
///
/// Use [`Self::seq`] to add sequential handlers (`&mut Ctx`, `!Send` future) or
/// [`Self::on`] to add concurrent handlers (`Ctx: Clone + Send`, spawned as tasks).
pub struct Dispatcher<P, Ctx, D> {
    events: EventStream<P>,
    ctx: Ctx,
    chain: D,
}

impl<P, Ctx, D> Dispatcher<P, Ctx, D>
where
    D: DispatchEvent<Ctx>,
{
    /// Register a sequential event handler.
    ///
    /// - The handler signature is `AsyncFnMut(ev: Arc<{EventType}>, ctx: &mut Ctx) -> Result<StreamEvents, {ErrorType}>`
    /// - `Ctx` is whatever type you pass when creating the dispatcher with
    ///   `events.into_local_dispatcher(ctx)`. It will be passed by a mutable reference into all handlers
    /// - `{Eventype}` is one of the data structs defined in [events]. Events are dispatched
    ///   statically so this type links the handler to event. When handlers with the same `{EvenType}` are set
    ///   multiple times the last one wins.
    /// - `{ErrorType}` can be arbitrary but all handlers must share it and it must implement `From<ClientError>`.
    /// - Events are processed one at a time; handlers have exclusive `&mut` access to `Ctx`.
    ///
    /// ## Usage
    ///
    /// - Set async fn
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .seq(contact_connected)
    ///     .sequential_dispatch()
    ///     .await;
    ///
    /// async fn contact_connected(
    ///     ev: Arc<ContactConnected>,
    ///     ctx: &mut ws::Client,
    /// ) -> ClientResult<StreamEvents> { ... }
    /// ```
    ///
    /// - Set async closure by fully qualifying types
    ///
    /// ```rust
    /// events.into_local_dispatcher(client)
    ///     .seq_fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .seq::<ContactConnected, _>(async |ev, &mut client| { ... })
    ///     .sequential_dispatch()
    ///     .await;
    /// ```
    ///
    /// - Set async closure by specifying closure argument type
    ///
    /// ```rust
    /// events.into_local_dispatcher(client)
    ///     .seq_fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .seq(async |ev: Arc<ContactConnected>, &mut client| {
    ///         //...
    ///     })
    ///     .sequential_dispatch()
    ///     .await;
    /// ```
    pub fn seq<Ev, F>(mut self, f: F) -> Dispatcher<P, Ctx, Intercept<Match<Ev, F>, D>>
    where
        Ev: EventData,
        F: AsyncFnMut(Arc<Ev>, &mut Ctx) -> Result<StreamEvents, D::Error>,
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

    /// Dispatch events sequentially. Handlers block the event loop, allowing exclusive `&mut Ctx`
    /// access. Returning [`StreamEvents::Break`] stops the dispatcher and returns the event stream
    /// and `ctx` for further processing.
    ///
    /// Produces a `!Send` future that can be used with [`tokio::task::LocalSet`], on the main
    /// tokio thread, or with a single-threaded runtime.
    pub async fn sequential_dispatch(self) -> Result<(EventStream<P>, Ctx), D::Error>
    where
        P: EventParser,
        D::Error: From<P::Error>,
    {
        let Self {
            ctx,
            events,
            mut chain,
        } = self;

        events.stream_events_with_ctx_mut(async move |ev, ctx| {
            let Ok(handler) = chain.dispatch_event(ev, ctx) else {
                unreachable!("EventStream filters set by seq/fallback_seq drop events without handlers during parsing");
            };

            handler.await

        }, ctx).await
    }

    /// Like [`Self::sequential_dispatch`] but stops when `token` is cancelled. Token cancellation
    /// is equivalent to returning [`StreamEvents::Break`].
    #[cfg(feature = "cancellation")]
    pub async fn sequential_dispatch_with_cancellation(
        self,
        token: CancellationToken,
    ) -> Result<(EventStream<P>, Ctx), D::Error>
    where
        P: EventParser,
        D::Error: From<P::Error>,
    {
        let Self {
            mut ctx,
            mut events,
            mut chain,
        } = self;

        loop {
            tokio::select! {
                biased;
                _ = token.cancelled() => break,
                res = events.try_next() => match res {
                    Ok(Some(ev)) => {
                        let Ok(handler) = chain.dispatch_event(ev, &mut ctx) else {
                            unreachable!("EventStream filters set by seq/fallback_seq drop events without handlers during parsing");
                        };
                        if let StreamEvents::Break = handler.await? {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }

        Ok((events, ctx))
    }
}

impl<P, Ctx, D> Dispatcher<P, Ctx, D>
where
    P: 'static + EventParser,
    Ctx: 'static + Send + Clone,
    D: ConcurrentDispatchEvent<Ctx>,
    D::Error: From<P::Error>,
{
    /// Register a concurrent event handler.
    ///
    /// - The handler signature is `AsyncFn(ev: Arc<{EventType}>, ctx: Ctx) -> Result<StreamEvents, {ErrorType}>;`
    /// - `Ctx` is whatever is passed into the `into_dispatcher` call. It is cloned into every handler invocation
    /// - `{Eventype}` is one of the data structs defined in [events]. Events are dispatched
    ///   statically so this type links the handler to event. When handlers with the same `{EvenType}` are set
    ///   multiple times the last one wins.
    /// - `{ErrorType}` can be arbitrary but all handlers must share it and it must implement `From<ClientError>`.
    /// - All handlers run as tokio tasks so events are processed concurrently(and in parallel on
    /// multithreaded runtimes).
    ///
    ///
    /// ## Usage
    ///
    /// - Set async fn
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .on(contact_connected)
    ///     .dispatch()
    ///     .await;
    ///
    /// async fn contact_connected(
    ///     ev: Arc<ContactConnected>,
    ///     ctx: ws::Client,
    /// ) -> ClientResult<StreamEvents> { ... }
    /// ```
    ///
    /// - Set async closure by fully qualifying types
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on::<ContactConnected, _, _>(async |ev, client| { ... })
    ///     .dispatch()
    ///     .await;
    /// ```
    ///
    /// - Set async closure by specifying closure argument type
    ///
    /// ```rust
    /// events.into_dispatcher(client)
    ///     .fallback(async |ev, _| log::debug!("{ev:?}"))
    ///     .on(async |ev: Arc<ContactConnected>, client| { ... })
    ///     .dispatch()
    ///     .await;
    /// ```
    pub fn on<Ev, F, Fut>(mut self, f: F) -> Dispatcher<P, Ctx, Intercept<Match<Ev, F>, D>>
    where
        Ev: 'static + EventData,
        F: Fn(Arc<Ev>, Ctx) -> Fut,
        Fut: 'static + Send + Future<Output = Result<StreamEvents, D::Error>>,
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

    /// Spawns handlers as tokio tasks. Handlers execute and resolve in arbitrary order.
    /// [`StreamEvents::Break`] eventually stops the dispatcher after all in-flight handlers finish.
    /// The returned [`EventStream`] filters should be reset via [`EventStream::accept_all`] if you
    /// want to query the stream manually and process all events afterwards.
    ///
    /// # Errors and panics
    ///
    /// If a handler returns an error or panics, the dispatcher stops, waits for in-flight handlers
    /// to complete, then returns the first error or resumes the first panic.
    pub async fn dispatch(self) -> Result<(EventStream<P>, Ctx, Vec<Event>), D::Error> {
        let chain = self.chain;
        let ctx = self.ctx;
        let mut events = self.events;
        let (event_buffer, result) =
            run_concurrent_dispatch(&chain, &ctx, &mut events, std::future::pending::<()>()).await;
        match result {
            Ok(inner) => inner.map(move |_| (events, ctx, event_buffer)),
            Err(e) => std::panic::resume_unwind(e.into_panic()),
        }
    }

    /// Like [`Self::dispatch`] but stops when `token` is cancelled. Token cancellation behaviour
    /// is equivalent to returning [`StreamEvents::Break`].
    #[cfg(feature = "cancellation")]
    pub async fn dispatch_with_cancellation(
        self,
        token: CancellationToken,
    ) -> Result<(EventStream<P>, Ctx, Vec<Event>), D::Error> {
        let chain = self.chain;
        let ctx = self.ctx;
        let mut events = self.events;
        let (event_buffer, result) =
            run_concurrent_dispatch(&chain, &ctx, &mut events, token.cancelled()).await;
        match result {
            Ok(inner) => inner.map(move |_| (events, ctx, event_buffer)),
            Err(e) => std::panic::resume_unwind(e.into_panic()),
        }
    }

    /// Runs concurrent handlers one at a time, producing a `'static + Send` future.
    ///
    /// Unlike [Dispatcher::sequential_dispatch] this clones `Ctx` per event (no `&mut`) and the
    /// resulting future is `Send`. Use when handler execution order matters but you need a
    /// sendable future, e.g. inside `tokio::spawn`.
    pub async fn dispatch_sequentially(self) -> Result<(EventStream<P>, Ctx), D::Error> {
        let ctx = self.ctx;
        let events = self.events;
        let chain = self.chain;

        events.stream_events_with_ctx_cloned(async move |ev, ctx| {
            let Ok(handler) = chain.concurrent_dispatch_event(ev, ctx) else {
                unreachable!("EventStream filters set by on/fallback drop events without handlers during parsing");
            };
            handler.await
        }, ctx).await
    }

    /// Like [`Self::dispatch_sequentially`] but stops when `token` is cancelled.
    #[cfg(feature = "cancellation")]
    pub async fn dispatch_sequentially_with_cancellation(
        self,
        token: CancellationToken,
    ) -> Result<(EventStream<P>, Ctx), D::Error> {
        let Self {
            ctx,
            mut events,
            chain,
        } = self;

        loop {
            tokio::select! {
                biased;
                _ = token.cancelled() => break,
                res = events.try_next() => match res {
                    Ok(Some(ev)) => {
                        let Ok(handler) = chain.concurrent_dispatch_event(ev, ctx.clone()) else {
                            unreachable!("EventStream filters set by on/fallback drop events without handlers during parsing");
                        };
                        if let StreamEvents::Break = handler.await? {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }

        Ok((events, ctx))
    }
}

// Drives the main dispatch loop until the event stream closes, a handler signals
// Break/Err, or the stop future resolves. Then drains the join set to completion,
// concurrently pulling from the event stream so that handlers blocked on incoming
// events (e.g. XFTP downloads) can make progress. Returns buffered events that
// arrived during the drain and the final loop result.
async fn run_concurrent_dispatch<P, Ctx, D, Fut>(
    chain: &D,
    ctx: &Ctx,
    events: &mut EventStream<P>,
    stop: Fut,
) -> (
    Vec<Event>,
    Result<Result<StreamEvents, D::Error>, tokio::task::JoinError>,
)
where
    P: 'static + EventParser,
    Ctx: 'static + Send + Clone,
    D: ConcurrentDispatchEvent<Ctx>,
    D::Error: From<P::Error>,
    Fut: Future<Output = ()>,
{
    let mut join_set: tokio::task::JoinSet<Result<StreamEvents, D::Error>> =
        tokio::task::JoinSet::new();
    let (cancellator, cancellation) = tokio::sync::oneshot::channel::<()>();

    // A dummy task to avoid a busy select! loop when JoinSet is empty.
    join_set.spawn(async move {
        let _ = cancellation.await;
        Ok(StreamEvents::Continue)
    });

    let mut stop = std::pin::pin!(stop);

    let mut result = loop {
        tokio::select! {
            _ = stop.as_mut() => break Ok(Ok(StreamEvents::Break)),
            result = events.try_next() => match result {
                Ok(Some(event)) => {
                    let Ok(handler) = chain.concurrent_dispatch_event(event, ctx.clone()) else {
                        unreachable!(
                            "EventStream filtering set by on and fallback methods drops events without handlers before parsing them"
                        );
                    };
                    join_set.spawn(handler);
                }
                Ok(None) => break Ok(Ok(StreamEvents::Break)),
                Err(e) => break Ok(Err(e.into())),
            },
            result = join_set.join_next() => match result {
                Some(Ok(Ok(StreamEvents::Continue))) => continue,
                Some(Ok(Ok(StreamEvents::Break))) => break Ok(Ok(StreamEvents::Break)),
                Some(err) => break err,
                None => unreachable!("Dummy task must be running during the whole tokio select! loop"),
            }
        }
    };

    let _ = cancellator.send(());
    let mut event_buffer = Vec::new();

    loop {
        tokio::select! {
            joined = join_set.join_next() => match joined {
                Some(next) => {
                    if matches!(result, Ok(Ok(_))) {
                        result = next;
                    }
                }
                None => break,
            },
            event = events.try_next() => match event {
                Ok(Some(ev)) => event_buffer.push(ev),
                Ok(None) => (),
                Err(e) => {
                    result = Ok(Err(e.into()));
                    break;
                }
            }
        }
    }

    (event_buffer, result)
}

pub trait DispatchEvent<Ctx> {
    type Error;
    type Future<'s>: Future<Output = Result<StreamEvents, Self::Error>>
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
    type Future: 'static + Send + Future<Output = Result<StreamEvents, Self::Error>>;

    fn concurrent_dispatch_event(&self, ev: Event, ctx: Ctx) -> Result<Self::Future, (Event, Ctx)>;
}

// ---- Dispatcher combinators -----

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
        = Pin<Box<dyn 's + Future<Output = Result<StreamEvents, E>>>>
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
    Fut: 'static + Send + Future<Output = Result<StreamEvents, E>>,
{
    type Error = E;
    type Future = Fut;

    fn concurrent_dispatch_event(&self, ev: Event, ctx: Ctx) -> Result<Self::Future, (Event, Ctx)> {
        Ok((self.f)(ev, ctx))
    }
}

pub struct Match<Ev, F> {
    f: F,
    _phantom: std::marker::PhantomData<Ev>,
}

impl<Ctx, Ev, E, F> DispatchEvent<Ctx> for Match<Ev, F>
where
    Ev: EventData,
    F: AsyncFnMut(Arc<Ev>, &mut Ctx) -> Result<StreamEvents, E>,
{
    type Error = E;
    // TODO: Wait for `async_fn_traits` stabilization and use AsyncFnMut::CallRefFuture<'s> here
    type Future<'s>
        = Pin<Box<dyn 's + Future<Output = Result<StreamEvents, E>>>>
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
    Fut: 'static + Send + Future<Output = Result<StreamEvents, E>>,
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
