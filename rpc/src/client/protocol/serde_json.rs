use core::marker::PhantomData;

use super::{Pending, PendingMap, ProtocolHandler};
pub use crate::client::error::Error;
pub use crate::client::result::Result;
use crate::client::Interface;
use crate::imports::*;
use crate::messages::serde_json::*;

pub type JsonResponseFn =
    Arc<Box<(dyn Fn(Result<Value>, Option<&Duration>) -> Result<()> + Sync + Send)>>;

/// Serde JSON RPC message handler and dispatcher
pub struct SerdeJsonProtocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    ws: Arc<WebSocket>,
    pending: PendingMap<Id, JsonResponseFn>,
    interface: Option<Arc<Interface<Ops>>>,
    // ops: PhantomData<Ops>,
    id: PhantomData<Id>,
}

impl<Ops, Id> SerdeJsonProtocol<Ops, Id>
where
    Id: IdT,
    Ops: OpsT,
{
    fn new(ws: Arc<WebSocket>, interface: Option<Arc<Interface<Ops>>>) -> Self {
        SerdeJsonProtocol::<Ops, Id> {
            ws,
            pending: Arc::new(Mutex::new(AHashMap::new())),
            interface,
            // ops: PhantomData,
            id: PhantomData,
        }
    }
}

type MessageInfo<Ops, Id> = (Option<Id>, Option<Ops>, Result<Value>);

impl<Ops, Id> SerdeJsonProtocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    fn decode(&self, server_message: &str) -> Result<MessageInfo<Ops, Id>> {
        // println!("incoming: server_message={server_message}");

        let msg: SerdeJsonServerMessage<Ops, Id> = serde_json::from_str(server_message)?;

        if let Some(error) = msg.error {
            Ok((msg.id, None, Err(error.into())))
        } else if msg.id.is_some() {
            if let Some(result) = msg.params {
                Ok((msg.id, None, Ok(result)))
            } else {
                Ok((msg.id, None, Err(Error::NoDataInSuccessResponse)))
            }
        } else if let Some(params) = msg.params {
            Ok((None, msg.method, Ok(params)))
        } else {
            Ok((None, None, Err(Error::NoDataInNotificationMessage)))
        }
    }

    // pub async fn request(&self, op: Value, data: Value) -> Result<Receiver<Result<Value>>> {

    pub async fn request<Req, Resp>(&self, op: Ops, req: Req) -> Result<Resp>
    where
        Req: MsgT,
        Resp: MsgT,
    {
        // let op: Value = serde_json::to_value(op)?;

        // let op = serde_json::to_value(op)?;
        // let payload = serde_json::to_value(req)?;
        // let receiver = protocol.request(op, payload).await?;
        // let resp = receiver.recv().await??;
        // <Resp as Deserialize>::deserialize(resp)
        //     .map_err(|e| Error::SerdeDeserialize(e.to_string()))

        // let id = u64::from_le_bytes(rand::random::<[u8; 8]>());
        let id = Id::generate();
        let (sender, receiver) = oneshot();

        {
            let mut pending = self.pending.lock().unwrap();
            pending.insert(
                id.clone(),
                Pending::new(Arc::new(Box::new(move |result, _duration| {
                    sender.try_send(result)?;
                    Ok(())
                }))),
            );
            drop(pending);
        }

        let payload = serde_json::to_value(req)?;
        let client_message = SerdeJsonClientMessage::new(Some(id), op, payload);
        let json = serde_json::to_string(&client_message)?;

        self.ws.post(WebSocketMessage::Text(json)).await?;

        let data = receiver.recv().await??;

        let resp = <Resp as Deserialize>::deserialize(data)
            .map_err(|e| Error::SerdeDeserialize(e.to_string()))?;
        Ok(resp)

        // Ok(receiver)
    }

    // pub async fn notify(&self, op: Value, data: Value) -> Result<()> {
    pub async fn notify<Msg>(&self, op: Ops, data: Msg) -> Result<()>
    where
        Msg: Serialize + Send + Sync + 'static,
    {
        // let id = None;
        // let op: Value = serde_json::to_value(op)?;
        let payload = serde_json::to_value(data)?;
        let client_message = SerdeJsonClientMessage::<Ops, Id>::new(None, op, payload);
        let json = serde_json::to_string(&client_message)?;
        self.ws.post(WebSocketMessage::Text(json)).await?;
        Ok(())
    }

    async fn handle_notification(&self, op: Ops, payload: Value) -> Result<()> {
        if let Some(interface) = &self.interface {
            interface
                .call_notification_with_serde_json(&op, payload)
                .await
                .unwrap_or_else(|err| log_trace!("error handling server notification {}", err));
        } else {
            log_trace!("unable to handle server notification - interface is not initialized");
        }

        Ok(())
    }
}

#[async_trait]
impl<Ops, Id> ProtocolHandler<Ops> for SerdeJsonProtocol<Ops, Id>
where
    Ops: OpsT,
    Id: IdT,
{
    fn new(ws: Arc<WebSocket>, interface: Option<Arc<Interface<Ops>>>) -> Self
    where
        Self: Sized,
    {
        SerdeJsonProtocol::new(ws, interface)
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

    async fn handle_message(&self, message: WebSocketMessage) -> Result<()> {
        if let WebSocketMessage::Text(server_message) = message {
            let (id, method, result) = self.decode(server_message.as_str())?;
            if let Some(id) = id {
                if let Some(pending) = self.pending.lock().unwrap().remove(&id) {
                    (pending.callback)(result, Some(&pending.timestamp.elapsed()))
                } else {
                    Err(Error::ResponseHandler(format!("{id:?}"))) // ("rpc callback with id {} not found", msg.id);
                }
            } else if let Some(method) = method {
                match result {
                    Ok(data) => self.handle_notification(method, data).await,
                    _ => Ok(()),
                }
            } else {
                Err(Error::NotificationMethod)
            }
        } else {
            return Err(Error::WebSocketMessageType);
        }
    }

    async fn handle_disconnect(&self) -> Result<()> {
        self.pending.lock().unwrap().retain(|_, pending| {
            (pending.callback)(Err(Error::Disconnect), None)
                .unwrap_or_else(|err| log_trace!("Error in RPC callback during timeout: `{err}`"));
            false
        });

        Ok(())
    }

    // async fn handle_notification(&self, msg: WebSocketMessage) -> Result<()> {
    //     Ok(())
    // }

    // async fn notification(&self,op: Self::Op, data : Self::Data) -> Result<()> {
    //     Ok(())
    // }
}
