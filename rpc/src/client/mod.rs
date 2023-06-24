//!
//! RPC client (operates uniformly in native and WASM-browser environments).
//!

pub mod error;
mod interface;
pub mod prelude;
mod protocol;
pub mod result;
pub use crate::client::error::Error;
pub use crate::client::result::Result;

use crate::imports::*;
pub use interface::{Interface, Notification};
use protocol::ProtocolHandler;
pub use protocol::{BorshProtocol, SerdeJsonProtocol};
use std::fmt::Debug;

///
/// notification!() macro for declaration of RPC notification handlers
///
/// This macro simplifies creation of async notification handler
/// closures supplied to the RPC notification interface. An
/// async notification closure requires to be *Box*ed
/// and its result must be *Pin*ned, resulting in the following
/// syntax:
///
/// ```ignore
///
/// interface.notification(Box::new(Notification::new(|msg: MyMsg|
///     Box::pin(
///         async move {
///             // ...
///             Ok(())
///         }
///     )
/// )))
///
/// ```
///
/// The notification macro adds the required Box and Pin syntax,
/// simplifying the declaration as follows:
///
/// ```ignore
/// interface.notification(notification!(|msg: MyMsg| async move {
///     // ...
///     Ok(())
/// }))
/// ```
///
pub use workflow_rpc_macros::client_notification as notification;

#[derive(Debug, Clone)]
pub enum Ctl {
    Open,
    Close,
}

#[async_trait]
pub trait NotificationHandler: Send + Sync + 'static {
    async fn handle_notification(&self, data: &[u8]) -> Result<()>;
}

#[derive(Default)]
pub struct Options<'url> {
    pub ctl_channel: Option<Channel<Ctl>>,
    pub handshake: Option<Arc<dyn Handshake>>,
    pub url: &'url str,
}

struct Inner<Ops> {
    ws: Arc<WebSocket>,
    is_open: AtomicBool,
    receiver_is_running: AtomicBool,
    timeout_is_running: AtomicBool,
    receiver_shutdown: DuplexChannel,
    timeout_shutdown: DuplexChannel,
    timeout_timer_interval: AtomicU64,
    timeout_duration: AtomicU64,
    ctl_channel: Option<Channel<Ctl>>,
    protocol: Arc<dyn ProtocolHandler<Ops>>,
}

impl<Ops> Inner<Ops>
where
    Ops: OpsT,
{
    fn new<T>(
        ws: Arc<WebSocket>,
        protocol: Arc<dyn ProtocolHandler<Ops>>,
        options: Options<'_>,
    ) -> Result<Self>
    where
        T: ProtocolHandler<Ops> + Send + Sync + 'static,
    {
        let inner = Inner {
            ws,
            is_open: AtomicBool::new(false),
            receiver_is_running: AtomicBool::new(false),
            receiver_shutdown: DuplexChannel::oneshot(),
            timeout_is_running: AtomicBool::new(false),
            timeout_shutdown: DuplexChannel::oneshot(),
            timeout_duration: AtomicU64::new(60_000),
            timeout_timer_interval: AtomicU64::new(5_000),
            ctl_channel: options.ctl_channel,
            protocol,
        };

        Ok(inner)
    }

    pub fn start(self: &Arc<Self>) -> Result<()> {
        self.clone().timeout_task();
        self.clone().receiver_task();
        Ok(())
    }

    pub async fn shutdown(self: &Arc<Self>) -> Result<()> {
        self.stop_timeout().await?;
        self.stop_receiver().await?;
        self.ws.disconnect().await?;
        Ok(())
    }

    fn timeout_task(self: Arc<Self>) {
        self.timeout_is_running.store(true, Ordering::SeqCst);
        workflow_core::task::spawn(async move {
            loop {
                let timeout_timer_interval =
                    Duration::from_millis(self.timeout_timer_interval.load(Ordering::SeqCst));
                select! {
                    _ = self.timeout_shutdown.request.receiver.recv().fuse() => {
                        break;
                    },
                    () = workflow_core::task::sleep(timeout_timer_interval).fuse() => {
                        let timeout = Duration::from_millis(self.timeout_duration.load(Ordering::Relaxed));
                        self.protocol.handle_timeout(timeout).await;
                    },
                }
            }

            self.timeout_is_running.store(false, Ordering::SeqCst);
            self.timeout_shutdown.response.sender.send(()).await.unwrap_or_else(|err|
                log_error!("wRPC client - unable to signal shutdown completion for timeout task: `{err}`"));
        });
    }

    fn receiver_task(self: Arc<Self>) {
        self.receiver_is_running.store(true, Ordering::SeqCst);
        let receiver_rx = self.ws.receiver_rx().clone();
        workflow_core::task::spawn(async move {
            loop {
                select! {
                    _ = self.receiver_shutdown.request.receiver.recv().fuse() => {
                        break;
                    },
                    msg = receiver_rx.recv().fuse() => {
                        match msg {
                            Ok(msg) => {
                                match msg {
                                    WebSocketMessage::Binary(_) | WebSocketMessage::Text(_) => {
                                        self.protocol.handle_message(msg).await
                                        .unwrap_or_else(|err|log_trace!("wRPC error: `{err}`"));
                                    }
                                    WebSocketMessage::Open => {
                                        self.is_open.store(true, Ordering::SeqCst);
                                        if let Some(ctl_channel) = &self.ctl_channel {
                                            ctl_channel.send(Ctl::Open).await.unwrap();
                                        }
                                    }
                                    WebSocketMessage::Close => {
                                        self.is_open.store(false, Ordering::SeqCst);

                                        self.protocol.handle_disconnect().await.unwrap_or_else(|err|{
                                            log_error!("wRPC error during protocol disconnect: {err}");
                                        });

                                        if let Some(ctl_channel) = &self.ctl_channel {
                                            ctl_channel.send(Ctl::Close).await.unwrap();
                                        }
                                    }
                                }
                            },
                            Err(err) => {
                                log_error!("wRPC client receiver channel error: {err}");
                            }
                        }
                    }
                }
            }

            self.receiver_is_running.store(false, Ordering::SeqCst);
            self.receiver_shutdown.response.sender.send(()).await.unwrap_or_else(|err|
                log_error!("wRPC client - unable to signal shutdown completion for receiver task: `{err}`")
            );
        });
    }

    async fn stop_receiver(&self) -> Result<()> {
        if self.receiver_is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.receiver_shutdown
            .signal(())
            .await
            .unwrap_or_else(|err| {
                log_error!("wRPC client unable to signal receiver shutdown: `{err}`")
            });

        Ok(())
    }

    async fn stop_timeout(&self) -> Result<()> {
        if !self.timeout_is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.timeout_shutdown
            .signal(())
            .await
            .unwrap_or_else(|err| {
                log_error!("wRPC client unable to signal timeout shutdown: `{err}`")
            });

        Ok(())
    }
}

#[derive(Clone)]
enum Protocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    Borsh(Arc<BorshProtocol<Ops, Id>>),
    SerdeJson(Arc<SerdeJsonProtocol<Ops, Id>>),
}

impl<Ops, Id> From<Arc<dyn ProtocolHandler<Ops>>> for Protocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    fn from(protocol: Arc<dyn ProtocolHandler<Ops>>) -> Self {
        if let Ok(protocol) = protocol.clone().downcast_arc::<BorshProtocol<Ops, Id>>() {
            Protocol::Borsh(protocol)
        } else if let Ok(protocol) = protocol
            .clone()
            .downcast_arc::<SerdeJsonProtocol<Ops, Id>>()
        {
            Protocol::SerdeJson(protocol)
        } else {
            panic!()
        }
    }
}

#[derive(Clone)]
pub struct RpcClient<Ops, Id = Id64>
where
    Ops: OpsT,
    Id: IdT,
{
    inner: Arc<Inner<Ops>>,
    protocol: Protocol<Ops, Id>,
    ops: PhantomData<Ops>,
    id: PhantomData<Id>,
}

impl<Ops, Id> RpcClient<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    ///
    /// Create new wRPC client connecting to the supplied URL
    ///
    /// This function accepts the [`Encoding`] enum argument denoting the underlying
    /// protocol that will be used by the client. Current variants supported
    /// are:
    ///
    /// - [`Encoding::Borsh`]
    /// - [`Encoding::SerdeJson`]
    ///
    ///
    pub fn new_with_encoding(
        encoding: Encoding,
        interface: Option<Arc<Interface<Ops>>>,
        options: Options<'_>,
    ) -> Result<RpcClient<Ops, Id>> {
        match encoding {
            Encoding::Borsh => Self::new::<BorshProtocol<Ops, Id>>(interface, options),
            Encoding::SerdeJson => Self::new::<SerdeJsonProtocol<Ops, Id>>(interface, options),
        }
    }

    ///
    /// Create new wRPC client connecting to the supplied URL.
    ///
    /// This function accepts a generic denoting the underlying
    /// protocol that will be used by the client. Current protocols
    /// supported are:
    ///
    /// - [`BorshProtocol`]
    /// - [`SerdeJsonProtocol`]
    ///
    ///
    pub fn new<T>(
        interface: Option<Arc<Interface<Ops>>>,
        options: Options<'_>,
    ) -> Result<RpcClient<Ops, Id>>
    where
        T: ProtocolHandler<Ops> + Send + Sync + 'static,
    {
        let ws_options = WebSocketOptions {
            handshake: options.handshake.clone(),
            ..WebSocketOptions::default()
        };

        let url = options.url;
        let url = Regex::new(r"^wrpc://")?.replace(url, "ws://");
        let url = Regex::new(r"^wrpcs://")?.replace(&url, "wss://");

        let ws = Arc::new(WebSocket::new(&url, ws_options)?);
        let protocol: Arc<dyn ProtocolHandler<Ops>> = Arc::new(T::new(ws.clone(), interface));
        let inner = Arc::new(Inner::new::<T>(ws, protocol.clone(), options)?);

        let client = RpcClient::<Ops, Id> {
            inner,
            protocol: protocol.into(),
            ops: PhantomData,
            id: PhantomData,
        };

        client.inner.start()?;

        Ok(client)
    }

    /// Connect to the target wRPC endpoint (websocket address)
    pub async fn connect(&self, block_until_connected: bool) -> Result<Option<Listener>> {
        Ok(self.inner.ws.connect(block_until_connected).await?)
    }

    /// Stop wRPC client services
    pub async fn shutdown(&self) -> Result<()> {
        self.inner.shutdown().await?;
        Ok(())
    }

    /// Test if the underlying WebSocket is currently open
    pub fn is_open(&self) -> bool {
        self.inner.ws.is_open()
    }

    ///
    /// Issue an async Notification to the server (no response is expected)
    ///
    /// Following are the trait requirements on the arguments:
    /// - `Ops`: [`OpsT`]
    /// - `Msg`: [`MsgT`]
    ///
    pub async fn notify<Msg>(&self, op: Ops, payload: Msg) -> Result<()>
    where
        Msg: BorshSerialize + Serialize + Send + Sync + 'static,
    {
        if !self.is_open() {
            return Err(WebSocketError::NotConnected.into());
        }

        match &self.protocol {
            Protocol::Borsh(protocol) => {
                protocol.notify(op, payload).await?;
            }
            Protocol::SerdeJson(protocol) => {
                protocol.notify(op, payload).await?;
            }
        }

        Ok(())
    }

    ///
    /// Issue an async wRPC call and wait for response.
    ///
    /// Following are the trait requirements on the arguments:
    /// - `Ops`: [`OpsT`]
    /// - `Req`: [`MsgT`]
    /// - `Resp`: [`MsgT`]
    ///
    pub async fn call<Req, Resp>(&self, op: Ops, req: Req) -> Result<Resp>
    where
        Req: MsgT,
        Resp: MsgT,
    {
        if !self.is_open() {
            return Err(WebSocketError::NotConnected.into());
        }

        match &self.protocol {
            Protocol::Borsh(protocol) => Ok(protocol.request(op, req).await?),
            Protocol::SerdeJson(protocol) => Ok(protocol.request(op, req).await?),
        }
    }
}
