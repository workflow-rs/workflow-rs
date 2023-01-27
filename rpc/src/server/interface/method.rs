//! Module containing RPC [`Method`] closure wrappers
use crate::imports::*;

/// Base trait representing an RPC method, used to retain
/// method structures in an [`Interface`](super::Interface)
/// map without generics.
#[async_trait]
pub(crate) trait MethodTrait<ServerContext, ConnectionContext>:
    Send + Sync + 'static
{
    async fn call_with_borsh(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        data: &[u8],
    ) -> ServerResult<Vec<u8>>;
    async fn call_with_serde_json(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        value: Value,
    ) -> ServerResult<Value>;
}

/// RPC method function type
pub type MethodFn<ServerContext, ConnectionContext, Req, Resp> = Arc<
    Box<
        dyn Send
            + Sync
            + Fn(ServerContext, ConnectionContext, Req) -> MethodFnReturn<Resp>
            + 'static,
    >,
>;

/// RPC method function return type
pub type MethodFnReturn<T> =
    Pin<Box<(dyn Send + Sync + 'static + Future<Output = ServerResult<T>>)>>;

/// RPC method wrapper. Contains the method closure function.
pub struct Method<ServerContext, ConnectionContext, Req, Resp>
where
    ServerContext: Send + Sync + 'static,
    Req: MsgT,
    Resp: MsgT,
{
    method: MethodFn<ServerContext, ConnectionContext, Req, Resp>,
}

impl<ServerContext, ConnectionContext, Req, Resp>
    Method<ServerContext, ConnectionContext, Req, Resp>
where
    ServerContext: Send + Sync + 'static,
    Req: MsgT,
    Resp: MsgT,
{
    pub fn new<FN>(method_fn: FN) -> Method<ServerContext, ConnectionContext, Req, Resp>
    where
        FN: Send
            + Sync
            + Fn(ServerContext, ConnectionContext, Req) -> MethodFnReturn<Resp>
            + 'static,
    {
        Method {
            method: Arc::new(Box::new(method_fn)),
        }
    }
}

#[async_trait]
impl<ServerContext, ConnectionContext, Req, Resp> MethodTrait<ServerContext, ConnectionContext>
    for Method<ServerContext, ConnectionContext, Req, Resp>
where
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Req: MsgT,
    Resp: MsgT,
{
    async fn call_with_borsh(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        data: &[u8],
    ) -> ServerResult<Vec<u8>> {
        let req = Req::try_from_slice(data)?;
        let resp = (self.method)(server_ctx, connection_ctx, req).await;
        let vec = <ServerResult<Resp> as BorshSerialize>::try_to_vec(&resp)?;
        Ok(vec)
    }

    async fn call_with_serde_json(
        &self,
        server_ctx: ServerContext,
        connection_ctx: ConnectionContext,
        value: Value,
    ) -> ServerResult<Value> {
        let req: Req = serde_json::from_value(value).map_err(|_| ServerError::ReqDeserialize)?;
        let resp = (self.method)(server_ctx, connection_ctx, req).await?;
        Ok(serde_json::to_value(resp).map_err(|_| ServerError::RespSerialize)?)
    }
}
