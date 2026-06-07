//! XFTP file download manager.
//!
//! [`XftpClient`] wraps any [`ClientApi`] client and observes the file rcv events emitted by the
//! SimpleX-Chat. [`DownloadFileBuilder`] (obtained via [`XftpExt::download_file`]) initiates the transfer
//! and awaits those events, returning the outcome directly to the caller.
//!
//! # When to use
//!
//! - **Out-of-handler downloads.** When the decision to download a file is made outside an event
//!   handler (for example, after a user command or a timer), `download_file` provides the result
//!   without requiring custom event routing.
//!
//! - **Keeping download logic in one handler.** Sometimes it may be useful to keep all logic in a
//!   single handler to simplify state management.
//!

use std::sync::Arc;

use serde::Deserialize;
use simploxide_api_types::{
    client_api::ClientApi,
    commands::ReceiveFile,
    events::{Event, EventKind, RcvFileComplete, RcvFileError, RcvFileSndCancelled},
    responses::{CancelFileResponse, RcvFileAcceptedSndCancelledResponse, ReceiveFileResponse},
};

use crate::{Hook, id::FileId};

type FxDashMap<K, V> = dashmap::DashMap<K, V, rustc_hash::FxBuildHasher>;
type XftpDownloadResponder = tokio::sync::oneshot::Sender<XftpManagerDownloadResponse>;

/// Adds [`download_file`](Self::download_file) to any [`ClientApi`].
/// Automatically implemented for [`XftpClient`].
pub trait XftpExt: ClientApi {
    /// Begin downloading `file_id` and return a builder to configure and await the result.
    ///
    /// # Deadlock warning
    ///
    /// `download_file` awaits a completion event that only arrives when the event loop processes
    /// **Awaiting a download inside a sequential handler blocks the event loop**: that event
    /// never arrives, causing a deadlock. Only use `download_file` from a **concurrent** handler
    /// (registered with [`Dispatcher::on`](crate::dispatcher::Dispatcher::on)) or outside the
    /// dispatcher entirely.
    fn download_file<FID: Into<FileId>>(&self, file_id: FID) -> DownloadFileBuilder<'_, Self>;
}

/// A [`ClientApi`] wrapper that intercepts file-result events and routes them to the
/// corresponding [`DownloadFileBuilder`] futures. Should be constructed by
/// [`EventStream::hook_xftp`](crate::EventStream::hook_xftp) to work correctly.
#[derive(Clone)]
pub struct XftpClient<C> {
    client: C,
    xftp: Arc<XftpManager>,
}

#[cfg(feature = "websocket")]
impl XftpClient<crate::ws::Client> {
    pub fn version(
        &self,
    ) -> impl Future<Output = Result<crate::ws::SimplexVersion, crate::ws::VersionError>> {
        self.client.version()
    }

    pub fn disconnect(self) -> impl Future<Output = ()> {
        self.client.disconnect()
    }
}

#[cfg(feature = "ffi")]
impl XftpClient<crate::ffi::Client> {
    pub fn version(
        &self,
    ) -> impl Future<Output = Result<crate::ffi::SimplexVersion, crate::ffi::VersionError>> {
        self.client.version()
    }

    pub fn disconnect(self) -> impl Future<Output = ()> {
        self.client.disconnect()
    }
}

#[cfg(feature = "farm")]
impl<C> XftpClient<C> {
    pub(crate) fn new(client: C, xftp: Arc<XftpManager>) -> Self {
        Self { client, xftp }
    }

    pub(crate) fn manager(&self) -> Arc<XftpManager> {
        self.xftp.clone()
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
            cmd: ReceiveFile::new(file_id.into().raw()),
        }
    }
}

impl<C: 'static + ClientApi + Send> Hook for XftpClient<C> {
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
    /// Route the download through user-approved relays rather than the default ones.
    pub fn via_user_approved_relays(mut self) -> Self {
        self.cmd.user_approved_relays = true;
        self
    }

    /// Store the downloaded file in encrypted form.
    pub fn store_encrypted(mut self) -> Self {
        self.cmd.store_encrypted = Some(true);
        self
    }

    /// Request inline delivery (small files only).
    pub fn inline(mut self) -> Self {
        self.cmd.file_inline = Some(true);
        self
    }

    /// Override the path where the downloaded file will be saved.
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

/// Error returned when a [`DownloadFileBuilder`] future resolves unsuccessfully.
pub enum DownloadError<E> {
    /// The sender cancelled the transfer after the download was accepted.
    SendCancelled(Arc<RcvFileSndCancelled>),
    /// The file was no longer available when the download request arrived.
    AcceptCancelled(Arc<RcvFileAcceptedSndCancelledResponse>),
    /// The SimpleX agent reported an error while receiving the file.
    Receive(Arc<RcvFileError>),
    /// The API call to initiate the download failed.
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
pub(crate) struct XftpManager {
    downloads: FxDashMap<i64, XftpDownloadResponder>,
}

enum XftpManagerDownloadResponse {
    Complete(Arc<RcvFileComplete>),
    Error(Arc<RcvFileError>),
    Cancelled(Arc<RcvFileSndCancelled>),
}
