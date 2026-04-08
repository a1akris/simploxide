pub use simploxide_ws_core::{
    self as core, Error as CoreError, Event as CoreEvent, Result as CoreResult,
    tungstenite::Error as WsError,
};

#[cfg(feature = "cli")]
pub use simploxide_ws_core::cli;

use serde::Deserialize;
use simploxide_api_types::{
    client_api::{ExtractResponse, WebSocketResponseShape, WebSocketResponseShapeInner},
    events::{Event, EventKind},
};
use simploxide_ws_core::RawClient;

use crate::{BadResponseError, ClientApi, ClientApiError, EventParser, EventStream};

pub type ClientResult<T = ()> = ::std::result::Result<T, ClientError>;

/// A wrapper over [`simploxide_core::connect`] that turns [`simploxide_core::RawClient`] into
/// [`Client`] and raw event queue into the [`EventStream`] which handle serialization/deserialization.
///
/// ```ignore
/// let (client, mut events) = simploxide_client::ws::connect("ws://127.0.0.1:5225").await?;
///
/// let current_user = client.api_show_active_user().await?;
/// println!("{current_user:#?}");
///
/// while let Some(ev) = events.try_next().await? {
///     // Process events...
/// }
/// ```
pub async fn connect<S: AsRef<str>>(
    uri: S,
) -> Result<(Client, EventStream<CoreResult<CoreEvent>>), WsError> {
    let (raw_client, raw_event_queue) = simploxide_ws_core::connect(uri.as_ref()).await?;
    Ok((
        Client::from(raw_client),
        EventStream::from(raw_event_queue.into_receiver()),
    ))
}

/// Like [`connect`] but retries to connect `retries_count` times before returning an error. This
/// method is needed when you run simplex-cli programmatically and don't know when WebSocket port
/// becomes available.
///
/// ```ignore
/// let port = 5225;
/// let cli = SimplexCli::spawn(port);
/// let uri = format!("ws://127.0.0.1:{port}");
///
/// let (client, mut events) = simploxide_client::retry_connect(&uri, Duration::from_secs(1), 10).await?;
///
/// //...
///
/// ```
pub async fn retry_connect<S: AsRef<str>>(
    uri: S,
    retry_delay: std::time::Duration,
    mut retries_count: usize,
) -> Result<(Client, EventStream<CoreResult<CoreEvent>>), WsError> {
    loop {
        match connect(uri.as_ref()).await {
            Ok(connection) => break Ok(connection),
            Err(e) if retries_count == 0 => break Err(e),
            Err(_) => {
                retries_count -= 1;
                tokio::time::sleep(retry_delay).await
            }
        }
    }
}

impl EventParser for CoreResult<String> {
    type Error = ClientError;

    fn parse_kind(&self) -> Result<EventKind, Self::Error> {
        #[derive(Deserialize)]
        struct TypeField<'a> {
            #[serde(rename = "type", borrow)]
            typ: &'a str,
        }

        match parse_data::<TypeField<'_>>(self) {
            Ok(f) => Ok(EventKind::from_type_str(f.typ)),
            Err(ClientError::BadResponse(BadResponseError::Undocumented(_))) => {
                Ok(EventKind::Undocumented)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_event(&self) -> Result<Event, Self::Error> {
        parse_data(self)
    }
}

fn parse_data<'de, 'r: 'de, D: 'de + Deserialize<'de>>(
    res: &'r CoreResult<String>,
) -> ClientResult<D> {
    res.as_ref()
        .map_err(|e| ClientError::WebSocketFailure(e.clone()))
        .and_then(|ev| {
            serde_json::from_str::<EventShape<D>>(ev)
                .map_err(BadResponseError::InvalidJson)
                .and_then(|shape| shape.extract_response())
                .map_err(ClientError::BadResponse)
        })
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum EventShape<T> {
    ResponseShape(WebSocketResponseShape<T>),
    InlineShape(WebSocketResponseShapeInner<T>),
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for EventShape<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        match self {
            Self::ResponseShape(resp) => resp.extract_response(),
            Self::InlineShape(inline) => inline.extract_response(),
        }
    }
}

/// A high level SimpleX-Chat client which provides typed API methods with automatic command
/// serialization and response deserialization.
#[derive(Clone)]
pub struct Client {
    inner: RawClient,
}

impl From<RawClient> for Client {
    fn from(inner: RawClient) -> Self {
        Self { inner }
    }
}

impl Client {
    /// Initiates a graceful shutdown for the underlying web socket connection. See
    /// [`simploxide_core::RawClient::disconnect`] for details.
    pub fn disconnect(self) {
        self.inner.disconnect();
    }
}

impl ClientApi for Client {
    type ResponseShape<'de, T>
        = WebSocketResponseShape<T>
    where
        T: 'de + Deserialize<'de>;

    type Error = ClientError;

    async fn send_raw(&self, command: String) -> Result<String, Self::Error> {
        self.inner
            .send(command)
            .await
            .map_err(ClientError::WebSocketFailure)
    }
}

/// See [`crate::client_api::AllowUndocumentedResponses`] if you don't want to trigger an error when
/// you receive undocumeted responses(you usually receive undocumented responses when your
/// simplex-chat server version is not compatible with the simploxide-client version. Keep an eye
/// on the
/// [Version compatability table](https://github.com/a1akris/simploxide?tab=readme-ov-file#version-compatability-table)
/// )
#[derive(Debug)]
pub enum ClientError {
    /// Critical error signalling that the web socket connection is dropped for some reason. You
    /// will have to reconnect to the SimpleX server to recover from this one.
    WebSocketFailure(CoreError),
    /// SimpleX command error or unexpected(undocumented) response.
    BadResponse(BadResponseError),
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WebSocketFailure(error) => Some(error),
            Self::BadResponse(error) => Some(error),
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::WebSocketFailure(err) => writeln!(f, "Web socket failure: {err}"),
            ClientError::BadResponse(err) => err.fmt(f),
        }
    }
}

impl From<BadResponseError> for ClientError {
    fn from(err: BadResponseError) -> Self {
        Self::BadResponse(err)
    }
}

impl ClientApiError for ClientError {
    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError> {
        if let Self::BadResponse(resp) = self {
            Some(resp)
        } else {
            None
        }
    }
}
