#[cfg(feature = "crypto")]
pub mod crypto;
#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "websocket")]
pub mod ws;

pub mod bot;
pub mod dispatcher;
pub mod ext;
pub mod id;
pub mod messages;
pub mod prelude;

pub use simploxide_api_types::{
    self as types,
    client_api::{self, BadResponseError, ClientApi, ClientApiError},
    commands, events,
    events::{Event, EventKind},
    responses,
    utils::CommandSyntax,
};

#[cfg(feature = "cancellation")]
pub use tokio_util::{self, sync::CancellationToken};

pub use dispatcher::{DispatchChain, LocalDispatchChain};

use futures::{Stream, TryStreamExt as _};

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
    pub fn set_filter<I: IntoIterator<Item = EventKind>>(&mut self, f: Filter<I>) -> &mut Self {
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

    /// Waits for a particular event `Ev` **dropping** other events in the process. This method is
    /// mostly useful in bot initialisation scenarios when the bot doesn't have any active users.
    /// Misusing this method may result in not receiving user messages and other important events.
    pub async fn wait_for<Ev: events::EventData>(&mut self) -> Result<Option<Arc<Ev>>, P::Error> {
        self.reject_all();
        self.accept(Ev::KIND);
        let result = self.try_next().await;
        self.accept_all();

        let ev = result?;
        Ok(ev.map(|ev| Ev::from_event(ev).unwrap()))
    }

    /// Waits for one one of the events in the `kinds` list **dropping** other events in the
    /// process. Returns the first encountered event of the specified kind. This method is mostly
    /// useful in bot initialisation scenarios when the bot doesn't have any active users. Misusing
    /// this method may result in not receiving user messages and other important events.
    pub async fn wait_for_any(
        &mut self,
        kinds: impl IntoIterator<Item = EventKind>,
    ) -> Result<Option<Event>, P::Error> {
        self.set_filter(Filter::Accept(kinds));
        let result = self.try_next().await;
        self.accept_all();
        result
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
