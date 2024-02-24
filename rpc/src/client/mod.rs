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
use futures_util::select_biased;
pub use interface::{Interface, Notification};
use protocol::ProtocolHandler;
pub use protocol::{BorshProtocol, JsonProtocol};
use std::fmt::Debug;
use std::str::FromStr;
use workflow_core::{channel::Multiplexer, task::yield_now};
pub use workflow_websocket::client::{
    ConnectOptions, ConnectResult, ConnectStrategy, Resolver, ResolverResult, WebSocketConfig,
    WebSocketError,
};

#[cfg(feature = "wasm32-sdk")]
pub use workflow_websocket::client::options::IConnectOptions;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Ctl {
    Open,
    Close,
}

impl std::fmt::Display for Ctl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ctl::Open => write!(f, "open"),
            Ctl::Close => write!(f, "close"),
        }
    }
}

impl FromStr for Ctl {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "open" => Ok(Ctl::Open),
            "close" => Ok(Ctl::Close),
            _ => Err(Error::InvalidEvent(s.to_string())),
        }
    }
}

#[async_trait]
pub trait NotificationHandler: Send + Sync + 'static {
    async fn handle_notification(&self, data: &[u8]) -> Result<()>;
}

#[derive(Default)]
pub struct Options<'url> {
    pub ctl_multiplexer: Option<Multiplexer<Ctl>>,
    pub url: Option<&'url str>,
}

impl<'url> Options<'url> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_url(mut self, url: &'url str) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_ctl_multiplexer(mut self, ctl_multiplexer: Multiplexer<Ctl>) -> Self {
        self.ctl_multiplexer = Some(ctl_multiplexer);
        self
    }
}

struct Inner<Ops> {
    ws: Arc<WebSocket>,
    is_running: AtomicBool,
    is_open: AtomicBool,
    receiver_is_running: AtomicBool,
    timeout_is_running: AtomicBool,
    receiver_shutdown: DuplexChannel,
    timeout_shutdown: DuplexChannel,
    timeout_timer_interval: AtomicU64,
    timeout_duration: AtomicU64,
    ctl_multiplexer: Option<Multiplexer<Ctl>>,
    protocol: Arc<dyn ProtocolHandler<Ops>>,
}

impl<Ops> Inner<Ops>
where
    Ops: OpsT,
{
    fn new<T>(
        ws: Arc<WebSocket>,
        protocol: Arc<dyn ProtocolHandler<Ops>>,
        options: Options,
    ) -> Result<Self>
    where
        T: ProtocolHandler<Ops> + Send + Sync + 'static,
    {
        let inner = Inner {
            ws,
            is_running: AtomicBool::new(false),
            is_open: AtomicBool::new(false),
            receiver_is_running: AtomicBool::new(false),
            receiver_shutdown: DuplexChannel::oneshot(),
            timeout_is_running: AtomicBool::new(false),
            timeout_shutdown: DuplexChannel::oneshot(),
            timeout_duration: AtomicU64::new(60_000),
            timeout_timer_interval: AtomicU64::new(5_000),
            ctl_multiplexer: options.ctl_multiplexer,
            protocol,
        };

        Ok(inner)
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn start(self: &Arc<Self>) -> Result<()> {
        if !self.is_running.load(Ordering::Relaxed) {
            self.is_running.store(true, Ordering::SeqCst);
            self.clone().timeout_task();
            self.clone().receiver_task();
        } else {
            log_warn!("wRPC services are already running: rpc::start() was called multiple times");
        }
        Ok(())
    }

    pub async fn shutdown(self: &Arc<Self>) -> Result<()> {
        self.ws.disconnect().await?;
        yield_now().await;
        self.stop_timeout().await?;
        self.stop_receiver().await?;
        self.is_running.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn timeout_task(self: Arc<Self>) {
        self.timeout_is_running.store(true, Ordering::SeqCst);
        workflow_core::task::spawn(async move {
            'outer: loop {
                let timeout_timer_interval =
                    Duration::from_millis(self.timeout_timer_interval.load(Ordering::SeqCst));
                select_biased! {
                    _ = workflow_core::task::sleep(timeout_timer_interval).fuse() => {
                        let timeout = Duration::from_millis(self.timeout_duration.load(Ordering::Relaxed));
                        self.protocol.handle_timeout(timeout).await;
                    },
                    _ = self.timeout_shutdown.request.receiver.recv().fuse() => {
                        break 'outer;
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
            'outer: loop {
                select_biased! {
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
                                        if let Some(ctl_channel) = &self.ctl_multiplexer {
                                            ctl_channel.try_broadcast(Ctl::Open).expect("ctl_channel.try_broadcast(Ctl::Open)");
                                        }
                                    }
                                    WebSocketMessage::Close => {
                                        self.is_open.store(false, Ordering::SeqCst);

                                        self.protocol.handle_disconnect().await.unwrap_or_else(|err|{
                                            log_error!("wRPC error during protocol disconnect: {err}");
                                        });

                                        if let Some(ctl_channel) = &self.ctl_multiplexer {
                                            ctl_channel.try_broadcast(Ctl::Close).expect("ctl_channel.try_broadcast(Ctl::Close)");
                                        }
                                    }
                                }
                            },
                            Err(err) => {
                                log_error!("wRPC client receiver channel error: {err}");
                                break 'outer;
                            }
                        }
                    },
                    _ = self.receiver_shutdown.request.receiver.recv().fuse() => {
                        break 'outer;
                    },

                }
            }

            self.receiver_is_running.store(false, Ordering::SeqCst);
            self.receiver_shutdown.response.sender.send(()).await.unwrap_or_else(|err|
                log_error!("wRPC client - unable to signal shutdown completion for receiver task: `{err}`")
            );
        });
    }

    async fn stop_receiver(&self) -> Result<()> {
        if !self.receiver_is_running.load(Ordering::SeqCst) {
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
    Json(Arc<JsonProtocol<Ops, Id>>),
}

impl<Ops, Id> From<Arc<dyn ProtocolHandler<Ops>>> for Protocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    fn from(protocol: Arc<dyn ProtocolHandler<Ops>>) -> Self {
        if let Ok(protocol) = protocol.clone().downcast_arc::<BorshProtocol<Ops, Id>>() {
            Protocol::Borsh(protocol)
        } else if let Ok(protocol) = protocol.clone().downcast_arc::<JsonProtocol<Ops, Id>>() {
            Protocol::Json(protocol)
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
        options: Options,
        config: Option<WebSocketConfig>,
    ) -> Result<RpcClient<Ops, Id>> {
        match encoding {
            Encoding::Borsh => Self::new::<BorshProtocol<Ops, Id>>(interface, options, config),
            Encoding::SerdeJson => Self::new::<JsonProtocol<Ops, Id>>(interface, options, config),
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
    /// - [`JsonProtocol`]
    ///
    ///
    pub fn new<T>(
        interface: Option<Arc<Interface<Ops>>>,
        options: Options,
        config: Option<WebSocketConfig>,
    ) -> Result<RpcClient<Ops, Id>>
    where
        T: ProtocolHandler<Ops> + Send + Sync + 'static,
    {
        let url = options.url.map(sanitize_url).transpose()?;

        let ws = Arc::new(WebSocket::new(url.as_deref(), config)?);
        let protocol: Arc<dyn ProtocolHandler<Ops>> = Arc::new(T::new(ws.clone(), interface));
        let inner = Arc::new(Inner::new::<T>(ws, protocol.clone(), options)?);

        let client = RpcClient::<Ops, Id> {
            inner,
            protocol: protocol.into(),
            ops: PhantomData,
            id: PhantomData,
        };

        Ok(client)
    }

    /// Connect to the target wRPC endpoint (websocket address)
    pub async fn connect(&self, options: ConnectOptions) -> ConnectResult<Error> {
        if !self.inner.is_running() {
            self.inner.start()?;
        }
        Ok(self.inner.ws.connect(options).await?)
    }

    /// Stop wRPC client services
    pub async fn shutdown(&self) -> Result<()> {
        self.inner.shutdown().await?;
        Ok(())
    }

    pub fn ctl_multiplexer(&self) -> &Option<Multiplexer<Ctl>> {
        &self.inner.ctl_multiplexer
    }

    /// Test if the underlying WebSocket is currently open
    pub fn is_open(&self) -> bool {
        self.inner.ws.is_open()
    }

    /// Obtain the current URL of the underlying WebSocket
    pub fn url(&self) -> Option<String> {
        self.inner.ws.url()
    }

    /// Change the URL of the underlying WebSocket
    /// (applicable only to the next connection).
    /// Alternatively, the new URL can be supplied
    /// in the `connect()` method using [`ConnectOptions`].
    pub fn set_url(&self, url: &str) -> Result<()> {
        self.inner.ws.set_url(url);
        Ok(())
    }

    /// Change the configuration of the underlying WebSocket.
    /// This method can be used to alter the configuration
    /// for the next connection.
    pub fn configure(&self, config: WebSocketConfig) {
        self.inner.ws.configure(config);
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
            Protocol::Json(protocol) => {
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
            Protocol::Json(protocol) => Ok(protocol.request(op, req).await?),
        }
    }

    /// Triggers a disconnection on the underlying WebSocket.
    /// This is intended for debug purposes only.
    /// Can be used to test application reconnection logic.
    pub fn trigger_abort(&self) -> Result<()> {
        Ok(self.inner.ws.trigger_abort()?)
    }
}

fn sanitize_url(url: &str) -> Result<String> {
    let url = url
        .replace("wrpc://", "ws://")
        .replace("wrpcs://", "wss://");
    Ok(url)
}
