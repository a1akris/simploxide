use std::sync::Arc;

use dashmap::DashMap;
use serde::Deserialize;
use simploxide_api_types::{
    client_api::ClientApi,
    commands::ReceiveFile,
    events::{Event, EventKind, RcvFileComplete, RcvFileError, RcvFileSndCancelled},
    responses::{CancelFileResponse, RcvFileAcceptedSndCancelledResponse, ReceiveFileResponse},
};

use crate::{Hook, id::FileId};

type XftpDownloadResponder = tokio::sync::oneshot::Sender<XftpManagerDownloadResponse>;

pub trait XftpExt: ClientApi {
    fn download_file<FID: Into<FileId>>(&self, file_id: FID) -> DownloadFileBuilder<'_, Self>;
}

#[derive(Clone)]
pub struct XftpClient<C> {
    client: C,
    xftp: Arc<XftpManager>,
}

#[cfg(feature = "websocket")]
impl XftpClient<crate::ws::Client> {
    pub fn disconnect(self) -> impl Future<Output = ()> {
        self.client.disconnect()
    }
}

#[cfg(feature = "ffi")]
impl XftpClient<crate::ffi::Client> {
    pub fn disconnect(self) -> impl Future<Output = ()> {
        self.client.disconnect()
    }
}

impl<C: ClientApi> ClientApi for XftpClient<C> {
    type ResponseShape<'de, T: 'de + Deserialize<'de>> = C::ResponseShape<'de, T>;
    type Error = C::Error;

    fn send_raw(
        &self,
        command: String,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send {
        self.client.send_raw(command)
    }

    fn cancel_file(
        &self,
        file_id: i64,
    ) -> impl Future<Output = Result<CancelFileResponse, Self::Error>> + Send {
        self.xftp.downloads.remove(&file_id);
        self.client.cancel_file(file_id)
    }
}

impl<C: ClientApi> From<C> for XftpClient<C> {
    fn from(client: C) -> Self {
        Self {
            client,
            xftp: Arc::new(XftpManager::default()),
        }
    }
}

impl<C: ClientApi> XftpExt for XftpClient<C> {
    fn download_file<FID: Into<FileId>>(&self, file_id: FID) -> DownloadFileBuilder<'_, Self> {
        DownloadFileBuilder {
            client: self,
            cmd: ReceiveFile::new(file_id.into().0),
        }
    }
}

impl<C: ClientApi> Hook for XftpClient<C> {
    fn should_intercept(&self, kind: EventKind) -> bool {
        const EVENT_KINDS: [EventKind; 3] = [
            EventKind::RcvFileSndCancelled,
            EventKind::RcvFileComplete,
            EventKind::RcvFileError,
        ];

        EVENT_KINDS.contains(&kind)
    }

    fn intercept_event(&mut self, event: Event) {
        match event {
            Event::RcvFileComplete(ev) => {
                if let Some((_, responder)) = self
                    .xftp
                    .downloads
                    .remove(&ev.chat_item.chat_item.file.as_ref().unwrap().file_id)
                {
                    let _ = responder.send(XftpManagerDownloadResponse::Complete(ev));
                }
            }
            Event::RcvFileSndCancelled(ev) => {
                if let Some((_, responder)) =
                    self.xftp.downloads.remove(&ev.rcv_file_transfer.file_id)
                {
                    let _ = responder.send(XftpManagerDownloadResponse::Cancelled(ev));
                }
            }
            Event::RcvFileError(ev) => {
                if let Some((_, responder)) =
                    self.xftp.downloads.remove(&ev.rcv_file_transfer.file_id)
                {
                    let _ = responder.send(XftpManagerDownloadResponse::Error(ev));
                }
            }
            _ => (),
        }
    }
}

pub struct DownloadFileBuilder<'a, C: 'a + ?Sized> {
    client: &'a C,
    cmd: ReceiveFile,
}

impl<'a, C: 'a + ?Sized> DownloadFileBuilder<'a, C> {
    pub fn via_user_approved_relays(mut self) -> Self {
        self.cmd.user_approved_relays = true;
        self
    }

    pub fn store_encrypted(mut self) -> Self {
        self.cmd.store_encrypted = Some(true);
        self
    }

    pub fn inline(mut self) -> Self {
        self.cmd.file_inline = Some(true);
        self
    }

    pub fn file_path<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.cmd.file_path = Some(path.as_ref().display().to_string());
        self
    }
}

impl<'a, C: 'a + ClientApi> IntoFuture for DownloadFileBuilder<'a, XftpClient<C>>
where
    <XftpClient<C> as ClientApi>::Error: 'static + Send,
{
    type Output = Result<Arc<RcvFileComplete>, DownloadError<<XftpClient<C> as ClientApi>::Error>>;
    type IntoFuture = std::pin::Pin<Box<dyn 'a + Send + Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let file_id = self.cmd.file_id;

            let (responder, response) = tokio::sync::oneshot::channel();
            self.client.xftp.downloads.insert(file_id, responder);

            match self.client.receive_file(self.cmd).await {
                Ok(ReceiveFileResponse::RcvFileAccepted(_)) => {
                    match response.await.expect("XFTP responses are always delivered") {
                        XftpManagerDownloadResponse::Complete(success) => Ok(success),
                        XftpManagerDownloadResponse::Cancelled(err) => {
                            Err(DownloadError::SendCancelled(err))
                        }
                        XftpManagerDownloadResponse::Error(err) => Err(DownloadError::Receive(err)),
                    }
                }
                Ok(ReceiveFileResponse::RcvFileAcceptedSndCancelled(err)) => {
                    self.client.xftp.downloads.remove(&file_id);
                    Err(DownloadError::AcceptCancelled(err))
                }
                Err(e) => {
                    self.client.xftp.downloads.remove(&file_id);
                    Err(DownloadError::Api(e))
                }
            }
        })
    }
}

pub enum DownloadError<E> {
    SendCancelled(Arc<RcvFileSndCancelled>),
    AcceptCancelled(Arc<RcvFileAcceptedSndCancelledResponse>),
    Receive(Arc<RcvFileError>),
    Api(E),
}

impl<E> std::fmt::Debug for DownloadError<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendCancelled(arg) => f
                .debug_tuple("SendCancelled")
                .field(&arg.rcv_file_transfer.file_id)
                .finish(),
            Self::AcceptCancelled(arg) => f
                .debug_tuple("AcceptCancelled")
                .field(&arg.rcv_file_transfer.file_id)
                .finish(),
            Self::Receive(arg) => f.debug_tuple("Receive").field(&arg.agent_error).finish(),
            Self::Api(e) => f.debug_tuple("Api").field(e).finish(),
        }
    }
}

impl<E> std::fmt::Display for DownloadError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendCancelled(err) => write!(
                f,
                "File(ID={}) was cancelled by the user",
                err.rcv_file_transfer.file_id
            ),
            Self::AcceptCancelled(err) => write!(
                f,
                "File(ID={}) is no longer available",
                err.rcv_file_transfer.file_id
            ),
            Self::Receive(err) => write!(
                f,
                "File(ID={}) receive error: {:?}",
                err.rcv_file_transfer.file_id, err.agent_error
            ),
            Self::Api(err) => write!(f, "{err}"),
        }
    }
}

impl<E> std::error::Error for DownloadError<E>
where
    E: 'static + std::error::Error,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SendCancelled(_) => None,
            Self::AcceptCancelled(_) => None,
            Self::Receive(_) => None,
            Self::Api(error) => Some(error),
        }
    }
}

#[derive(Default)]
struct XftpManager {
    downloads: DashMap<i64, XftpDownloadResponder>,
}

enum XftpManagerDownloadResponse {
    Complete(Arc<RcvFileComplete>),
    Error(Arc<RcvFileError>),
    Cancelled(Arc<RcvFileSndCancelled>),
}
