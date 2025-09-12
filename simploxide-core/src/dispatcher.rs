//! Event dispatcher task.

use std::{sync::Arc, task::Poll};

use crate::{WsIn, router::ResponseRouter};
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use super::{Event, RequestId, Result};

type EventSender = mpsc::UnboundedSender<Result<Event>>;
type EventReceiver = mpsc::UnboundedReceiver<Result<Event>>;

pub fn init(ws_in: WsIn, router: ResponseRouter, token: CancellationToken) -> EventQueue {
    let (events_tx, receiver) = mpsc::unbounded_channel::<Result<Event>>();
    tokio::spawn(event_dispatcher_task(ws_in, events_tx, router, token));

    EventQueue { receiver }
}

pub struct EventQueue {
    receiver: EventReceiver,
}

impl EventQueue {
    /// Can return a SimpleX event or a [`tungstenite::Error`] if a connection is dropped due to a
    /// web socket failure. SimpleX events can themselves represent SimpleX errors but recognizing
    /// and handling them them is a task of the upstream code.
    pub async fn next_event(&mut self) -> Option<Result<Event>> {
        self.receiver.recv().await
    }

    /// Get the underlying tokio unbounded receiver that enables more complicated use cases.
    pub fn into_receiver(self) -> EventReceiver {
        self.receiver
    }
}

async fn event_dispatcher_task(
    mut ws_in: WsIn,
    mut event_queue: EventSender,
    router: ResponseRouter,
    token: CancellationToken,
) {
    loop {
        tokio::select! {
            ev = ws_in.next() => {
                match ev {
                    Some(Ok(msg)) => {
                        process_raw_event(Some(&router), &mut event_queue, msg);
                    }
                    Some(Err(e)) => {
                        let e = Arc::new(e);
                        let _ = event_queue.send(Err(Arc::clone(&e)));
                        router.shutdown(e);

                        break;
                    }
                    None => unreachable!("Must receive an error before connection drops")

                }
            }
            // Can get cancelled only after router task completion.
            _ = token.cancelled() => {
                // Processing buffered events
                let mut ws_in = Closed(ws_in);
                while let Some(ev) = ws_in.next().await {
                    match ev {
                        Ok(msg) => {
                            process_raw_event(None, &mut event_queue, msg);
                        }
                        Err(e) => {
                            let _ = event_queue.send(Err(Arc::new(e)));
                            break;
                        }
                    }
                }

                break;
            }
        }
    }

    log::debug!("Dispatcher task finished");
}

/// Parse the top level JSON and either route event to the `event_queue` or deliver a response by
/// `corrId` via the `router`.
///
/// TODO: `Option<&Router>` was added to reuse code in a branch that handles the interruption
/// event. In this case all buffered events can only be sent to the `event_queue`. This could be
/// refactored to look less hacky.
fn process_raw_event(router: Option<&ResponseRouter>, event_queue: &mut EventSender, msg: Message) {
    let mut json: serde_json::Value = match msg {
        Message::Text(txt) => serde_json::from_str(&txt).expect("Server sends a valid JSON"),
        unexpected => {
            log::warn!("Ignoring event in unexpecetd format: {unexpected:#?}");
            return;
        }
    };

    let corr_id = json["corrId"].take();

    if !corr_id.is_null() {
        let id: RequestId = corr_id.as_str().unwrap().parse().unwrap();
        let response = json["resp"].take();
        assert!(!response.is_null(), "Server sends a valid resp field");

        if let Some(router) = router {
            router.deliver(id, response);
        } else {
            log::warn!("Dropping response: {response}\nBecause router task already finished");
        }
    } else {
        let event = json["resp"].take();
        // The client may choose to drop the event queue to stop buffering events. This is an
        // expected behavior so errors are ignored.
        if event.is_null() {
            let _ = event_queue.send(Ok(json));
        } else {
            let _ = event_queue.send(Ok(event));
        }
    }
}

/// A helper that allows to process buffered items. Returns `None` when internal stream buffer
/// becomes empty.
struct Closed<S>(S);

impl<S> Stream for Closed<S>
where
    S: Stream + Unpin,
{
    type Item = S::Item;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.0.poll_next_unpin(cx) {
            Poll::Ready(v) => Poll::Ready(v),
            Poll::Pending => Poll::Ready(None),
        }
    }
}
