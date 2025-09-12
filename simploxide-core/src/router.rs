//! Request registration and response routing task

use crate::transmission;

use super::{Error, RequestId, Response, Result};

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite;
use tokio_util::sync::CancellationToken;

pub type Responder = oneshot::Sender<Result<Response>>;

type CommandSender = mpsc::UnboundedSender<ClientCommand>;
type CommandReceiver = mpsc::UnboundedReceiver<ClientCommand>;

type ResponseSender = mpsc::UnboundedSender<DeliveryCommand>;
type ResponseReceiver = mpsc::UnboundedReceiver<DeliveryCommand>;

/// Splitting a router into a client part and a dispatcher(ResponseRouter) part is necessarry to
/// handle graceful shutdown correctly. With a single channel it would be impossible to block
/// clients from booking new request `corrIds` after shutdown is initiated because the same channel
/// is used to receive responses from the dispatcher task.
///
/// The split allows to receive responses but stop receiving new booking requests.
pub fn init(
    dispatching_cancellator: CancellationToken,
    transmission_interrupter: transmission::Interrupter,
) -> (ClientRouter, ResponseRouter) {
    let (client_sender, client_receiver) = mpsc::unbounded_channel();
    let (response_sender, response_receiver) = mpsc::unbounded_channel();

    tokio::spawn(routing_task(
        client_receiver,
        response_receiver,
        dispatching_cancellator,
        transmission_interrupter,
    ));

    (
        ClientRouter {
            sender: client_sender,
        },
        ResponseRouter {
            sender: response_sender,
        },
    )
}

#[derive(Clone)]
pub struct ClientRouter {
    sender: CommandSender,
}

impl ClientRouter {
    /// Returns the [`tungstenite::Error::AlreadyClosed`] after shutdown was called or when
    /// connection was lost due to an internal WS error.
    pub fn book(&self, id: RequestId, responder: Responder) -> Result {
        self.sender
            .send(ClientCommand::Book { id, responder })
            .map_err(|_| Arc::new(tungstenite::Error::AlreadyClosed))
    }

    /// Initiates graceful shutdown
    pub fn shutdown(self) {
        // Ignoring the error because the task could be already shut down
        let _ = self.sender.send(ClientCommand::Shutdown);
    }
}

#[derive(Clone)]
pub struct ResponseRouter {
    sender: ResponseSender,
}

impl ResponseRouter {
    /// Delivers a response to the awaiting future. Must always succeed.
    pub fn deliver(&self, id: RequestId, response: Response) {
        self.sender
            .send(DeliveryCommand::Deliver { id, response })
            .expect("Routing task exists while there are responses to deliver")
    }

    /// Initiates emergency shutdown due to internal [`tungstenite::Error`]
    pub fn shutdown(self, err: Error) {
        self.sender
            .send(DeliveryCommand::Shutdown(err))
            .expect("Delivery error must be received by the router in any circumstances");
    }
}

async fn routing_task(
    mut client_commands: CommandReceiver,
    mut responses: ResponseReceiver,
    dispatching_cancellator: CancellationToken,
    transmission_interrupter: transmission::Interrupter,
) {
    let mut router = InnerRouter::new();

    let internal_error = normal_operation(&mut router, &mut client_commands, &mut responses).await;
    let _ = transmission_interrupter.send(internal_error.clone());

    if let Some(err) = internal_error {
        error_handler(router, client_commands, responses, err).await;
        return;
    }

    if let Err(err) = graceful_shutdown(&mut router, &mut client_commands, &mut responses).await {
        error_handler(router, client_commands, responses, err).await;
    } else {
        dispatching_cancellator.cancel();
    }

    log::debug!("Router task finished");
}

/// Deliver responses to the awaiting futures
async fn normal_operation(
    router: &mut InnerRouter,
    client_commands: &mut CommandReceiver,
    responses: &mut ResponseReceiver,
) -> Option<Error> {
    loop {
        tokio::select! {
            // Biased is required to avoid receiving responses before booking for them succeeds.
            //
            // This situation is theoretically possible under a heavy load when select! macro
            // chooses to poll the `response` branch more frequently than the `cmd` branch.
            // `biased;` prevents this by guaranteeing that the `cmd` branch is always polled
            // first.
            biased;

            cmd = client_commands.recv() => {
                match cmd {
                    // Register corrId for a response
                    Some(ClientCommand::Book {id, responder }) => { router.book(id, responder); }
                    // "disconnect()" explicitly called
                    Some(ClientCommand::Shutdown) => {
                        client_commands.close();
                        break None;
                    }
                    // All clients were dropped so the client_commands channel must be closed at
                    // this point
                    None => {
                        assert!(client_commands.is_closed());
                        break None;
                    }
                }
            }

            response = responses.recv() => {
                match response {
                    // Deliver response by corrId
                    Some(DeliveryCommand::Deliver { id, response }) =>  { router.deliver(id, Ok(response)); }
                    // WS error
                    Some(DeliveryCommand::Shutdown(err)) =>  {
                        client_commands.close();
                        break Some(err);
                    }
                    None => unreachable!("Dispatcher task always sends Shutdown before dropping the channel"),
                }
            }
        }
    }
}

/// Wait for all scheduled futures to receive their responses before dropping the connection.
async fn graceful_shutdown(
    router: &mut InnerRouter,
    client_commands: &mut CommandReceiver,
    responses: &mut ResponseReceiver,
) -> Result {
    while let Some(cmd) = client_commands.recv().await {
        match cmd {
            // All requests after the shutdown signal are considered failed.
            ClientCommand::Book { responder, .. } => {
                let _ = responder.send(Err(Arc::new(tungstenite::Error::AlreadyClosed)));
            }
            ClientCommand::Shutdown => {
                log::warn!(
                    "Ignoring another disconnect() call because the client is already shutting down"
                );
            }
        }
    }

    while !router.table.is_empty() {
        match responses.recv().await {
            // Deliver response by corrId
            Some(DeliveryCommand::Deliver { id, response }) => {
                router.deliver(id, Ok(response));
            }
            // WS_error
            Some(DeliveryCommand::Shutdown(err)) => {
                return Err(err);
            }
            None => {
                unreachable!("Dispatcher task always sends Shutdown before dropping the channel")
            }
        }
    }

    Ok(())
}

/// Distribute internal error between all awaiting futures.
async fn error_handler(
    router: InnerRouter,
    mut client_commands: CommandReceiver,
    mut receiver: ResponseReceiver,
    error: Error,
) {
    receiver.close();

    while let Some(cmd) = client_commands.recv().await {
        match cmd {
            ClientCommand::Book { responder, .. } => {
                let _ = responder.send(Err(Arc::clone(&error)));
            }
            ClientCommand::Shutdown => {
                log::warn!(
                    "Ignoring disconnect() call because the client is already shutting down due to an error"
                );
            }
        }
    }

    for (_, responder) in router.table.into_iter() {
        let _ = responder.send(Err(Arc::clone(&error)));
    }
}

#[derive(Debug)]
enum DeliveryCommand {
    Deliver { id: RequestId, response: Response },
    Shutdown(Error),
}

enum ClientCommand {
    Book { id: RequestId, responder: Responder },
    Shutdown,
}

#[derive(Default)]
struct InnerRouter {
    table: HashMap<RequestId, Responder>,
}

impl InnerRouter {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, id: RequestId, responder: Responder) {
        let prev = self.table.insert(id, responder);
        assert!(prev.is_none(), "Request ID cannot not be duplicated");
    }

    fn deliver(&mut self, id: RequestId, result: Result<Response>) {
        let responder = self
            .table
            .remove(&id)
            .expect("Request ID is booked before sending a request");

        // Not the router's business whether the future awaiting the response got dropped or not
        let _ = responder.send(result);
    }
}
