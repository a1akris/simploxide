//! Events demultiplexer

use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot,
};

use std::sync::Arc;

use super::BotId;
use crate::{EventParser, EventStream};

pub type Suspender = UnboundedSender<oneshot::Receiver<()>>;
pub type Blocker = UnboundedReceiver<oneshot::Receiver<()>>;
pub type FxDashMap<K, V> = dashmap::DashMap<K, V, rustc_hash::FxBuildHasher>;
pub type BotMap<P> = FxDashMap<BotId, Channel<P>>;

pub fn start<P: 'static + Send + EventParser>(
    map: Arc<BotMap<P>>,
    global_stream: EventStream<P>,
) -> (Suspender, EventStream<P>) {
    let (sender, receiver) = mpsc::unbounded_channel();
    let (suspender, blocker): (Suspender, Blocker) = mpsc::unbounded_channel();

    tokio::spawn(Box::pin(task(map, blocker, sender, global_stream)));
    (suspender, EventStream::from(receiver))
}

async fn task<P: 'static + Send + EventParser>(
    map: Arc<BotMap<P>>,
    mut blocker: Blocker,
    fallback_sender: UnboundedSender<P>,
    global_stream: EventStream<P>,
) {
    // TODO: investigate if there are use-cases for global hooks
    let mut receiver = global_stream.into_receiver();

    loop {
        tokio::select! {
            biased;

            block = blocker.recv() => {
                if let Some(suspension) = block {
                    let _ = suspension.await;
                }
            }

            ev = receiver.recv() => match ev {
                Some(ev) => if let Err(ev) = try_demux(&map, ev) {
                    let _ = fallback_sender.send(ev);
                }
                None => break,
            }
        }
    }

    blocker.close();
    while blocker.recv().await.is_some() {}
}

fn try_demux<P: 'static + Send + EventParser>(map: &Arc<BotMap<P>>, ev: P) -> Result<(), P> {
    let Ok(Some(id)) = ev.parse_user_id() else {
        return Err(ev);
    };

    let Some(entry) = map.get(&BotId::from(id)) else {
        return Err(ev);
    };

    if let Channel::Bot(pipe) = entry.value() {
        pipe.send(ev);
        Ok(())
    } else {
        Err(ev)
    }
}

pub enum Channel<P> {
    Ghost,
    Bot(Pipe<P>),
}

impl<P> Channel<P> {
    pub fn new_bot() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self::Bot(Pipe::new(tx, rx))
    }

    pub fn take_receiver(&mut self) -> Option<UnboundedReceiver<P>> {
        match self {
            Self::Ghost => None,
            Self::Bot(pipe) => pipe.take_receiver(),
        }
    }

    pub fn is_ghost(&self) -> bool {
        matches!(self, Self::Ghost)
    }
}

pub struct Pipe<P> {
    sender: UnboundedSender<P>,
    receiver: Option<UnboundedReceiver<P>>,
}

impl<P> Pipe<P> {
    fn new(sender: UnboundedSender<P>, receiver: UnboundedReceiver<P>) -> Self {
        Self {
            sender,
            receiver: Some(receiver),
        }
    }

    pub fn send(&self, ev: P) {
        let _ = self.sender.send(ev);
    }

    pub fn take_receiver(&mut self) -> Option<UnboundedReceiver<P>> {
        self.receiver.take()
    }
}
