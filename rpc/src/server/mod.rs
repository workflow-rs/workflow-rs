//!
//! RPC server module (native only). This module encapsulates
//! server-side types used to create an RPC server: [`RpcServer`],
//! [`RpcHandler`], [`Messenger`], [`Interface`] and the
//! protocol handlers: [`BorshProtocol`] and [`SerdeJsonProtocol`].
//!

pub mod error;
pub mod interface;
pub mod prelude;
mod protocol;
pub mod result;

pub use super::error::*;
use crate::imports::*;
pub use interface::{Interface, Method, Notification};
pub use protocol::{BorshProtocol, Encoding, ProtocolHandler, SerdeJsonProtocol};
pub use std::net::SocketAddr;
pub use tokio::sync::mpsc::UnboundedSender as TokioUnboundedSender;
pub use workflow_websocket::server::{
    Error as WebSocketError, Message, Result as WebSocketResult, WebSocketHandler,
    WebSocketReceiver, WebSocketSender, WebSocketServer, WebSocketServerTrait, WebSocketSink,
};
pub mod handshake {
    //! WebSocket handshake helpers
    pub use workflow_websocket::server::handshake::*;
}
use crate::server::result::Result;

///
/// method!() macro for declaration of RPC method handlers
///
/// This macro simplifies creation of async method handler
/// closures supplied to the RPC dispatch interface. An
/// async method closure requires to be *Box*ed
/// and its result must be *Pin*ned, resulting in the following
/// syntax:
///
/// ```rust
///
/// interface.method(Box::new(MyOps::Method, Method::new(|req: MyReq|
///     Box::pin(
///         async move {
///             // ...
///             Ok(MyResp { })
///         }
///     )
/// )))
///
/// ```
///
/// The method macro adds the required Box and Pin syntax,
/// simplifying the declaration as follows:
///
/// ```rust
/// interface.method(MyOps::Method, method!(
///   | connection_ctx: ConnectionCtx,
///     server_ctx: ServerContext,
///     req: MyReq |
/// async move {
///     // ...
///     Ok(MyResp { })
/// }))
/// ```
///
pub use workflow_rpc_macros::server_method as method;

///
/// notification!() macro for declaration of RPC notification handlers
///
/// This macro simplifies creation of async notification handler
/// closures supplied to the RPC notification interface. An
/// async notification closure requires to be *Box*ed
/// and its result must be *Pin*ned, resulting in the following
/// syntax:
///
/// ```rust
///
/// interface.notification(MyOps::Notify,Box::new(Notification::new(|msg: MyMsg|
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
/// ```rust
/// interface.notification(MyOps::Notify, notification!(|msg: MyMsg| async move {
///     // ...
///     Ok(())
/// }))
/// ```
///
pub use workflow_rpc_macros::server_notification as notification;

/// A basic example RpcContext, can be used to keep track of
/// connected peers.
#[derive(Debug, Clone)]
pub struct RpcContext {
    pub peer: SocketAddr,
}

/// [`RpcHandler`] - a server-side event handler for RPC connections.
#[async_trait]
pub trait RpcHandler: Send + Sync + 'static {
    type Context: Send + Sync;

    /// Connection notification - issued when the server has opened a WebSocket
    /// connection, before any other interactions occur.  The supplied argument
    /// is the [`SocketAddr`] of the incoming connection. This function should
    /// return [`WebSocketResult::Ok`] if the server accepts connection or
    /// [`WebSocketError`] if the connection is rejected. This function can
    /// be used to reject connections based on a ban list.
    async fn connect(self: Arc<Self>, _peer: &SocketAddr) -> WebSocketResult<()> {
        Ok(())
    }

    /// [`RpcHandler::handshake()`] is called right acter [`RpcHandler::connect()`]
    /// and is provided with a [`WebSocketSender`] and [`WebSocketReceiver`] channels
    /// which can be used to communicate with the underlying WebSocket connection
    /// to negotiate a connection. The function also receives the `&peer` ([`SocketAddr`])
    /// of the connection and a [`Messenger`] struct.  The [`Messenger`] struct can
    /// be used to post notifications to the given connection as well as to close it.
    /// If negotiation is successful, this function should return a `ConnectionContext`
    /// defined as [`Self::Context`]. This context will be supplied to all subsequent
    /// RPC calls received from this connection. The [`Messenger`] struct can be
    /// cloned and captured within the `ConnectionContext`. This allows an RPC
    /// method handler to later capture and post notifications to the connection
    /// asynchronously.
    async fn handshake(
        self: Arc<Self>,
        peer: &SocketAddr,
        sender: &mut WebSocketSender,
        receiver: &mut WebSocketReceiver,
        messenger: Arc<Messenger>,
    ) -> WebSocketResult<Arc<Self::Context>>;

    /// Disconnect notification, receives the context and the result containing
    /// the disconnection reason (can be success if the connection is closed gracefully)
    async fn disconnect(self: Arc<Self>, _ctx: Arc<Self::Context>, _result: WebSocketResult<()>) {}
}

///
/// The [`Messenger`] struct is supplied to the [`RpcHandler::handshake()`] call at
/// the connection negotiation time. This structure comes in as [`Arc<Messenger>`]
/// and can be retained for later processing. It provides two methods: [`Messenger::notify`]
/// that can be used asynchronously to dispatch RPC notifications to the client
/// and [`Messenger::close`] that can be used to terminate the RPC connection with
/// the client.
///
#[derive(Debug)]
pub struct Messenger {
    encoding: Encoding,
    sink: WebSocketSink,
}

impl Messenger {
    pub fn new(encoding: Encoding, sink: &WebSocketSink) -> Self {
        Self {
            encoding,
            sink: sink.clone(),
        }
    }

    pub fn close(&self) -> Result<()> {
        self.sink.send(Message::Close(None))?;
        Ok(())
    }

    pub async fn notify<Ops, Msg>(&self, op: Ops, msg: Msg) -> Result<()>
    where
        Ops: OpsT,
        Msg: BorshSerialize + BorshDeserialize + Serialize + Send + Sync + 'static,
    {
        match self.encoding {
            Encoding::Borsh => {
                self.sink
                    .send(protocol::create_notify_message_with_borsh(op, msg)?)?;
            }
            Encoding::SerdeJson => {
                self.sink
                    .send(protocol::create_notify_message_with_serde_json(op, msg)?)?;
            }
        }

        Ok(())
    }
}

/// WebSocket processor in charge of managing
/// WRPC Request/Response interactions.
#[derive(Clone)]
struct RpcWebSocketHandler<ConnectionContext, ServerContext, Protocol, Ops>
where
    Ops: OpsT,
    ConnectionContext: Send + Sync + 'static,
    ServerContext: Send + Sync + 'static,
    Protocol: ProtocolHandler<ConnectionContext, ServerContext, Ops> + Send + Sync + 'static,
{
    rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
    protocol: Arc<Protocol>,
    _server_ctx: PhantomData<ServerContext>,
    _ops: PhantomData<Ops>,
}

impl<ConnectionContext, ServerContext, Protocol, Ops>
    RpcWebSocketHandler<ConnectionContext, ServerContext, Protocol, Ops>
where
    Ops: OpsT,
    ConnectionContext: Send + Sync + 'static,
    ServerContext: Send + Sync + 'static,
    Protocol: ProtocolHandler<ConnectionContext, ServerContext, Ops> + Send + Sync + 'static,
{
    pub fn new(
        rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
        interface: Arc<Interface<ConnectionContext, ServerContext, Ops>>,
    ) -> Self {
        let protocol = Arc::new(Protocol::new(interface));
        Self {
            rpc_handler,
            protocol,
            _server_ctx: PhantomData,
            _ops: PhantomData,
        }
    }
}

#[async_trait]
impl<ConnectionContext, ServerContext, Protocol, Ops> WebSocketHandler
    for RpcWebSocketHandler<ConnectionContext, ServerContext, Protocol, Ops>
where
    Ops: OpsT,
    ConnectionContext: Send + Sync + 'static,
    ServerContext: Send + Sync + 'static,
    Protocol: ProtocolHandler<ConnectionContext, ServerContext, Ops> + Send + Sync + 'static,
{
    type Context = ConnectionContext;

    async fn connect(self: &Arc<Self>, peer: &SocketAddr) -> WebSocketResult<()> {
        self.rpc_handler.clone().connect(peer).await
    }

    async fn disconnect(self: &Arc<Self>, ctx: Arc<Self::Context>, result: WebSocketResult<()>) {
        self.rpc_handler.clone().disconnect(ctx, result).await
    }

    async fn handshake(
        self: &Arc<Self>,
        peer: &SocketAddr,
        sender: &mut WebSocketSender,
        receiver: &mut WebSocketReceiver,
        sink: &WebSocketSink,
    ) -> WebSocketResult<Arc<Self::Context>> {
        let messenger = Arc::new(Messenger::new(self.protocol.encoding(), sink));

        self.rpc_handler
            .clone()
            .handshake(peer, sender, receiver, messenger)
            .await
    }

    async fn message(
        self: &Arc<Self>,
        connection_ctx: &Arc<Self::Context>,
        msg: Message,
        sink: &WebSocketSink,
    ) -> WebSocketResult<()> {
        self.protocol
            .handle_message(connection_ctx.clone(), msg, sink)
            .await
    }
}

/// [`RpcServer`] - a server-side object that listens
/// for incoming websocket connections and delegates interaction
/// with them to the supplied interfaces: [`RpcHandler`] (for RPC server
/// management) and [`Interface`] (for method and notification dispatch).
#[derive(Clone)]
pub struct RpcServer {
    ws_server: Arc<dyn WebSocketServerTrait>,
}

impl RpcServer {
    /// Create a new [`RpcServer`] supplying an [`Arc`] of the previsouly-created
    /// [`RpcHandler`] trait and the [`Interface`] struct.
    /// This method takes 4 generics:
    /// - `ConnectionContext`: a struct used as [`RpcHandler::Context`] to
    /// represent the connection. This struct is passed to each RPC method
    /// and notification call.
    /// - `ServerContext`: a struct supplied to the [`Interface`] at the
    /// Interface creation time. This struct is passed to each RPC method
    /// and notification call.
    /// - `Protocol`: A protocol type used for the RPC message serialization
    /// and deserialization (this can be omitted by using [`RpcServer::new_with_encoding`])
    /// - `Ops`: A data type (index or an `enum`) representing the RPC method
    /// or notification.
    pub fn new<ConnectionContext, ServerContext, Protocol, Ops>(
        rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
        interface: Arc<Interface<ConnectionContext, ServerContext, Ops>>,
    ) -> RpcServer
    where
        ConnectionContext: Send + Sync + 'static,
        ServerContext: Send + Sync + 'static,
        Protocol: ProtocolHandler<ConnectionContext, ServerContext, Ops> + Send + Sync + 'static,
        Ops: OpsT,
    {
        let ws_handler = Arc::new(RpcWebSocketHandler::<
            ConnectionContext,
            ServerContext,
            Protocol,
            Ops,
        >::new(rpc_handler, interface));
        let ws_server = WebSocketServer::new(ws_handler);
        RpcServer { ws_server }
    }
    /// Create a new [`RpcServer`] supplying an [`Arc`] of the previsouly-created
    /// [`RpcHandler`] trait and the [`Interface`] struct.
    /// This method takes 4 generics:
    /// - `ConnectionContext`: a struct used as [`RpcHandler::Context`] to
    /// represent the connection. This struct is passed to each RPC method
    /// and notification call.
    /// - `ServerContext`: a struct supplied to the [`Interface`] at the
    /// Interface creation time. This struct is passed to each RPC method
    /// and notification call.
    /// - `Ops`: A data type (index or an `enum`) representing the RPC method
    /// or notification.
    /// - `Id`: A data type representing a message `Id` - this type must implement
    /// the [`id::Generator`](crate::id::Generator) trait. Implementation for default
    /// Ids such as [`Id32`] and [`Id64`] can be found in the [`id`](crate::id) module.
    ///
    /// This function call receives an `encoding`: [`Encoding`] argument containing
    /// [`Encoding::Borsh`] or [`Encoding::SerdeJson`], based on which it will
    /// instantiate the corresponding protocol handler ([`BorshProtocol`] or
    /// [`SerdeJsonProtocol`] respectively).
    ///
    pub fn new_with_encoding<ConnectionContext, ServerContext, Ops, Id>(
        encoding: Encoding,
        rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
        interface: Arc<Interface<ConnectionContext, ServerContext, Ops>>,
    ) -> RpcServer
    where
        ConnectionContext: Send + Sync + 'static,
        ServerContext: Send + Sync + 'static,
        Ops: OpsT,
        Id: IdT,
    {
        match encoding {
            Encoding::Borsh => RpcServer::new::<
                ConnectionContext,
                ServerContext,
                BorshProtocol<ConnectionContext, ServerContext, Ops, Id>,
                Ops,
            >(rpc_handler, interface),
            Encoding::SerdeJson => RpcServer::new::<
                ConnectionContext,
                ServerContext,
                SerdeJsonProtocol<ConnectionContext, ServerContext, Ops, Id>,
                Ops,
            >(rpc_handler, interface),
        }
    }

    /// Start listening for incoming RPC connections on the `addr`
    pub async fn listen(&self, addr: &str) -> WebSocketResult<()> {
        self.ws_server.clone().listen(addr).await
    }

    /// Signal the listening task to stop
    pub fn stop(&self) -> WebSocketResult<()> {
        self.ws_server.stop()
    }

    /// Blocks until the listening task has stopped
    pub async fn join(&self) -> WebSocketResult<()> {
        self.ws_server.join().await
    }

    /// Signal the listening task to stop and block
    /// until it has stopped
    pub async fn stop_and_join(&self) -> WebSocketResult<()> {
        self.ws_server.stop_and_join().await
    }
}
