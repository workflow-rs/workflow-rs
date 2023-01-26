mod borsh;
mod serde_json;
pub use crate::client::error::Error;
pub use crate::client::result::Result;
use crate::imports::*;

pub use self::borsh::BorshProtocol;
pub use self::serde_json::SerdeJsonProtocol;
use crate::client::Interface;

#[async_trait]
pub trait ProtocolHandler<Ops>: DowncastSync
where
    Ops: OpsT,
{
    fn new(ws: Arc<WebSocket>, interface: Option<Arc<Interface<Ops>>>) -> Self
    where
        Self: Sized;
    async fn handle_timeout(&self, timeout: Duration);
    async fn handle_message(&self, message: WebSocketMessage) -> Result<()>;
    async fn handle_disconnect(&self) -> Result<()>;
    // async fn handle_notification(&self, msg: WebSocketMessage) -> Result<()>;
}
impl_downcast!(sync ProtocolHandler<Ops> where Ops: OpsT);

struct Pending<F> {
    timestamp: Instant,
    callback: F,
}
impl<F> Pending<F> {
    fn new(callback: F) -> Self {
        Self {
            timestamp: Instant::now(),
            callback,
        }
    }
}

type PendingMap<Id, F> = Arc<Mutex<AHashMap<Id, Pending<F>>>>;
