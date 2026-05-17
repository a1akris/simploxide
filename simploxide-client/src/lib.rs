//! For first-time users it's recommended to get hands-on experience by running some example bots
//! on [GitHub](https://github.com/a1akris/simploxide/tree/main/simploxide-client) before writing
//! their own.
//!
//! This SDK is supposed to be used with `tokio` runtime. Here are the steps to implement any bot:
//!
//! ### 1. Choose a backend
//!
//! `simploxide` provides support for both **websocket** and **FFI** SimpleX-Chat backends.
//! `simploxide` also reimplements all **FFI** exclusive methods in native Rust so in practice the
//! backends differ only by their runtime characteristics: a single-process app via **FFI** _vs_ an
//! app depending on SimpleX-Chat **websocket** server.
//!
//! Since backends are equally powerful always start developing with the **websocket**
//! backend(enabled by default). Switching code to **FFI** later is as simple as replacing `ws`
//! imports with `ffi` imports but **FFI** requires configuring the crate build and obliges you to
//! use AGPL-3.0 license. You can read more about switching to **FFI**
//! [here](simploxide-sxcrt-sys).
//!
//! ### 2. Initialise the bot
//!
//! `simploxide` provides convenient bot builders to launch and configure your bot
//!
//! ```ignore
//!
//! let (bot, events, mut cli) = ws::BotBuilder::new("YesMan", 5225)
//!     .db_prefix("db/bot")
//!     // create a public bot address auto-accepting new users with a welcome message
//!     .auto_accept_with(
//!         "Hello, I'm a bot always agreeing with my users",
//!     )
//!     // Launch CLI, connect the client, and initialise the bot
//!     .launch()
//!     .await?;
//!
//! let address = bot.address().await?;
//! println!("My address: {address}");
//! ```
//!
//! See all available options in [ws::BotBuilder] and [ffi::BotBuilder].
//!
//! ### 3. Set up event dispatcher
//!
//! Dispatchers are zero-cost and provide a convenient API to handle events
//!
//! ```ignore
//! // into_dispatcher accepts any type and creates a dispatcher from the event stream.
//! // The value provided here is then passed into all event handlers as a second argument
//! events.into_dispatcher(bot)
//!     .on(new_messages)
//!     .dispatch()
//!     .await?;
//! ```
//!
//! Learn more about dispatchers in [dispatcher] and [EventStream] docs.
//!
//! ### 4. Implement event handlers
//!
//! The first handler argument determines which event the handler processes. The [StreamEvents]
//! type allows to interrupt event dispatching with [`StreamEvents::Break`].
//!
//! ```ignore
//! async fn new_msgs(ev: Arc<NewChatItems>, bot: Bot) -> ws::ClientResult<StreamEvents> {
//!     for (chat, msg, content) in ev.filter_messages() {
//!         bot.update_msg_reaction(chat, msg, Reaction::Set("👍")).await?;
//!
//!         bot.send_msg(chat, "I absolutely agree with this!".bold())
//!            .reply_to(msg);
//!            .await?;
//!     }
//!
//!     Ok(StreamEvents::Continue)
//! }
//! ```
//!
//! Bot message builders are very powerful and flexible, learn more about them in [messages]. In
//! most places where some kind of ID is expected you can pass a struct directly, see what
//! type-safe conversions are available in [id]
//!
//! ### 5. Execute cleanup before exiting
//!
//! ```ignore
//! bot.shutdown().await;
//! cli.kill().await?;
//! ```
//!
//! ## Features
//!
//! -
//! -
//!
//! ### How to work with this documentation?
//!
//! The [bot] page should become your primary page and [events] page should become your secondary
//! page. From these 2 pages you should be able to find everything in a structured manner.

#[cfg(feature = "crypto")]
pub mod crypto;
#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "websocket")]
pub mod ws;
#[cfg(feature = "xftp")]
pub mod xftp;

pub mod bot;
pub mod dispatcher;
pub mod ext;
pub mod id;
pub mod messages;
pub mod prelude;
pub mod preview;

mod util;

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

pub use dispatcher::DispatchChain;

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
/// Use [`Self::into_dispatcher`] to handle events conveniently. Dispatchers are completely
/// zerocost, manage filters internally, and provide a high-level easy to use API covering the
/// absolute majority of use cases.
pub struct EventStream<P> {
    filter: [bool; EventKind::COUNT],
    receiver: tokio::sync::mpsc::UnboundedReceiver<P>,
    hooks: Vec<Box<dyn Hook>>,
}

impl<P> From<tokio::sync::mpsc::UnboundedReceiver<P>> for EventStream<P> {
    fn from(receiver: tokio::sync::mpsc::UnboundedReceiver<P>) -> Self {
        Self {
            filter: [true; EventKind::COUNT],
            receiver,
            hooks: Vec::new(),
        }
    }
}

impl<P> EventStream<P> {
    pub fn add_hook(&mut self, hook: Box<dyn Hook>) {
        self.hooks.push(hook);
    }

    #[cfg(feature = "xftp")]
    pub fn hook_xftp<C: 'static + Clone + ClientApi>(&mut self, client: C) -> xftp::XftpClient<C> {
        let xftp_client = xftp::XftpClient::from(client);
        let hook = xftp_client.clone();
        self.add_hook(Box::new(hook));

        xftp_client
    }

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

impl<P: EventParser> EventStream<P> {
    /// Turns stream into a [`DispatchChain`] builder with the provided `ctx`. The `ctx` is an
    /// arbitrary type that can be used within event handlers. Use [`Dispatcher::seq`] to add
    /// sequential handlers: `AsyncFnMut(Arc<Ev>, &mut Ctx)`; or [`Dispatcher::on`] for concurrent
    /// ones: `AsyncFn(Arc<Ev>, Ctx) where Ctx: 'static + Clone + Send`.
    pub fn into_dispatcher<C>(self, ctx: C) -> DispatchChain<P, C> {
        DispatchChain::with_ctx(self, ctx)
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

pub enum Filter<I: IntoIterator<Item = EventKind>> {
    Accept(I),
    AcceptAll,
    AcceptAllExcept(I),
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

                    if !self.hooks.iter().any(|h| h.should_intercept(kind))
                        && !self.filter[kind.as_usize()]
                    {
                        continue;
                    }

                    match raw_event.parse_event() {
                        Ok(event) => {
                            for hook in self.hooks.iter_mut() {
                                if hook.should_intercept(kind) {
                                    hook.intercept_event(event.clone());
                                }
                            }

                            if self.filter[kind.as_usize()] {
                                break Poll::Ready(Some(Ok(event)));
                            }
                        }
                        Err(e) => break Poll::Ready(Some(Err(e))),
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

pub trait Hook {
    fn should_intercept(&self, kind: EventKind) -> bool;

    /// Hooks shouldn't block the event stream so this method is supposed to be a cheap
    /// sync call. You can delegate work to another thread or spawn async tasks internally.
    fn intercept_event(&mut self, event: Event);
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

        pub fn ttl_to_secs(ttl: std::time::Duration) -> i32 {
            let clamped = std::cmp::min(ttl, TTL_MAX);
            clamped.as_secs() as i32
        }

        pub fn always(ttl: std::time::Duration) -> Option<TimedMessagesPreference> {
            Some(TimedMessagesPreference {
                allow: FeatureAllowed::Always,
                ttl: Some(ttl_to_secs(ttl)),
                undocumented: serde_json::Value::Null,
            })
        }

        pub fn yes(ttl: std::time::Duration) -> Option<TimedMessagesPreference> {
            Some(TimedMessagesPreference {
                allow: FeatureAllowed::Yes,
                ttl: Some(ttl_to_secs(ttl)),
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
