//! Module containing RPC [`Notification`] closure wrappers
use crate::imports::*;

/// Base trait representing an RPC notification, used to retain
/// notification structures in an [`Interface`](super::Interface)
/// map without generics.
#[async_trait]
pub(crate) trait NotificationTrait<ServerContext, ConnectionContext>:
    Send + Sync + 'static
{
    async fn call_with_borsh(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        data: &[u8],
    ) -> ServerResult<()>;
    async fn call_with_serde_json(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        value: Value,
    ) -> ServerResult<()>;
}

/// Notification closure type
pub type NotificationFn<ServerContext, ConnectionContext, Msg> = Arc<
    Box<
        dyn Send
            + Sync
            + Fn(ServerContext, ConnectionContext, Msg) -> NotificationFnReturn<()>
            + 'static,
    >,
>;

/// Notification closure return type
pub type NotificationFnReturn<T> =
    Pin<Box<(dyn Send + Sync + 'static + Future<Output = ServerResult<T>>)>>;

/// RPC notification wrapper. Contains the notification closure function.

pub struct Notification<ServerContext, ConnectionContext, Msg>
where
    ServerContext: Send + Sync + 'static,
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    method: NotificationFn<ServerContext, ConnectionContext, Msg>,
}

impl<ServerContext, ConnectionContext, Msg> Notification<ServerContext, ConnectionContext, Msg>
where
    ServerContext: Send + Sync + 'static,
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new<FN>(method_fn: FN) -> Notification<ServerContext, ConnectionContext, Msg>
    where
        FN: Send
            + Sync
            + Fn(ServerContext, ConnectionContext, Msg) -> NotificationFnReturn<()>
            + 'static,
    {
        Notification {
            method: Arc::new(Box::new(method_fn)),
        }
    }
}

#[async_trait]
impl<ServerContext, ConnectionContext, Msg> NotificationTrait<ServerContext, ConnectionContext>
    for Notification<ServerContext, ConnectionContext, Msg>
where
    ConnectionContext: Clone + Send + Sync + 'static,
    ServerContext: Send + Sync + 'static,
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn call_with_borsh(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        data: &[u8],
    ) -> ServerResult<()> {
        let req = Msg::try_from_slice(data)
            .map_err(|err| ServerError::NotificationDeserialize(err.to_string()))?;
        (self.method)(server_ctx, connection_ctx, req).await
    }

    async fn call_with_serde_json(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        value: Value,
    ) -> ServerResult<()> {
        let req: Msg = serde_json::from_value(value)
            .map_err(|err| ServerError::NotificationDeserialize(err.to_string()))?;
        (self.method)(server_ctx, connection_ctx, req).await
    }
}
