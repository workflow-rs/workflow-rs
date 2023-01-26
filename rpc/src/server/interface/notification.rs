//! Module containing RPC [`Notification`] closure wrappers
use crate::imports::*;

/// Base trait representing an RPC notification, used to retain
/// notification structures in an [`Interface`](super::Interface)
/// map without generics.
#[async_trait]
pub(crate) trait NotificationTrait<ConnectionContext, ServerContext>:
    Send + Sync + 'static
{
    async fn call_with_borsh(
        &self,
        connection_ctx: Arc<ConnectionContext>,
        server_ctx: Arc<ServerContext>,
        data: &[u8],
    ) -> ServerResult<()>;
    async fn call_with_serde_json(
        &self,
        connection_ctx: Arc<ConnectionContext>,
        server_ctx: Arc<ServerContext>,
        value: Value,
    ) -> ServerResult<()>;
}

/// Notification closure type
pub type NotificationFn<ConnectionContext, ServerContext, Msg> = Arc<
    Box<
        dyn Send
            + Sync
            + Fn(Arc<ConnectionContext>, Arc<ServerContext>, Msg) -> NotificationFnReturn<()>
            + 'static,
    >,
>;

/// Notification closure return type
pub type NotificationFnReturn<T> =
    Pin<Box<(dyn Send + Sync + 'static + Future<Output = ServerResult<T>>)>>;

/// RPC notification wrapper. Contains the notification closure function.

pub struct Notification<ConnectionContext, ServerContext, Msg>
where
    ServerContext: Send + Sync + 'static,
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    method: NotificationFn<ConnectionContext, ServerContext, Msg>,
}

impl<ConnectionContext, ServerContext, Msg> Notification<ConnectionContext, ServerContext, Msg>
where
    ServerContext: Send + Sync + 'static,
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new<FN>(method_fn: FN) -> Notification<ConnectionContext, ServerContext, Msg>
    where
        FN: Send
            + Sync
            + Fn(Arc<ConnectionContext>, Arc<ServerContext>, Msg) -> NotificationFnReturn<()>
            + 'static,
    {
        Notification {
            method: Arc::new(Box::new(method_fn)),
        }
    }
}

#[async_trait]
impl<ConnectionContext, ServerContext, Msg> NotificationTrait<ConnectionContext, ServerContext>
    for Notification<ConnectionContext, ServerContext, Msg>
where
    ConnectionContext: Send + Sync + 'static,
    ServerContext: Send + Sync + 'static,
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn call_with_borsh(
        &self,
        connection_ctx: Arc<ConnectionContext>,
        method_ctx: Arc<ServerContext>,
        data: &[u8],
    ) -> ServerResult<()> {
        let req = Msg::try_from_slice(data)
            .map_err(|err| ServerError::NotificationDeserialize(err.to_string()))?;
        (self.method)(connection_ctx, method_ctx, req).await
    }

    async fn call_with_serde_json(
        &self,
        connection_ctx: Arc<ConnectionContext>,
        method_ctx: Arc<ServerContext>,
        value: Value,
    ) -> ServerResult<()> {
        let req: Msg = serde_json::from_value(value)
            .map_err(|err| ServerError::NotificationDeserialize(err.to_string()))?;
        (self.method)(connection_ctx, method_ctx, req).await
    }
}
