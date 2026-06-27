use crate::ipc::imports::*;

#[derive(Clone, Debug)]
/// A handle to an IPC peer context (a window or the global object), wrapping
/// the underlying JavaScript value that messages are dispatched to.
pub struct IpcTarget {
    target: Rc<JsValue>,
}

unsafe impl Send for IpcTarget {}
unsafe impl Sync for IpcTarget {}

impl IpcTarget {
    /// Creates an [`IpcTarget`] wrapping a clone of the given JavaScript value.
    pub fn new(target: &JsValue) -> IpcTarget {
        IpcTarget {
            target: Rc::new(target.clone()),
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

impl From<Arc<nw_sys::Window>> for IpcTarget {
    fn from(window: Arc<nw_sys::Window>) -> IpcTarget {
        IpcTarget::new(window.window().as_ref())
    }
}
