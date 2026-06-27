use crate::imports::*;

#[async_trait]
pub trait NotificationTrait: Send + Sync + 'static {
    async fn call_with_borsh(&self, data: &[u8]) -> ServerResult<()>;
    async fn call_with_serde_json(&self, value: Value) -> ServerResult<()>;
}

pub type NotificationFn<Msg> =
    Arc<Box<dyn Send + Sync + Fn(Msg) -> NotificationFnReturn<()> + 'static>>;

pub type NotificationFnReturn<T> = Pin<Box<dyn Send + 'static + Future<Output = ServerResult<T>>>>;

/// A typed notification handler that deserializes an inbound `Msg` payload
/// (from Borsh or JSON) and invokes the wrapped async callback with it.
pub struct Notification<Msg>
where
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    method: NotificationFn<Msg>,
}

impl<Msg> Notification<Msg>
where
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Wrap the given async callback into a typed notification handler.
    pub fn new<FN>(method_fn: FN) -> Notification<Msg>
    where
        FN: Send + Sync + Fn(Msg) -> NotificationFnReturn<()> + 'static,
    {
        Notification {
            method: Arc::new(Box::new(method_fn)),
        }
    }
}

#[async_trait]
impl<Msg> NotificationTrait for Notification<Msg>
where
    Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn call_with_borsh(&self, data: &[u8]) -> ServerResult<()> {
        let msg = Msg::try_from_slice(data)
            .map_err(|err| ServerError::NotificationDeserialize(err.to_string()))?;
        (self.method)(msg).await
    }

    async fn call_with_serde_json(&self, value: Value) -> ServerResult<()> {
        let msg: Msg = serde_json::from_value(value)
            .map_err(|err| ServerError::NotificationDeserialize(err.to_string()))?;
        (self.method)(msg).await
    }
}
