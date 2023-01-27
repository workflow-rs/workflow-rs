//!
//! Module containing [`BorshProtocol`] responsible for server-side
//! dispatch of RPC methods and notifications when using `Borsh`
//! protocol.
//!

use super::Encoding;
use crate::imports::*;
use crate::messages::borsh::*;
pub use crate::server::result::Result;
use crate::server::Interface;
use crate::server::ProtocolHandler;
use workflow_websocket::server::{
    Error as WebSocketError, Message, Result as WebSocketResult, WebSocketSink,
};

/// Server-side message serializer and dispatcher when using `Borsh` protocol.
pub struct BorshProtocol<ServerContext, ConnectionContext, Ops, Id>
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
    for BorshProtocol<ServerContext, ConnectionContext, Ops, Id>
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
        BorshProtocol {
            id: PhantomData,
            ops: PhantomData,
            interface,
        }
    }

    fn encoding(&self) -> Encoding {
        Encoding::Borsh
    }

    async fn handle_message(
        &self,
        connection_ctx: ConnectionContext,
        msg: Message,
        sink: &WebSocketSink,
    ) -> WebSocketResult<()> {
        let data = &msg.into_data();
        let req: BorshClientMessage<Ops, Id> = data
            .try_into()
            .map_err(|_| WebSocketError::MalformedMessage)?;

        if req.header.id.is_some() {
            let result = self
                .interface
                .call_method_with_borsh(&req.header.op, connection_ctx, req.payload)
                .await;

            match result {
                Ok(data) => {
                    if let Ok(msg) = BorshServerMessage::<Ops, Id>::new(
                        BorshServerMessageHeader::new(
                            req.header.id,
                            ServerMessageKind::Success,
                            Some(req.header.op),
                        ),
                        &data,
                    )
                    .try_to_vec()
                    {
                        if let Err(e) = sink.send(msg.into()) {
                            log_trace!("Sink error: {:?}", e);
                        }
                    }
                }
                Err(err) => {
                    log_trace!("RPC server error: {:?} req: {:#?}", err, req);
                    if err == ServerError::Close {
                        return Err(WebSocketError::ServerClose);
                    } else if let Ok(err_vec) = err.try_to_vec() {
                        if let Ok(msg) = BorshServerMessage::new(
                            BorshServerMessageHeader::<Ops, Id>::new(
                                req.header.id,
                                ServerMessageKind::Error,
                                None,
                            ),
                            &err_vec,
                        )
                        .try_to_vec()
                        {
                            if let Err(e) = sink.send(msg.into()) {
                                log_trace!("Sink error: {:?}", e);
                            }
                        }
                    }
                }
            }
        } else {
            self.interface
                .call_notification_with_borsh(&req.header.op, connection_ctx, req.payload)
                .await
                .unwrap_or_else(|err| {
                    log_trace!("error handling client-side notification {}", err)
                });
        }

        Ok(())
    }
}

pub fn create_notify_message_with_borsh<Ops, Msg>(op: Ops, msg: Msg) -> Result<Message>
where
    Ops: BorshSerialize + BorshDeserialize + Send + Sync + 'static,
    Msg: BorshSerialize + Send + Sync + 'static,
{
    let payload = msg.try_to_vec()?;
    let data = BorshServerMessage::new(
        BorshServerMessageHeader::<Ops, ()>::new(None, ServerMessageKind::Notification, Some(op)),
        &payload,
    )
    .try_to_vec()?;
    Ok(Message::Binary(data))
}
