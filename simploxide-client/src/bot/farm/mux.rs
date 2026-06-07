//! Commands multiplexer
//!
//! The multiplexing is required because most commands need to activate their user to execute
//! correctly. Multiplexing ensures that the user remains active for the whole duration of its
//! commands execution

use futures::{StreamExt as _, stream::FuturesOrdered};
use simploxide_api_types::{client_api::ClientApi, commands::ApiSetActiveUser};

use crate::id::UserId;

use super::{BotId, DelegateReceiver, DelegateRequest};

pub fn start<C: 'static + Send + ClientApi>(client: C, requests: DelegateReceiver<C>)
where
    C::Error: 'static + Send,
{
    tokio::spawn(Box::pin(task(client, requests)));
}

async fn task<C: ClientApi>(client: C, mut requests: DelegateReceiver<C>) {
    let mut active_bot = BotId::anybot();
    let mut batcher = RequestBatcher::with_capacity(64);
    let mut executor = FuturesOrdered::new();

    while let Some(request) = requests.recv().await {
        batcher.push(request);

        while let Ok(request) = requests.try_recv() {
            batcher.push(request);
        }

        if batcher.len() > 64 {
            log::warn!(
                "Batcher is performing unoptimally on size {}. Update the algorithm!",
                batcher.len()
            );
        }

        // Process requests in batches and minimize bot switches
        for request in batcher.drain() {
            if let Some(user_id) = should_switch_bot(active_bot, request.bot_id) {
                while executor.next().await.is_some() {}

                if let Err(e) = try_switch_bot(&client, &mut active_bot, user_id).await {
                    let _ = request.responder.send(Err(e));
                } else {
                    executor.push_back(exec_request(&client, request));
                }
            } else {
                executor.push_back(exec_request(&client, request));
            }
        }

        // Execute requests from the latest batch
        while executor.next().await.is_some() {}
    }
}

async fn exec_request<C: ClientApi>(client: &C, request: DelegateRequest<C>) {
    let result = client.send_raw(request.cmd).await;
    let _ = request.responder.send(result);
}

async fn try_switch_bot<C: ClientApi>(
    client: &C,
    active_bot: &mut BotId,
    next_bot: UserId,
) -> Result<(), C::Error> {
    let result = client
        .api_set_active_user(ApiSetActiveUser::new(next_bot.raw()))
        .await;

    if result.is_ok() {
        *active_bot = next_bot.into();
    }

    result.map(drop)
}

fn should_switch_bot(active_bot: BotId, next_bot: BotId) -> Option<UserId> {
    if next_bot != active_bot {
        next_bot.get()
    } else {
        None
    }
}

struct RequestBatcher<C: ClientApi> {
    requests: Vec<DelegateRequest<C>>,
}

impl<C: ClientApi> RequestBatcher<C> {
    fn with_capacity(cap: usize) -> Self {
        Self {
            requests: Vec::with_capacity(cap),
        }
    }

    fn len(&self) -> usize {
        self.requests.len()
    }

    /// O(n) group-by push. Inserting n requests is O(n) in the best case where all requests
    /// originate from the same bot or O(n^2) in the worst case when there are multiple bots. In
    /// practice the `n` should rarely exceed 10 requests so further optimizations should be
    /// applied only when pathological cases that severly affect perfomance are discovered.
    fn push(&mut self, request: DelegateRequest<C>) {
        let pos = self
            .requests
            .iter()
            .rposition(|batched| batched.bot_id == request.bot_id)
            .map(|pos| pos + 1)
            .unwrap_or(self.requests.len());

        self.requests.insert(pos, request);
    }

    fn drain(&mut self) -> std::vec::Drain<'_, DelegateRequest<C>> {
        self.requests.drain(..)
    }
}
