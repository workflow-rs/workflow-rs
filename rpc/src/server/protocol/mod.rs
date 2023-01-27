//!
//! Protocol module containing protocol handlers in charge
//! of incoming and outgoing message serialization and
//! RPC method and notification dispatch.
//!

mod borsh;
mod serde_json;

use crate::imports::*;
use crate::server::Interface;
use workflow_websocket::server::{Message, Result as WebSocketResult, WebSocketSink};

pub use self::borsh::*;
pub use self::serde_json::*;

/// Base trait for [`BorshProtocol`] and [`SerdeJsonProtocol`] protocol handlers
#[async_trait]
pub trait ProtocolHandler<ServerContext, ConnectionContext, Ops>:
    DowncastSync + Sized + Send + Sync
where
    Ops: OpsT,
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
{
    fn new(methods: Arc<Interface<ServerContext, ConnectionContext, Ops>>) -> Self
    where
        Self: Sized;

    fn encoding(&self) -> Encoding;

    async fn handle_message(
        &self,
        connection_ctx: ConnectionContext,
        message: Message,
        sink: &WebSocketSink,
    ) -> WebSocketResult<()>;
}
