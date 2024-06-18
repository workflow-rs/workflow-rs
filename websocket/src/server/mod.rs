//!
//! async WebSocket server functionality (requires tokio executor)
//!
use async_trait::async_trait;
use cfg_if::cfg_if;
use downcast_rs::*;
use futures::{future::FutureExt, select};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
pub use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{
    UnboundedReceiver as TokioUnboundedReceiver, UnboundedSender as TokioUnboundedSender,
};
use tokio_tungstenite::{accept_async_with_config, WebSocketStream};
use tungstenite::Error as WebSocketError;
use workflow_core::channel::DuplexChannel;
use workflow_log::*;
pub mod error;
pub mod result;

pub use error::Error;
pub use result::Result;
pub use tungstenite::protocol::WebSocketConfig;
pub use tungstenite::Message;
/// WebSocket stream sender for dispatching [`tungstenite::Message`].
/// This stream object must have a mutable reference and can not be cloned.
pub type WebSocketSender = SplitSink<WebSocketStream<TcpStream>, Message>;
/// WebSocket stream receiver for receiving [`tungstenite::Message`].
/// This stream object must have a mutable reference and can not be cloned.
pub type WebSocketReceiver = SplitStream<WebSocketStream<TcpStream>>;
/// WebSocketSink [`tokio::sync::mpsc::UnboundedSender`] for dispatching
/// messages from within the [`WebSocketHandler::message`]. This is an
/// `MPSC` channel that can be cloned and retained externally for the
/// lifetime of the WebSocket connection.
pub type WebSocketSink = TokioUnboundedSender<Message>;

/// Atomic counters that allow tracking connection counts
/// and cumulative message sizes in bytes (bandwidth consumption
/// without accounting for the websocket framing overhead).
/// These counters can be created and supplied externally or
/// supplied as `None`.
pub struct WebSocketCounters {
    pub total_connections: Arc<AtomicUsize>,
    pub active_connections: Arc<AtomicUsize>,
    pub handshake_failures: Arc<AtomicUsize>,
    pub rx_bytes: Arc<AtomicUsize>,
    pub tx_bytes: Arc<AtomicUsize>,
}

impl Default for WebSocketCounters {
    fn default() -> Self {
        WebSocketCounters {
            total_connections: Arc::new(AtomicUsize::new(0)),
            active_connections: Arc::new(AtomicUsize::new(0)),
            handshake_failures: Arc::new(AtomicUsize::new(0)),
            rx_bytes: Arc::new(AtomicUsize::new(0)),
            tx_bytes: Arc::new(AtomicUsize::new(0)),
        }
    }
}

/// WebSocketHandler trait that represents the WebSocket processor
/// functionality.  This trait is supplied to the WebSocket
/// which subsequently invokes it's functions during websocket
/// connection and messages.  The trait can override `with_handshake()` method
/// to enable invocation of the `handshake()` method upon receipt of the
/// first valid websocket message from the incoming connection.
#[async_trait]
pub trait WebSocketHandler
where
    Arc<Self>: Sync,
{
    /// Context type used by impl trait to represent websocket connection
    type Context: Send + Sync;

    /// Called to determine if the connection should be accepted.
    fn accept(&self, _peer: &SocketAddr) -> bool {
        true
    }

    /// Called immediately when connection is established.
    /// This function should return an error to terminate the connection.
    /// If the server manages a client ban list, it should process it
    /// in this function and return an [`Error`] to prevent further processing.
    async fn connect(self: &Arc<Self>, _peer: &SocketAddr) -> Result<()> {
        Ok(())
    }

    /// Called upon websocket disconnection
    async fn disconnect(self: &Arc<Self>, _ctx: Self::Context, _result: Result<()>) {}

    /// Called after [`Self::connect()`], after creating the [`tokio::sync::mpsc`] sender `sink`
    /// channel, allowing the server to execute additional handshake communication phase,
    /// or retain the sink for external message dispatch (such as server-side notifications).
    async fn handshake(
        self: &Arc<Self>,
        peer: &SocketAddr,
        sender: &mut WebSocketSender,
        receiver: &mut WebSocketReceiver,
        sink: &WebSocketSink,
    ) -> Result<Self::Context>;

    /// Called for every websocket message
    /// This function can return an error to terminate the connection
    async fn message(
        self: &Arc<Self>,
        ctx: &Self::Context,
        msg: Message,
        sink: &WebSocketSink,
    ) -> Result<()>;

    async fn ctl(self: &Arc<Self>, msg: Message, sender: &mut WebSocketSender) -> Result<()> {
        if let Message::Ping(data) = msg {
            sender.send(Message::Pong(data)).await?;
        }
        Ok(())
    }
}

/// WebSocketServer that provides the main websocket connection
/// and message processing loop that delivers messages to the
/// installed WebSocketHandler trait.
pub struct WebSocketServer<T>
where
    T: WebSocketHandler + Send + Sync + 'static + Sized,
{
    // pub connections: AtomicU64,
    pub counters: Arc<WebSocketCounters>,
    pub handler: Arc<T>,
    pub stop: DuplexChannel,
}

impl<T> WebSocketServer<T>
where
    T: WebSocketHandler + Send + Sync + 'static,
{
    pub fn new(handler: Arc<T>, counters: Option<Arc<WebSocketCounters>>) -> Arc<Self> {
        Arc::new(WebSocketServer {
            counters: counters.unwrap_or_default(),
            handler,
            stop: DuplexChannel::oneshot(),
        })
    }

    async fn handle_connection(
        self: &Arc<Self>,
        peer: SocketAddr,
        stream: TcpStream,
        config: Option<WebSocketConfig>,
    ) -> Result<()> {
        let ws_stream = accept_async_with_config(stream, config).await?;
        self.handler.connect(&peer).await?;
        // log_trace!("WebSocket connected: {}", peer);

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (sink_sender, sink_receiver) = tokio::sync::mpsc::unbounded_channel::<Message>();

        let ctx = match self
            .handler
            .handshake(&peer, &mut ws_sender, &mut ws_receiver, &sink_sender)
            .await
        {
            Ok(ctx) => ctx,
            Err(err) => {
                self.counters
                    .handshake_failures
                    .fetch_add(1, Ordering::Relaxed);
                return Err(err);
            }
        };

        let result = self
            .connection_task(&ctx, ws_sender, ws_receiver, sink_sender, sink_receiver)
            .await;
        self.handler.disconnect(ctx, result).await;
        // log_trace!("WebSocket disconnected: {}", peer);

        Ok(())
    }

    async fn connection_task(
        self: &Arc<Self>,
        ctx: &T::Context,
        mut ws_sender: WebSocketSender,
        mut ws_receiver: WebSocketReceiver,
        sink_sender: TokioUnboundedSender<Message>,
        mut sink_receiver: TokioUnboundedReceiver<Message>,
    ) -> Result<()> {
        loop {
            tokio::select! {
                msg = sink_receiver.recv() => {
                    let msg = msg.unwrap();
                    match msg {
                        Message::Binary(data)  => {
                            self.counters.tx_bytes.fetch_add(data.len(), Ordering::Relaxed);
                            ws_sender.send(Message::Binary(data)).await?;
                        },
                        Message::Text(text)  => {
                            self.counters.tx_bytes.fetch_add(text.len(), Ordering::Relaxed);
                            ws_sender.send(Message::Text(text)).await?;
                        },
                        Message::Close(_) => {
                            ws_sender.send(msg).await?;
                            break;
                        },
                        Message::Ping(data) => {
                            self.counters.tx_bytes.fetch_add(data.len(), Ordering::Relaxed);
                            ws_sender.send(Message::Ping(data)).await?;
                        },
                        Message::Pong(data) => {
                            self.counters.tx_bytes.fetch_add(data.len(), Ordering::Relaxed);
                            ws_sender.send(Message::Pong(data)).await?;
                        },
                        msg => {
                            ws_sender.send(msg).await?;
                        }
                    }
                },
                msg = ws_receiver.next() => {
                    match msg {
                        Some(msg) => {
                            let msg = msg?;
                            match msg {
                                Message::Binary(data)  => {
                                    self.counters.rx_bytes.fetch_add(data.len(), Ordering::Relaxed);
                                    self.handler.message(ctx, Message::Binary(data), &sink_sender).await?;
                                },
                                Message::Text(text)  => {
                                    self.counters.rx_bytes.fetch_add(text.len(), Ordering::Relaxed);
                                    self.handler.message(ctx, Message::Text(text), &sink_sender).await?;
                                },
                                Message::Close(_) => {
                                    self.handler.message(ctx, msg, &sink_sender).await?;
                                    break;
                                },
                                Message::Ping(data) => {
                                    self.counters.rx_bytes.fetch_add(data.len(), Ordering::Relaxed);
                                    cfg_if! {
                                        if #[cfg(feature = "ping-pong")] {
                                            self.handler.ctl(Message::Ping(data), &mut ws_sender).await?;
                                        } else {
                                            ws_sender.send(Message::Pong(data)).await?;
                                        }
                                    }
                                },
                                Message::Pong(data) => {
                                    self.counters.rx_bytes.fetch_add(data.len(), Ordering::Relaxed);
                                    cfg_if! {
                                        if #[cfg(feature = "ping-pong")] {
                                            self.handler.ctl(Message::Pong(data), &mut ws_sender).await?;
                                        } else {
                                            // ignore pong
                                        }
                                    }
                                },
                                _ => {
                                }
                            }
                        }
                        None => {
                            return Err(Error::AbnormalClose);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn bind(self: &Arc<Self>, addr: &str) -> Result<TcpListener> {
        let listener = TcpListener::bind(&addr).await.map_err(|err| {
            Error::Listen(format!(
                "WebSocket server unable to listen on `{addr}`: {err}",
            ))
        })?;
        // log_trace!("WebSocket server listening on: {}", addr);
        Ok(listener)
    }

    async fn accept(self: &Arc<Self>, stream: TcpStream, config: Option<WebSocketConfig>) {
        let peer = match stream.peer_addr() {
            Ok(peer_address) => peer_address,
            Err(_) => {
                self.counters
                    .handshake_failures
                    .fetch_add(1, Ordering::Relaxed);
                return;
            }
        };

        self.counters
            .total_connections
            .fetch_add(1, Ordering::Relaxed);
        self.counters
            .active_connections
            .fetch_add(1, Ordering::Relaxed);

        let self_ = self.clone();
        tokio::spawn(async move {
            if let Err(e) = self_.handle_connection(peer, stream, config).await {
                match e {
                    Error::WebSocketError(WebSocketError::ConnectionClosed)
                    | Error::WebSocketError(WebSocketError::Protocol(_))
                    | Error::WebSocketError(WebSocketError::Utf8) => (),
                    err => log_error!("Error processing connection: {}", err),
                }
            }
            self_
                .counters
                .active_connections
                .fetch_sub(1, Ordering::Relaxed)
        });
    }

    pub async fn listen(
        self: &Arc<Self>,
        listener: TcpListener,
        config: Option<WebSocketConfig>,
    ) -> Result<()> {
        loop {
            select! {
                stream = listener.accept().fuse() => {
                    if let Ok((stream,socket_addr)) = stream {
                        if self.handler.accept(&socket_addr) {
                            self.accept(stream, config).await;
                        }
                    }
                },
                _ = self.stop.request.receiver.recv().fuse() => break,
            }
        }

        self.stop
            .response
            .sender
            .send(())
            .await
            .map_err(|err| Error::Done(err.to_string()))
    }

    pub fn stop(&self) -> Result<()> {
        self.stop
            .request
            .sender
            .try_send(())
            .map_err(|err| Error::Stop(err.to_string()))
    }

    pub async fn join(&self) -> Result<()> {
        self.stop
            .response
            .receiver
            .recv()
            .await
            .map_err(|err| Error::Join(err.to_string()))
    }

    pub async fn stop_and_join(&self) -> Result<()> {
        self.stop()?;
        self.join().await
    }
}

/// Base WebSocketServer trait allows the [`WebSocketServer<T>`] struct
/// to be retained by the trait reference by casting it to the trait
/// as follows:
///
/// ```rust
/// use std::sync::Arc;
/// use async_trait::async_trait;
/// use workflow_websocket::server::{Result,WebSocketServerTrait,WebSocketConfig,TcpListener};
///
/// struct Server{}
///
/// #[async_trait]
/// impl WebSocketServerTrait for Server {
///     async fn bind(self: Arc<Self>, addr: &str) -> Result<TcpListener>{
///         unimplemented!()
///     }
///     async fn listen(self: Arc<Self>, listener : TcpListener, config: Option<WebSocketConfig>) -> Result<()>{
///         unimplemented!()
///     }
///     fn stop(&self) -> Result<()>{
///         unimplemented!()
///     }
///     async fn join(&self) -> Result<()>{
///         unimplemented!()
///     }
///     async fn stop_and_join(&self) -> Result<()>{
///         unimplemented!()
///     }
/// }
/// let server_trait: Arc<dyn WebSocketServerTrait> = Arc::new(Server{});
/// let server = server_trait.downcast_arc::<Server>();
/// ```
/// This can help simplify web socket handling in case the supplied
/// `T` generic contains complex generic types that typically
/// results in generics propagating up into the ownership type chain.
///
/// This trait is used in the [`workflow-rpc`](https://docs.rs/workflow-rpc)
/// crate to isolate `RpcHandler` generics from the RpcServer owning the WebSocket.
///
#[async_trait]
pub trait WebSocketServerTrait: DowncastSync {
    async fn bind(self: Arc<Self>, addr: &str) -> Result<TcpListener>;
    async fn listen(
        self: Arc<Self>,
        listener: TcpListener,
        config: Option<WebSocketConfig>,
    ) -> Result<()>;
    fn stop(&self) -> Result<()>;
    async fn join(&self) -> Result<()>;
    async fn stop_and_join(&self) -> Result<()>;
}
impl_downcast!(sync WebSocketServerTrait);

#[async_trait]
impl<T> WebSocketServerTrait for WebSocketServer<T>
where
    T: WebSocketHandler + Send + Sync + 'static + Sized,
{
    async fn bind(self: Arc<Self>, addr: &str) -> Result<TcpListener> {
        self.bind(addr).await
    }

    async fn listen(
        self: Arc<Self>,
        listener: TcpListener,
        config: Option<WebSocketConfig>,
    ) -> Result<()> {
        self.listen(listener, config).await
    }

    fn stop(&self) -> Result<()> {
        self.stop()
    }

    async fn join(&self) -> Result<()> {
        self.join().await
    }

    async fn stop_and_join(&self) -> Result<()> {
        self.stop_and_join().await
    }
}

pub mod handshake {
    //!
    //! Module containing simple convenience handshake functions
    //! such as `greeting()`
    //!     

    use super::*;

    /// Handshake closure function type for [`greeting()`] handshake
    pub type HandshakeFn = Pin<Box<dyn Send + Sync + Fn(&str) -> Result<()>>>;

    /// Simple greeting handshake where supplied closure receives
    /// the first message from the client and should return
    /// `Ok(())` to proceed or [`Error`] to abort the connection.
    pub async fn greeting<'ws>(
        timeout_duration: Duration,
        _sender: &'ws mut WebSocketSender,
        receiver: &'ws mut WebSocketReceiver,
        handler: HandshakeFn,
    ) -> Result<()> {
        let delay = tokio::time::sleep(timeout_duration);
        tokio::select! {
            msg = receiver.next() => {
                if let Some(Ok(msg)) = msg {
                    if msg.is_text() || msg.is_binary() {
                        return handler(msg.to_text()?);
                    }
                }
                Err(Error::MalformedHandshake)
            }
            _ = delay => {
                Err(Error::ConnectionTimeout)
            }
        }
    }
}
