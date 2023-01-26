use crate::imports::*;

#[async_trait]
pub trait NotificationTrait: Send + Sync + 'static {
    async fn call_with_borsh(&self, data: &[u8]) -> ServerResult<()>;
    async fn call_with_serde_json(&self, value: Value) -> ServerResult<()>;
}

pub type NotificationFn<Msg> =
    Arc<Box<dyn Send + Sync + Fn(Msg) -> NotificationFnReturn<()> + 'static>>;

pub type NotificationFnReturn<T> =
    Pin<Box<(dyn Send + Sync + 'static + Future<Output = ServerResult<T>>)>>;

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
