use super::{Pending, PendingMap, ProtocolHandler};
pub use crate::client::error::Error;
pub use crate::client::result::Result;
use crate::client::Interface;
use crate::imports::*;
use crate::messages::borsh::*;
use core::marker::PhantomData;

pub type BorshResponseFn =
    Arc<Box<(dyn Fn(Result<&[u8]>, Option<&Duration>) -> Result<()> + Sync + Send)>>;

/// Borsh RPC message handler and dispatcher
pub struct BorshProtocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    ws: Arc<WebSocket>,
    pending: PendingMap<Id, BorshResponseFn>,
    interface: Option<Arc<Interface<Ops>>>,
    ops: PhantomData<Ops>,
    id: PhantomData<Id>,
}

impl<Ops, Id> BorshProtocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    fn new(ws: Arc<WebSocket>, interface: Option<Arc<Interface<Ops>>>) -> Self {
        BorshProtocol {
            ws,
            pending: Arc::new(Mutex::new(AHashMap::new())),
            interface,
            ops: PhantomData,
            id: PhantomData,
        }
    }
}

type MessageInfo<'l, Ops, Id> = (Option<Id>, Option<Ops>, Result<&'l [u8]>);

impl<Ops, Id> BorshProtocol<Ops, Id>
where
    Id: IdT,
    Ops: OpsT,
{
    fn decode<'l>(&self, server_message: &'l [u8]) -> ServerResult<MessageInfo<'l, Ops, Id>> {
        match BorshServerMessage::try_from(server_message) {
            Ok(msg) => {
                let header = msg.header;
                match header.kind {
                    ServerMessageKind::Success => {
                        Ok((header.id, header.op, Ok(msg.payload)))
                        // Ok((Some(header.id), header.op.clone(), Ok(msg.data)))
                    }
                    ServerMessageKind::Error => {
                        if let Ok(err) = ServerError::try_from_slice(msg.payload) {
                            Ok((header.id, None, Err(Error::RpcCall(err))))
                        } else {
                            Ok((header.id, None, Err(Error::ErrorDeserializingResponseData)))
                        }
                    }
                    ServerMessageKind::Notification => Ok((None, header.op, Ok(msg.payload))),
                }
            }
            Err(err) => Err(ServerError::RespDeserialize(err.to_string())),
        }
    }

    pub async fn request<Req, Resp>(&self, op: Ops, req: Req) -> Result<Resp>
    where
        Req: MsgT,
        Resp: MsgT,
    {
        let payload = req.try_to_vec().map_err(|_| Error::BorshSerialize)?;

        // let id = u64::from_le_bytes(rand::random::<[u8; 8]>());
        let id = Id::generate();
        let (sender, receiver) = oneshot();

        {
            let mut pending = self.pending.lock().unwrap();
            pending.insert(
                id.clone(),
                Pending::new(Arc::new(Box::new(move |result, _duration| {
                    sender.try_send(result.map(|data| data.to_vec()))?;
                    Ok(())
                }))),
            );
            drop(pending);
        }

        // TODO - post error into sender if ws.send() fails
        self.ws
            .post(to_ws_msg(BorshReqHeader::new(Some(id), op), &payload))
            .await?;

        let data = receiver.recv().await??;
        let resp = ServerResult::<Resp>::try_from_slice(data.as_ref())
            .map_err(|e| Error::BorshDeserialize(e.to_string()))?;

        Ok(resp?)
    }

    pub async fn notify<Msg>(&self, op: Ops, payload: Msg) -> Result<()>
    where
        Msg: BorshSerialize + Send + Sync + 'static,
    {
        let payload = payload.try_to_vec().map_err(|_| Error::BorshSerialize)?;
        // let op: u32 = op.into();
        // let id = None;
        self.ws
            .post(to_ws_msg(
                BorshReqHeader::<Ops, Id>::new(None, op),
                &payload,
            ))
            .await?;
        Ok(())
    }

    async fn handle_notification(&self, op: &Ops, payload: &[u8]) -> Result<()> {
        if let Some(interface) = &self.interface {
            interface
                .call_notification_with_borsh(op, payload)
                .await
                .unwrap_or_else(|err| log_trace!("error handling server notification {}", err));
        } else {
            log_trace!("unable to handle server notification - interface is not initialized");
        }

        Ok(())
    }
}

#[async_trait]
impl<Ops, Id> ProtocolHandler<Ops> for BorshProtocol<Ops, Id>
where
    Id: IdT,
    Ops: OpsT,
{
    fn new(ws: Arc<WebSocket>, interface: Option<Arc<Interface<Ops>>>) -> Self
    where
        Self: Sized,
    {
        BorshProtocol::new(ws, interface)
    }

    async fn handle_timeout(&self, timeout: Duration) {
        self.pending.lock().unwrap().retain(|_, pending| {
            if pending.timestamp.elapsed() > timeout {
                (pending.callback)(Err(Error::Timeout), None).unwrap_or_else(|err| {
                    log_trace!("Error in RPC callback during timeout: `{err}`")
                });
                false
            } else {
                true
            }
        });
    }

    async fn handle_disconnect(&self) -> Result<()> {
        self.pending.lock().unwrap().retain(|_, pending| {
            (pending.callback)(Err(Error::Disconnect), None)
                .unwrap_or_else(|err| log_trace!("Error in RPC callback during timeout: `{err}`"));
            false
        });

        Ok(())
    }

    async fn handle_message(&self, message: WebSocketMessage) -> Result<()> {
        if let WebSocketMessage::Binary(server_message) = message {
            let (id, op, result) = self.decode(server_message.as_slice())?;
            if let Some(id) = id {
                if let Some(pending) = self.pending.lock().unwrap().remove(&id) {
                    (pending.callback)(result, Some(&pending.timestamp.elapsed()))
                } else {
                    Err(Error::ResponseHandler(format!("{id:?}")))
                }
            } else if let Some(op) = op {
                match result {
                    Ok(data) => self.handle_notification(&op, data).await,
                    _ => Ok(()),
                }
            } else {
                Err(Error::NotificationMethod)
            }
        } else {
            return Err(Error::WebSocketMessageType);
        }
    }
}
