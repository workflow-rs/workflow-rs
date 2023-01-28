//!
//! Module containing [`SerdeJsonProtocol`] responsible for server-side
//! dispatch of RPC methods and notifications when using `SerdeJson`
//! protocol.
//!
use super::Encoding;
use crate::imports::*;
use crate::messages::serde_json::*;
pub use crate::server::result::Result;
use crate::server::Interface;
use crate::server::ProtocolHandler;
use workflow_websocket::server::{
    Error as WebSocketError, Message, Result as WebSocketResult, WebSocketSink,
};

/// Server-side message serializer and dispatcher when using `SerdeJson` protocol.
pub struct SerdeJsonProtocol<ServerContext, ConnectionContext, Ops, Id>
where
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Ops: OpsT,
    Id: IdT,
{
    id: PhantomData<Id>,
    ops: PhantomData<Ops>,
    interface: Arc<Interface<ServerContext, ConnectionContext, Ops>>,
}

#[async_trait]
impl<ServerContext, ConnectionContext, Ops, Id>
    ProtocolHandler<ServerContext, ConnectionContext, Ops>
    for SerdeJsonProtocol<ServerContext, ConnectionContext, Ops, Id>
where
    ServerContext: Clone + Send + Sync + 'static,
    ConnectionContext: Clone + Send + Sync + 'static,
    Ops: OpsT,
    Id: IdT,
{
    fn new(interface: Arc<Interface<ServerContext, ConnectionContext, Ops>>) -> Self
    where
        Self: Sized,
    {
        SerdeJsonProtocol {
            id: PhantomData,
            ops: PhantomData,
            interface,
        }
    }

    fn encoding(&self) -> Encoding {
        Encoding::SerdeJson
    }

    async fn handle_message(
        &self,
        connection_ctx: ConnectionContext,
        msg: Message,
        sink: &WebSocketSink,
    ) -> WebSocketResult<()> {
        let text = &msg.into_text()?;
        println!("incoming client message: {text}");
        let req: SerdeJsonClientMessage<Ops, Id> =
            serde_json::from_str(text).map_err(|_| WebSocketError::MalformedMessage)?;

        if req.id.is_some() {
            let result = self
                .interface
                .call_method_with_serde_json(&req.method, connection_ctx, req.params)
                .await;

            match result {
                Ok(payload) => {
                    if let Ok(msg) = serde_json::to_string(&SerdeJsonServerMessage::new(
                        req.id,
                        Some(req.method),
                        Some(payload),
                        None,
                    )) {
                        if let Err(e) = sink.send(msg.into()) {
                            log_trace!("Sink error: {:?}", e);
                        }
                    }
                }
                Err(err) => {
                    if err == ServerError::Close {
                        return Err(WebSocketError::ServerClose);
                    } else {
                        let server_err = SerdeJsonServerError::from(err);
                        if let Ok(msg) = serde_json::to_string(&SerdeJsonServerMessage::new(
                            req.id,
                            Some(req.method),
                            None,
                            Some(server_err),
                        )) {
                            if let Err(e) = sink.send(msg.into()) {
                                log_trace!("Sink error: {:?}", e);
                            }
                        }
                    }
                }
            }
        } else {
            self.interface
                .call_notification_with_serde_json(&req.method, connection_ctx, req.params)
                .await
                .unwrap_or_else(|err| {
                    log_trace!("error handling client-side notification {}", err)
                });
        }
        Ok(())
    }
}

pub fn create_notify_message_with_serde_json<Ops, Msg>(op: Ops, msg: Msg) -> Result<Message>
where
    Ops: OpsT,
    Msg: Serialize + Send + Sync + 'static,
{
    let payload = serde_json::to_value(msg)?;
    let json = serde_json::to_string(&SerdeJsonServerMessage::<Ops, ()>::new(
        None,
        Some(op),
        Some(payload),
        None,
    ))?;
    Ok(Message::Text(json))
}
