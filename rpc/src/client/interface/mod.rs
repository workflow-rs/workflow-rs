pub mod notification;
use crate::imports::*;
pub use notification::*;

/// Collection of server-siede notification handlers
pub struct Interface<Ops>
where
    Ops: OpsT,
{
    notifications: AHashMap<Ops, Box<dyn NotificationTrait>>,
}

impl<Ops> Default for Interface<Ops>
where
    Ops: OpsT,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Ops> Interface<Ops>
where
    Ops: OpsT,
{
    pub fn new() -> Interface<Ops> {
        Interface {
            notifications: AHashMap::new(),
        }
    }

    pub fn notification<Msg>(&mut self, op: Ops, method: Notification<Msg>)
    where
        Ops: OpsT,
        Msg: BorshDeserialize + DeserializeOwned + Send + Sync + 'static,
    {
        let method: Box<dyn NotificationTrait> = Box::new(method);
        if self.notifications.insert(op.clone(), method).is_some() {
            panic!("RPC notification {:?} is declared multiple times", op)
        }
    }

    pub async fn call_notification_with_borsh(&self, op: &Ops, payload: &[u8]) -> ServerResult<()> {
        if let Some(notification) = self.notifications.get(op) {
            notification.call_with_borsh(payload).await
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub async fn call_notification_with_serde_json(
        &self,
        op: &Ops,
        payload: Value,
    ) -> ServerResult<()> {
        if let Some(notification) = self.notifications.get(op) {
            notification.call_with_serde_json(payload).await
        } else {
            Err(ServerError::NotFound)
        }
    }
}

impl<Ops> From<Interface<Ops>> for Option<Arc<Interface<Ops>>>
where
    Ops: OpsT,
{
    fn from(interface: Interface<Ops>) -> Self {
        Some(Arc::new(interface))
    }
}
