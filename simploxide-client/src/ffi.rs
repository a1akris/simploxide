use futures::Stream;
use simploxide_api_types::{
    client_api::{ExtractResponse as _, FfiResponseShape},
    events::Event,
};
pub use simploxide_ffi_core::{CallError, DbOpts, DefaultUser, InitError};

use simploxide_ffi_core::{EventReceiver, RawClient, RawEventQueue};

use std::task;

use crate::{BadResponseError, ClientApi, ClientApiError};

pub type ClientResult<T = ()> = ::std::result::Result<T, ClientError>;

pub async fn init(
    default_user: DefaultUser,
    db_opts: DbOpts,
) -> Result<(Client, EventStream), InitError> {
    let (raw_client, raw_event_queue) = simploxide_ffi_core::init(default_user, db_opts).await?;
    Ok((Client::new(raw_client), EventStream::from(raw_event_queue)))
}

#[derive(Clone)]
pub struct Client {
    inner: RawClient,
}

impl Client {
    fn new(raw_client: RawClient) -> Self {
        Self { inner: raw_client }
    }
}

impl ClientApi for Client {
    type ResponseShape<T>
        = FfiResponseShape<T>
    where
        T: for<'de> serde::Deserialize<'de>;

    type Error = ClientError;

    async fn send_raw(&self, command: String) -> Result<String, Self::Error> {
        self.inner
            .send(command)
            .await
            .map_err(ClientError::FfiFailure)
    }
}

pub struct EventStream(EventReceiver);

impl From<RawEventQueue> for EventStream {
    fn from(value: RawEventQueue) -> Self {
        Self(value.into_receiver())
    }
}

impl Stream for EventStream {
    type Item = ClientResult<Event>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        self.0.poll_recv(cx).map(|opt| {
            opt.map(|res| {
                res.map_err(ClientError::FfiFailure).and_then(|ev| {
                    serde_json::from_str::<FfiResponseShape<Event>>(&ev)
                        .map_err(BadResponseError::InvalidJson)
                        .and_then(|shape| shape.extract_response())
                        .map_err(ClientError::BadResponse)
                })
            })
        })
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
    FfiFailure(CallError),
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
            ClientError::FfiFailure(err) => writeln!(f, "FFI failure: {err}"),
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

