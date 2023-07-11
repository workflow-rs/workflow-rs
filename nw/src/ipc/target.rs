use crate::ipc::imports::*;

#[derive(Clone, Debug)]
pub struct IpcTarget {
    target: Arc<JsValue>,
}

unsafe impl Send for IpcTarget {}
unsafe impl Sync for IpcTarget {}

impl IpcTarget {
    pub fn new(target: &JsValue) -> IpcTarget {
        log_info!("Creating IpcTarget for object {:?}", target);

        IpcTarget {
            target: Arc::new(target.clone()),
        }
    }
}

impl AsRef<JsValue> for IpcTarget {
    fn as_ref(&self) -> &JsValue {
        &self.target
    }
}

impl From<nw_sys::Window> for IpcTarget {
    fn from(window: nw_sys::Window) -> IpcTarget {
        IpcTarget::new(window.window().as_ref())
    }
}
