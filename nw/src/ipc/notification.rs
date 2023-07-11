//! Module containing IPC [`Notification`] closure wrappers
use crate::ipc::imports::*;

/// Base trait representing an IPC notification, used to retain
/// notification structures in an IPC map without generics.
#[async_trait]
pub(crate) trait NotificationTrait: Send + Sync + 'static {
    async fn call_with_borsh(&self, data: &[u8]) -> ResponseResult<()>;
}

/// Notification closure type
pub type NotificationFn<Msg> =
    Arc<Box<dyn Send + Sync + Fn(Msg) -> NotificationFnReturn<()> + 'static>>;

/// Notification closure return type
pub type NotificationFnReturn<T> =
    Pin<Box<(dyn Send + 'static + Future<Output = ResponseResult<T>>)>>;

/// IPC notification wrapper. Contains the notification closure function.
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
    async fn call_with_borsh(&self, data: &[u8]) -> ResponseResult<()> {
        let req = Msg::try_from_slice(data)
            .map_err(|err| ResponseError::NotificationDeserialize(err.to_string()))?;
        (self.method)(req).await
    }
}
