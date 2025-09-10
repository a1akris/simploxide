use futures::{Stream, TryStreamExt as _};
use simploxide_api_types::{JsonObject, events::Event};
use simploxide_core::RawClient;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub use simploxide_api_types::{
    self as types, client_api::ClientApi, commands, events, responses, utils::CommandSyntax,
};
pub use simploxide_core::{
    self as core, Error as CoreError, Result as CoreResult, tungstenite::Error as WsError,
};

pub mod prelude;

pub async fn connect<S: AsRef<str>>(
    uri: S,
) -> Result<(Client, impl Stream<Item = Result<Event, CoreError>>), WsError> {
    let (inner, raw_queue) = simploxide_core::connect(uri.as_ref()).await?;
    let stream = UnboundedReceiverStream::new(raw_queue.into_receiver());

    Ok((
        Client { inner },
        stream.map_ok(|ev| serde_json::from_value::<Event>(ev).unwrap()),
    ))
}

pub struct Client {
    inner: RawClient,
}

impl Client {
    pub fn disconnect(self) {
        self.inner.disconnect();
    }
}

impl ClientApi for Client {
    type Error = CoreError;

    async fn send_raw(&self, command: String) -> Result<JsonObject, Self::Error> {
        self.inner.send(command).await
    }
}
