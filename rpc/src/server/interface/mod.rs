//!
//! Module containing an [`Interface`] struct that carries
//! mappings of RPC method and notification handlers.
//!

pub mod method;
pub mod notification;

use crate::imports::*;
pub use method::*;
pub use notification::*;

/// [`Interface`] struct carries a mapping of RPC methods
/// and notifications, used by protocols to dispatch calls
/// to their respective handlers.
pub struct Interface<ServerContext, ConnectionContext, Ops>
where
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Ops: OpsT,
{
    server_ctx: ServerContext,
    methods: AHashMap<Ops, Box<dyn MethodTrait<ServerContext, ConnectionContext>>>,
    notifications: AHashMap<Ops, Box<dyn NotificationTrait<ServerContext, ConnectionContext>>>,
}

impl<ServerContext, ConnectionContext, Ops> Interface<ServerContext, ConnectionContext, Ops>
where
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Ops: OpsT,
{
    /// Create an interface that will contain user-defined
    /// RPC message and notification handlers. This method
    /// accepts `server_ctx` argument that will be subsequently
    /// passed to each RPC method or notification invocation.
    pub fn new(server_ctx: ServerContext) -> Interface<ServerContext, ConnectionContext, Ops> {
        Interface {
            server_ctx,
            methods: AHashMap::new(),
            notifications: AHashMap::new(),
        }
    }

    ///
    /// Declare an RPC method handler. You can use a [`method!()`](macro@crate::server::method)
    /// macro to declare the method as follows:
    ///
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
    ///
    pub fn method<Req, Resp>(
        &mut self,
        op: Ops,
        method: Method<ServerContext, ConnectionContext, Req, Resp>,
    )
    // -> Self
    where
        Ops: Debug + Clone,
        Req: MsgT,
        Resp: MsgT,
    {
        let method: Box<dyn MethodTrait<ServerContext, ConnectionContext>> = Box::new(method);
        if self.methods.insert(op.clone(), method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    ///
    /// Declare an RPC notification handler. You can use a [`notification!()`](macro@crate::server::notification)
    /// macro to declare the notification as follows:
    ///
    ///
    /// ```rust
    /// interface.notification(MyOps::Notify, notification!(
    ///   | connection_ctx: ConnectionCtx,
    ///     server_ctx: ServerContext,
    ///     msg: MyMsg |
    /// async move {
    ///     // ...
    ///     Ok(())
    /// }))
    /// ```
    ///
    ///
    pub fn notification<Msg>(
        &mut self,
        op: Ops,
        method: Notification<ServerContext, ConnectionContext, Msg>,
    ) where
        Ops: Debug + Clone,
        Msg: MsgT,
    {
        let method: Box<dyn NotificationTrait<ServerContext, ConnectionContext>> = Box::new(method);
        if self.notifications.insert(op.clone(), method).is_some() {
            panic!("RPC notification {op:?} is declared multiple times")
        }
    }

    pub(crate) async fn call_method_with_borsh(
        &self,
        op: &Ops,
        connection_ctx: ConnectionContext,
        payload: &[u8],
    ) -> ServerResult<Vec<u8>> {
        if let Some(method) = self.methods.get(op) {
            method
                .call_with_borsh(self.server_ctx.clone(), connection_ctx, payload)
                .await
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub(crate) async fn call_method_with_serde_json(
        &self,
        op: &Ops,
        connection_ctx: ConnectionContext,
        payload: Value,
    ) -> ServerResult<Value> {
        if let Some(method) = self.methods.get(op) {
            method
                .call_with_serde_json(self.server_ctx.clone(), connection_ctx, payload)
                .await
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub(crate) async fn call_notification_with_borsh(
        &self,
        op: &Ops,
        connection_ctx: ConnectionContext,
        payload: &[u8],
    ) -> ServerResult<()> {
        if let Some(notification) = self.notifications.get(op) {
            notification
                .call_with_borsh(self.server_ctx.clone(), connection_ctx, payload)
                .await
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub(crate) async fn call_notification_with_serde_json(
        &self,
        op: &Ops,
        connection_ctx: ConnectionContext,
        payload: Value,
    ) -> ServerResult<()> {
        if let Some(notification) = self.notifications.get(op) {
            notification
                .call_with_serde_json(self.server_ctx.clone(), connection_ctx, payload)
                .await
        } else {
            Err(ServerError::NotFound)
        }
    }
}
