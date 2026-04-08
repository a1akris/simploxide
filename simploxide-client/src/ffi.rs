pub use simploxide_ffi_core::{CallError, DbOpts, DefaultUser, InitError};

use simploxide_api_types::{
    client_api::{ExtractResponse as _, FfiResponseShape},
    events::{Event, EventKind},
};
use simploxide_ffi_core::{Event as CoreEvent, RawClient, Result as CoreResult};

use std::sync::Arc;

use crate::{BadResponseError, ClientApi, ClientApiError, EventParser, EventStream};

pub type ClientResult<T = ()> = ::std::result::Result<T, ClientError>;

pub async fn init(
    default_user: DefaultUser,
    db_opts: DbOpts,
) -> Result<(Client, EventStream<CoreResult<CoreEvent>>), InitError> {
    let (raw_client, raw_event_queue) = simploxide_ffi_core::init(default_user, db_opts).await?;
    Ok((
        Client::from(raw_client),
        EventStream::from(raw_event_queue.into_receiver()),
    ))
}

/// A cheaply clonable high-level FFI client implementing [`ClientApi`]
#[derive(Clone)]
pub struct Client {
    inner: RawClient,
}

impl From<RawClient> for Client {
    fn from(inner: RawClient) -> Self {
        Self { inner }
    }
}

/// A high level SimpleX-Chat client which provides typed API methods with automatic command
/// serialization and response deserialization.
impl Client {
    /// Initiates a graceful shutdown for the underlying web socket connection. See
    /// [`simploxide_core::RawClient::disconnect`] for details.
    pub fn disconnect(self) {
        self.inner.disconnect();
    }
}

impl ClientApi for Client {
    type ResponseShape<'de, T>
        = FfiResponseShape<T>
    where
        T: 'de + serde::Deserialize<'de>;

    type Error = ClientError;

    async fn send_raw(&self, command: String) -> Result<String, Self::Error> {
        self.inner
            .send(command)
            .await
            .map_err(ClientError::FfiFailure)
    }
}

impl EventParser for CoreResult<String> {
    type Error = ClientError;

    fn parse_kind(&self) -> Result<EventKind, Self::Error> {
        #[derive(serde::Deserialize)]
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

fn parse_data<'de, 'r: 'de, D: 'de + serde::Deserialize<'de>>(
    result: &'r CoreResult<String>,
) -> Result<D, ClientError> {
    result
        .as_ref()
        .map_err(|e| ClientError::FfiFailure(e.clone()))
        .and_then(|ev| {
            serde_json::from_str::<FfiResponseShape<D>>(ev)
                .map_err(BadResponseError::InvalidJson)
                .and_then(|shape| shape.extract_response())
                .map_err(ClientError::BadResponse)
        })
}

/// See [`crate::client_api::AllowUndocumentedResponses`] if you don't want to trigger an error
/// when you receive undocumeted responses(you usually receive undocumented responses when your
/// simplex-chat version is not compatible with the current simploxide-client version. Keep an eye
/// on the [Version compatability table](https://github.com/a1akris/simploxide?tab=readme-ov-file#version-compatability-table))
#[derive(Debug)]
pub enum ClientError {
    FfiFailure(Arc<CallError>),
    BadResponse(BadResponseError),
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FfiFailure(error) => Some(error),
            Self::BadResponse(error) => Some(error),
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::FfiFailure(err) => writeln!(f, "FFI error: {err}"),
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
