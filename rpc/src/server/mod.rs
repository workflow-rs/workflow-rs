//!
//! RPC server module (native only). This module encapsulates
//! server-side types used to create an RPC server: [`RpcServer`],
//! [`RpcHandler`], [`Messenger`], [`Interface`] and the
//! protocol handlers: [`BorshProtocol`] and [`JsonProtocol`].
//!

pub mod error;
mod interface;
pub mod prelude;
pub mod protocol;
pub mod result;

pub use super::error::*;
pub use crate::encoding::Encoding;
use crate::imports::*;
pub use interface::{Interface, Method, Notification};
pub use protocol::{BorshProtocol, JsonProtocol, ProtocolHandler};
pub use std::net::SocketAddr;
pub use tokio::sync::mpsc::UnboundedSender as TokioUnboundedSender;
pub use workflow_websocket::server::{
    Error as WebSocketError, Message, Result as WebSocketResult, WebSocketConfig,
    WebSocketCounters, WebSocketHandler, WebSocketReceiver, WebSocketSender, WebSocketServer,
    WebSocketServerTrait, WebSocketSink,
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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

    /// Called to determine if the connection should be accepted.
    fn accept(&self, _peer: &SocketAddr) -> bool {
        true
    }

    /// Connection notification - issued when the server has opened a WebSocket
    /// connection, before any other interactions occur.  The supplied argument
    /// is the [`SocketAddr`] of the incoming connection. This function should
    /// return [`WebSocketResult::Ok`] if the server accepts connection or
    /// [`WebSocketError`] if the connection is rejected. This function can
    /// be used to reject connections based on a ban list.
    async fn connect(self: Arc<Self>, _peer: &SocketAddr) -> WebSocketResult<()> {
        Ok(())
    }

    /// [`RpcHandler::handshake()`] is called right after [`RpcHandler::connect()`]
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
    ) -> WebSocketResult<Self::Context>;

    /// Disconnect notification, receives the context and the result containing
    /// the disconnection reason (can be success if the connection is closed gracefully)
    async fn disconnect(self: Arc<Self>, _ctx: Self::Context, _result: WebSocketResult<()>) {}
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

    /// Close the WebSocket connection. The server checks for the connection channel
    /// for the dispatch of this message and relays it to the client as well as
    /// proactively terminates the connection.
    pub fn close(&self) -> Result<()> {
        self.sink.send(Message::Close(None))?;
        Ok(())
    }

    /// Post notification message to the WebSocket connection
    pub async fn notify<Ops, Msg>(&self, op: Ops, msg: Msg) -> Result<()>
    where
        Ops: OpsT,
        Msg: BorshSerialize + BorshDeserialize + Serialize + Send + Sync + 'static,
    {
        match self.encoding {
            Encoding::Borsh => {
                self.sink
                    .send(protocol::borsh::create_serialized_notification_message(
                        op, msg,
                    )?)?;
            }
            Encoding::SerdeJson => {
                self.sink
                    .send(protocol::serde_json::create_serialized_notification_message(op, msg)?)?;
            }
        }

        Ok(())
    }

    /// Serialize message into a [`tungstenite::Message`] for direct websocket delivery.
    /// Once serialized it can be relayed using [`Messenger::send_raw_message()`].
    pub fn serialize_notification_message<Ops, Msg>(
        &self,
        op: Ops,
        msg: Msg,
    ) -> Result<tungstenite::Message>
    where
        Ops: OpsT,
        Msg: MsgT,
    {
        match self.encoding {
            Encoding::Borsh => Ok(protocol::borsh::create_serialized_notification_message(
                op, msg,
            )?),
            Encoding::SerdeJson => {
                Ok(protocol::serde_json::create_serialized_notification_message(op, msg)?)
            }
        }
    }

    /// Send a raw [`tungstenite::Message`] via the websocket tokio channel.
    pub fn send_raw_message(&self, msg: tungstenite::Message) -> Result<()> {
        self.sink.send(msg)?;
        Ok(())
    }

    /// Provides direct access to the underlying tokio channel.
    pub fn sink(&self) -> &WebSocketSink {
        &self.sink
    }

    /// Get encoding of the current messenger.
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }
}

/// WebSocket processor in charge of managing
/// WRPC Request/Response interactions.
#[derive(Clone)]
struct RpcWebSocketHandler<ServerContext, ConnectionContext, Protocol, Ops>
where
    Ops: OpsT,
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Protocol: ProtocolHandler<ServerContext, ConnectionContext, Ops> + Send + Sync + 'static,
{
    rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
    protocol: Arc<Protocol>,
    _server_ctx: PhantomData<ServerContext>,
    _ops: PhantomData<Ops>,
}

impl<ServerContext, ConnectionContext, Protocol, Ops>
    RpcWebSocketHandler<ServerContext, ConnectionContext, Protocol, Ops>
where
    Ops: OpsT,
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Protocol: ProtocolHandler<ServerContext, ConnectionContext, Ops> + Send + Sync + 'static,
{
    pub fn new(
        rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
        interface: Arc<Interface<ServerContext, ConnectionContext, Ops>>,
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
impl<ServerContext, ConnectionContext, Protocol, Ops> WebSocketHandler
    for RpcWebSocketHandler<ServerContext, ConnectionContext, Protocol, Ops>
where
    Ops: OpsT,
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Protocol: ProtocolHandler<ServerContext, ConnectionContext, Ops> + Send + Sync + 'static,
{
    type Context = ConnectionContext;

    fn accept(&self, peer: &SocketAddr) -> bool {
        self.rpc_handler.accept(peer)
    }

    async fn connect(self: &Arc<Self>, peer: &SocketAddr) -> WebSocketResult<()> {
        self.rpc_handler.clone().connect(peer).await
    }

    async fn disconnect(self: &Arc<Self>, ctx: Self::Context, result: WebSocketResult<()>) {
        self.rpc_handler.clone().disconnect(ctx, result).await
    }

    async fn handshake(
        self: &Arc<Self>,
        peer: &SocketAddr,
        sender: &mut WebSocketSender,
        receiver: &mut WebSocketReceiver,
        sink: &WebSocketSink,
    ) -> WebSocketResult<Self::Context> {
        let messenger = Arc::new(Messenger::new(self.protocol.encoding(), sink));

        self.rpc_handler
            .clone()
            .handshake(peer, sender, receiver, messenger)
            .await
    }

    async fn message(
        self: &Arc<Self>,
        connection_ctx: &Self::Context,
        msg: Message,
        sink: &WebSocketSink,
    ) -> WebSocketResult<()> {
        self.protocol
            .handle_message((*connection_ctx).clone(), msg, sink)
            .await
    }
}

// trait Encoder<Ops> : DowncastSync + Sized + Send + Sync {
//     fn serialize_notification_message<Msg>(
//         &self,
//         op: Ops,
//         msg: Msg,
//     ) -> Result<tungstenite::Message>
//     where
//         Msg: BorshSerialize + BorshDeserialize + Serialize + Send + Sync + 'static;
// }

// // impl_downcast!(sync Encoder<Ops> where Ops: Send + Sync + 'static);

// impl<ServerContext, ConnectionContext, Protocol, Ops> Encoder<Ops>
//     for RpcWebSocketHandler<ServerContext, ConnectionContext, Protocol, Ops>
// where
//     Ops: OpsT,
//     ServerContext: Clone + Send + Sync + 'static,
//     ConnectionContext: Clone + Send + Sync + 'static,
//     Protocol: ProtocolHandler<ServerContext, ConnectionContext, Ops> + Send + Sync + 'static,
// {
//     fn serialize_notification_message<Msg>(
//         &self,
//         op: Ops,
//         msg: Msg,
//     ) -> Result<tungstenite::Message>
//     where
//         Ops: OpsT,
//         Msg: BorshSerialize + BorshDeserialize + Serialize + Send + Sync + 'static
//     {
//         self.protocol.serialize_notification_message(op, msg)
//     }
// }

/// [`RpcServer`] - a server-side object that listens
/// for incoming websocket connections and delegates interaction
/// with them to the supplied interfaces: [`RpcHandler`] (for RPC server
/// management) and [`Interface`] (for method and notification dispatch).
#[derive(Clone)]
pub struct RpcServer {
    ws_server: Arc<dyn WebSocketServerTrait>,
}

impl RpcServer {
    /// Create a new [`RpcServer`] supplying an [`Arc`] of the previously-created
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
    pub fn new<ServerContext, ConnectionContext, Protocol, Ops>(
        rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
        interface: Arc<Interface<ServerContext, ConnectionContext, Ops>>,
        counters: Option<Arc<WebSocketCounters>>,
    ) -> RpcServer
    where
        ServerContext: Clone + Send + Sync + 'static,
        ConnectionContext: Clone + Send + Sync + 'static,
        Protocol: ProtocolHandler<ServerContext, ConnectionContext, Ops> + Send + Sync + 'static,
        Ops: OpsT,
    {
        let ws_handler = Arc::new(RpcWebSocketHandler::<
            ServerContext,
            ConnectionContext,
            Protocol,
            Ops,
        >::new(rpc_handler, interface));

        let ws_server = WebSocketServer::new(ws_handler, counters);
        RpcServer { ws_server }
    }
    /// Create a new [`RpcServer`] supplying an [`Arc`] of the previously-created
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
    /// [`JsonProtocol`] respectively).
    ///
    pub fn new_with_encoding<ServerContext, ConnectionContext, Ops, Id>(
        encoding: Encoding,
        rpc_handler: Arc<dyn RpcHandler<Context = ConnectionContext>>,
        interface: Arc<Interface<ServerContext, ConnectionContext, Ops>>,
        counters: Option<Arc<WebSocketCounters>>,
    ) -> RpcServer
    where
        ServerContext: Clone + Send + Sync + 'static,
        ConnectionContext: Clone + Send + Sync + 'static,
        Ops: OpsT,
        Id: IdT,
    {
        match encoding {
            Encoding::Borsh => RpcServer::new::<
                ServerContext,
                ConnectionContext,
                BorshProtocol<ServerContext, ConnectionContext, Ops, Id>,
                Ops,
            >(rpc_handler, interface, counters),
            Encoding::SerdeJson => RpcServer::new::<
                ServerContext,
                ConnectionContext,
                JsonProtocol<ServerContext, ConnectionContext, Ops, Id>,
                Ops,
            >(rpc_handler, interface, counters),
        }
    }

    // pub fn

    /// Start listening for incoming RPC connections on the `addr`
    pub async fn listen(&self, addr: &str, config: Option<WebSocketConfig>) -> WebSocketResult<()> {
        let addr = addr.replace("wrpc://", "");
        self.ws_server.clone().listen(&addr, config).await
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
