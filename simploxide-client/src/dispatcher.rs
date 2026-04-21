use futures::TryStreamExt as _;
use simploxide_api_types::events::{Event, EventData};
#[cfg(feature = "cancellation")]
use tokio_util::sync::CancellationToken;

use std::{pin::Pin, sync::Arc};

use crate::{EventParser, EventStream, StreamEvents};

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

    /// Like [Self::dispatch] but allows to cancel the dispatching by some external signal
    /// triggering the [CancellationToken]. The behaviour of triggering the cancellation token is
    /// equivalent of returning [StreamEvents::Break].
    #[cfg(feature = "cancellation")]
    pub async fn dispatch_with_cancellation(
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

                _ = token.cancelled() => {
                    break;
                }

                res = events.try_next() => match res {
                    Ok(Some(ev)) => {
                        let Ok(handler) = chain.dispatch_event(ev, &mut ctx) else {
                            unreachable!("EventStream filters set by on/fallback methods drop events without handlers during parsing");
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

        events.accept_all();
        Ok((events, ctx))
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

    /// **WARNING:** This is not a concurrent dispatch!
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

    #[cfg(feature = "cancellation")]
    /// Like [Self::sequential_dispatch] but allows to cancel the dispatching by some external signal
    /// triggering the [CancellationToken]. The behaviour of triggering the cancellation token is
    /// equivalent of returning [StreamEvents::Break].
    pub async fn sequential_dispatch_with_cancellation(
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

                _ = token.cancelled() => {
                    break;
                }

                res = events.try_next() => match res {
                    Ok(Some(ev)) => {
                        let Ok(handler) = chain.concurrent_dispatch_event(ev, ctx.clone()) else {
                            unreachable!("EventStream filters set by on/fallback methods drop events without handlers during parsing");
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

        events.accept_all();
        Ok((events, ctx))
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
        let chain = Arc::new(self.chain);
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
                    None => unreachable!("Dummy task must be running during the whole tokio select! loop")
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

    #[cfg(feature = "cancellation")]
    /// Like [Self::dispatch] but allows to cancel the dispatching by some external signal
    /// triggering the [CancellationToken]. The behaviour of triggering the cancellation token is
    /// equivalent of returning [StreamEvents::Break].
    // TODO: This is a direct copy-paste of dispatch with an extra select branch. Try to reduce
    // code-duplication
    pub async fn dispatch_with_cancellation(
        self,
        token: CancellationToken,
    ) -> Result<(EventStream<P>, Ctx), D::Error> {
        let chain = Arc::new(self.chain);
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
                _ = token.cancelled() => {
                    break Ok(Ok(StreamEvents::Break));
                }
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
                    None => unreachable!("Dummy task must be running during the whole tokio select! loop")
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
