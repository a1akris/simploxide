use simploxide_sxcrt_sys::{CallError, InitError, SimpleXChat};
use slab::Slab;

use std::{
    sync::{
        Arc, OnceLock,
        mpsc::{self, TryRecvError},
    },
    time::Duration,
};

use crate::{
    ChatCommand, CmdReceiver, DbOpts, DefaultUser, EventTransmitter, RawClient, RawEventQueue,
    ShutdownEmitter, WorkerConfig, default,
};

type NewChatResponse = Result<(RawClient, RawEventQueue), InitError>;
type NewChatResponder = tokio::sync::oneshot::Sender<NewChatResponse>;

pub fn init(config: WorkerConfig) -> &'static Worker {
    static GLOBAL_WORKER: OnceLock<Worker> = OnceLock::new();

    GLOBAL_WORKER.get_or_init(move || {
        let (cmd, ctrl) = mpsc::channel();
        let worker = Worker(cmd.clone());

        WorkerThread::spawn(ctrl, config);
        worker
    })
}

#[derive(Clone)]
pub struct Worker(mpsc::Sender<WorkerCommand>);
type WorkerCtrl = mpsc::Receiver<WorkerCommand>;

impl Worker {
    pub async fn spawn_chat(
        &self,
        default_user: DefaultUser,
        db_opts: DbOpts,
    ) -> Result<(RawClient, RawEventQueue), InitError> {
        let (responder, response) = tokio::sync::oneshot::channel();

        self.0
            .send(WorkerCommand::NewChat(Box::new(NewChatParams {
                worker: self.clone(),
                default_user,
                db_opts,
                responder,
            })))
            .expect("WorkerThread is static");

        response.await.unwrap()
    }

    pub fn wake(&self) {
        let _ = self.0.send(WorkerCommand::Wakeup);
    }
}

struct WorkerThread {
    ctrl: WorkerCtrl,
    chats: Slab<Chat>,
    poll_order: RoundRobin<usize>,
    remove_queue: Vec<usize>,
    max_instances: usize,
    max_event_latency: Duration,
    skipped_iterations: u8,
}

impl WorkerThread {
    fn spawn(ctrl: WorkerCtrl, config: WorkerConfig) {
        std::thread::spawn(move || {
            let capacity = config.max_instances.unwrap_or(default::MAX_CHAT_INSTANCES);

            WorkerThread {
                ctrl,
                chats: Slab::with_capacity(capacity),
                poll_order: RoundRobin::with_capacity(capacity),
                remove_queue: Vec::with_capacity(capacity),
                max_instances: capacity,
                max_event_latency: config
                    .max_event_latency
                    .unwrap_or(default::MAX_EVENT_LATENCY),
                skipped_iterations: 0,
            }
            .run();
        });
    }

    fn run(mut self) {
        let mut has_activity = false;

        loop {
            if self.chats.is_empty() {
                // Blocking wait for new chat instance request
                let cmd = self.ctrl.recv().expect("Sender part is static");
                self.handle_ctrl_cmd(cmd);
                self.drain_ctrl();

                has_activity = true;
                continue;
            }

            // Need to update the order to correctly handle chat instance creations and removals
            // The call reuses the same allocation and is relativley cheap
            self.poll_order.set(self.chats.iter().map(|(k, _)| k));

            for chat_key in self.poll_order.iter().copied() {
                let status = self.chats[chat_key].handle_buffered_actions();
                has_activity |= status.not_skipped();

                if status.is_terminated() {
                    self.remove_queue.push(chat_key);
                }
            }

            for chat_key in self.remove_queue.drain(..) {
                let chat = self.chats.remove(chat_key);
                chat.close();
            }

            has_activity |= self.drain_ctrl();

            if has_activity {
                self.poll_order.advance();
                self.skipped_iterations = 0;
                has_activity = false;
            } else {
                const EXTRA_SPINS: u8 = 4;
                const SLEEP_STEP: u64 = 50;

                self.skipped_iterations = self.skipped_iterations.saturating_add(1);

                if self.skipped_iterations <= EXTRA_SPINS {
                    std::thread::yield_now();
                    continue;
                }

                let sleep_interval = std::cmp::min(
                    self.max_event_latency,
                    Duration::from_millis(
                        (self.skipped_iterations - EXTRA_SPINS) as u64 * SLEEP_STEP,
                    ),
                );

                if let Ok(cmd) = self.ctrl.recv_timeout(sleep_interval) {
                    self.handle_ctrl_cmd(cmd);
                    has_activity = true;
                }
            }
        }
    }

    /// Returns true if there were any ctrl commands
    ///
    /// Drain is needed to cleanup multiple wakeups at once
    fn drain_ctrl(&mut self) -> bool {
        let mut drained = false;
        while let Ok(cmd) = self.ctrl.try_recv() {
            drained = true;
            self.handle_ctrl_cmd(cmd);
        }

        drained
    }

    fn handle_ctrl_cmd(&mut self, cmd: WorkerCommand) {
        match cmd {
            WorkerCommand::NewChat(new_chat_params) => {
                self.spawn_chat(new_chat_params);
            }

            WorkerCommand::Wakeup => {}
        }
    }

    #[allow(clippy::boxed_local)]
    fn spawn_chat(&mut self, params: Box<NewChatParams>) {
        let responder = params.responder;

        if self.chats.len() >= self.max_instances {
            let _ = responder.send(Err(CallError::Failure.into()));
            return;
        }

        let chat = match simplex_chat_init(params.default_user, params.db_opts) {
            Ok(chat) => chat,
            Err(e) => {
                let _ = responder.send(Err(e));
                return;
            }
        };

        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (ev_tx, ev_rx) = tokio::sync::mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

        let client = RawClient {
            tx: cmd_tx,
            worker: params.worker,
            shutdown: shutdown_rx,
        };

        let events = RawEventQueue { receiver: ev_rx };

        self.chats.insert(Chat {
            chat,
            cmd_rx,
            ev_tx,
            shutdown: shutdown_tx,
            error: None,
        });

        let _ = responder.send(Ok((client, events)));
    }
}

struct Chat {
    chat: SimpleXChat,
    cmd_rx: CmdReceiver,
    ev_tx: EventTransmitter,
    shutdown: ShutdownEmitter,
    error: Option<Arc<CallError>>,
}

impl Chat {
    fn handle_buffered_actions(&mut self) -> Status {
        let mut status = Status::Skipped;

        for _ in 0..default::MAX_EVENTS_PER_ITER {
            match self.chat.try_recv_msg() {
                Ok(event) if event.is_empty() => break,
                Ok(event) => {
                    status = Status::Executed;
                    let _ = self.ev_tx.send(Ok(event));
                }
                Err(err) => {
                    let err = Arc::new(err);
                    self.error = Some(err.clone());
                    let _ = self.ev_tx.send(Err(err));

                    return Status::Terminated;
                }
            }
        }

        for _ in 0..default::MAX_CMDS_PER_ITER {
            match self.cmd_rx.try_recv() {
                Ok(ChatCommand::Execute(cmd, responder)) => {
                    status = Status::Executed;
                    let output = self.chat.send_cmd(cmd);
                    let _ = responder.send(output.map_err(Arc::new));
                }
                Ok(ChatCommand::Disconnect) | Err(TryRecvError::Disconnected) => {
                    return Status::Terminated;
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
            }
        }

        status
    }

    fn close(mut self) {
        let _ = self.chat.send_cmd("/_stop".to_owned());

        while let Ok(cmd) = self.cmd_rx.try_recv() {
            if let ChatCommand::Execute(_, responder) = cmd {
                let _ = responder.send(Err(self
                    .error
                    .clone()
                    .unwrap_or_else(|| Arc::new(CallError::Failure))));
            }
        }

        drop(self.cmd_rx);

        // Give the Haskell sometime to submit the last events
        //
        // TODO: investigate waiting for a "chatStopped" event instead. This would be a reliable
        // drain boundary, but it also extends the time the worker thread is unavailable to other
        // chat instances. Research perfomanc costs
        std::thread::yield_now();

        loop {
            match self.chat.try_recv_msg() {
                Ok(event) => {
                    if event.is_empty() {
                        break;
                    }

                    let _ = self.ev_tx.send(Ok(event));
                }
                Err(e) => {
                    let _ = self.ev_tx.send(Err(Arc::new(e)));
                    break;
                }
            }
        }

        // Calls chat_close_store in the destructor
        drop(self.chat);
        let _ = self.shutdown.send(true);
    }
}

/// A helper that ensures correct intialization of the underlying SimpleX runtime
fn simplex_chat_init(default_user: DefaultUser, db_opts: DbOpts) -> Result<SimpleXChat, InitError> {
    if db_opts.prefix.len() > default::MAX_DB_PREFIX_LEN
        || default_user.display_name.len() > default::MAX_DISPLAY_NAME_LEN
    {
        return Err(InitError::CallError(CallError::Failure));
    }

    let mut chat = SimpleXChat::init(
        db_opts.prefix,
        db_opts.key.unwrap_or_default(),
        db_opts.migration,
    )?;

    let output = chat.send_cmd("/users".to_owned())?;

    // To start chat successfully SimpleX instance must have one existing user
    if output.contains("\"users\":[]") {
        let subject = if default_user.is_bot { "bot" } else { "user" };
        let output = chat.send_cmd(format!("/create {subject} '{}'", default_user.display_name))?;

        if !output.contains("activeUser") {
            let json = serde_json::from_str(&output).map_err(CallError::InvalidJson)?;
            return Err(InitError::DbError(json));
        }
    }

    let output = chat.send_cmd("/_start".to_owned())?;

    if !output.contains("chatStarted") {
        let json = serde_json::from_str(&output).map_err(CallError::InvalidJson)?;
        return Err(InitError::DbError(json));
    }

    Ok(chat)
}

struct RoundRobin<T> {
    order: Vec<T>,
    pos: usize,
}

impl<T> RoundRobin<T> {
    fn with_capacity(cap: usize) -> Self {
        Self {
            order: Vec::with_capacity(cap),
            pos: 0,
        }
    }

    fn set(&mut self, new_order: impl IntoIterator<Item = T>) {
        self.order.clear();
        self.order.extend(new_order);
        self.pos = self.pos.checked_rem(self.order.len()).unwrap_or(0);
    }

    fn advance(&mut self) {
        self.pos = (self.pos + 1).checked_rem(self.order.len()).unwrap_or(0);
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.order[self.pos..].iter().chain(&self.order[..self.pos])
    }
}

enum WorkerCommand {
    NewChat(Box<NewChatParams>),
    // Interrupts the backoff sleep so a newly queued command is processed without waiting for the
    // next natural wake-up.
    //
    // TODO: Consider splitting into a separate ZST channel to optimize memory utilization
    Wakeup,
}

struct NewChatParams {
    worker: Worker,
    db_opts: DbOpts,
    default_user: DefaultUser,
    responder: NewChatResponder,
}

#[derive(Clone, Copy)]
enum Status {
    Skipped,
    Executed,
    Terminated,
}

impl Status {
    fn is_terminated(self) -> bool {
        matches!(self, Self::Terminated)
    }

    fn not_skipped(self) -> bool {
        !matches!(self, Self::Skipped)
    }
}
