//!
//! Protocol module containing protocol handlers in charge
//! of incoming and outgoing message serialization and
//! RPC method and notification dispatch.
//!

pub mod borsh;
pub mod serde_json;

use crate::imports::*;
use crate::server::Interface;
pub use crate::server::result::Result;
use workflow_websocket::server::{Message, Result as WebSocketResult, WebSocketSink};

pub use self::borsh::BorshProtocol;
pub use self::serde_json::JsonProtocol;

/// Base trait for [`BorshProtocol`] and [`JsonProtocol`] protocol handlers
#[async_trait]
pub trait ProtocolHandler<ServerContext, ConnectionContext, Ops>:
    DowncastSync + Sized + Send + Sync
where
    Ops: OpsT,
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
{
    /// Construct a protocol handler backed by the given RPC method/notification interface.
    fn new(methods: Arc<Interface<ServerContext, ConnectionContext, Ops>>) -> Self
    where
        Self: Sized;

    /// Return the wire encoding (`Borsh` or `JSON`) implemented by this handler.
    fn encoding(&self) -> Encoding;

    /// Decode an incoming WebSocket message, dispatch it to the matching method
    /// or notification handler, and send any response back through the sink.
    async fn handle_message(
        &self,
        connection_ctx: ConnectionContext,
        message: Message,
        sink: &WebSocketSink,
    ) -> WebSocketResult<()>;

    /// Serialize a server-initiated notification for the given operation and
    /// message into an outgoing WebSocket message.
    fn serialize_notification_message<Msg>(
        &self,
        op: Ops,
        msg: Msg,
    ) -> Result<tungstenite::Message>
    where
        Msg: BorshSerialize + Serialize + Send + Sync + 'static;
}
