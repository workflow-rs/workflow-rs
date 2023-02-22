//!
//! Cross-window IPC (message posting and function execution across multiple NWJS windows)
//!

use crate::error::Error;
use crate::result::Result;
use ahash::AHashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use js_sys::{ArrayBuffer, Function, Object, Reflect, Uint8Array};
use nw_sys::prelude::*;
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use web_sys::{BroadcastChannel, MessageEvent, Window};
use workflow_core::channel::{oneshot, Channel};
use workflow_core::id::Id;
use workflow_log::*;
use workflow_wasm::prelude::*;

pub enum Ctl {
    Attach { origin: Id, meta: JsValue },
    Detach { origin: Id },
}

impl TryFrom<Ctl> for JsValue {
    type Error = Error;
    fn try_from(ctl: Ctl) -> Result<Self> {
        let object = Object::new();
        match ctl {
            Ctl::Attach { origin, meta } => {
                let kind = JsValue::from("attach");
                let origin = to_value(&origin)?;
                object.set_properties(&[("type", &kind), ("origin", &origin), ("meta", &meta)])?;
            }

            Ctl::Detach { origin } => {
                let kind = JsValue::from("detach");
                let origin = to_value(&origin)?;
                object.set_properties(&[("type", &kind), ("origin", &origin)])?;
            }
        }
        Ok(object.into())
    }
}

impl TryFrom<JsValue> for Ctl {
    type Error = Error;
    fn try_from(value: JsValue) -> std::result::Result<Self, Self::Error> {
        let object = Object::try_from(&value).ok_or(Error::MalformedCtl)?;
        let kind = object.get("type")?;
        let kind = kind.as_string().ok_or(Error::MalformedCtl)?;
        let origin = object.get("origin")?;
        let origin: Id = from_value(origin)?;
        match kind.as_str() {
            "attach" => {
                let meta = object.get("meta")?;
                Ok(Ctl::Attach { origin, meta })
            }
            "detach" => Ok(Ctl::Detach { origin }),
            _ => Err(Error::MalformedCtl),
        }
    }
}

enum BroadcastMessageKind {
    Post,
    Request,
    Response,
}

impl From<BroadcastMessageKind> for JsValue {
    fn from(kind: BroadcastMessageKind) -> Self {
        match kind {
            BroadcastMessageKind::Post => JsValue::from("post"),
            BroadcastMessageKind::Request => JsValue::from("request"),
            BroadcastMessageKind::Response => JsValue::from("response"),
        }
    }
}

impl TryFrom<JsValue> for BroadcastMessageKind {
    type Error = Error;
    fn try_from(value: JsValue) -> std::result::Result<Self, Self::Error> {
        let kind = value
            .as_string()
            .ok_or(Error::UnknownBroadcastMessageKind)?;
        match kind.as_str() {
            "post" => Ok(BroadcastMessageKind::Post),
            "request" => Ok(BroadcastMessageKind::Request),
            "response" => Ok(BroadcastMessageKind::Response),
            _ => Err(Error::UnknownBroadcastMessageKind),
        }
    }
}

pub struct Message {
    #[allow(dead_code)]
    payload: JsValue,
}

struct Pending<F> {
    callback: F,
}
impl<F> Pending<F> {
    fn new(callback: F) -> Self {
        Self { callback }
    }
}

type PendingMap<Id, F> = Arc<Mutex<AHashMap<Id, Pending<F>>>>;

pub type ResponseFn =
    Arc<Box<(dyn Fn(std::result::Result<JsValueSend, JsValueSend>) -> Result<()> + Sync + Send)>>;

pub type __RequestFn<Req, Resp> = dyn Fn(Req) -> Result<Resp> + Sync + Send;
pub type RequestFn<Req, Resp> = Arc<Box<(dyn Fn(Req) -> Result<Resp> + Sync + Send)>>;

pub struct RouterInner {
    id: Id,
    map: Object,
    broadcast_ctl_channel: BroadcastChannel,
    ctl_channel: Channel<Ctl>,
    broadcast_msg_channel: BroadcastChannel,
    msg_channel: Channel<Message>,
    callbacks: CallbackMap,
    handlers: CallbackMap,
    pending: PendingMap<Id, ResponseFn>,
}

const IPC_PROXY_POST_WITH_BORSH: &str = "__ipc_post_borsh";
const IPC_PROXY_CALL_WITH_BORSH: &str = "__ipc_call_borsh";
const IPC_PROXY_POST_WITH_SERDE: &str = "__ipc_post_serde";
const IPC_PROXY_CALL_WITH_SERDE: &str = "__ipc_call_serde";

#[derive(Clone)]
pub struct Router {
    inner: Arc<RouterInner>,
}

impl Router {
    pub fn try_new(meta: JsValue) -> Result<Self> {
        let id = Id::new();
        let window = web_sys::window().unwrap();
        let global_jsv = js_sys::Reflect::get(&window, &JsValue::from("global"))?;
        let global = Object::try_from(&global_jsv).ok_or(Error::GlobalObjectNotFound)?;

        let map = Object::try_from(&global.get("__workflow_ipc")?)
            .cloned()
            .unwrap_or_else(|| {
                let map = Object::new();
                global
                    .set("__workflow_ipc", &map)
                    .expect("Unable to register `__workflow_ipc` property");
                map
            });

        map.set(id.to_string().as_str(), &window)
            .expect("Unable to assign to `__workflow_ipc` property");

        let broadcast_ctl_channel = BroadcastChannel::new("__workflow_ipc_ctl")?;
        let broadcast_msg_channel = BroadcastChannel::new("__workflow_ipc_msg")?;

        let inner = RouterInner {
            map,
            id,
            broadcast_ctl_channel: broadcast_ctl_channel.clone(),
            ctl_channel: Channel::unbounded(),
            broadcast_msg_channel: broadcast_msg_channel.clone(),
            msg_channel: Channel::unbounded(),
            callbacks: CallbackMap::default(),
            handlers: CallbackMap::default(),
            pending: PendingMap::default(),
        };

        let router = Router {
            inner: Arc::new(inner),
        };

        let router_ = router.clone();
        let ctl_handler = callback!(move |message: MessageEvent| {
            router_.handle_broadcast_ctl(message);
        });
        broadcast_ctl_channel.set_onmessage(Some(ctl_handler.as_ref()));
        router.inner.callbacks.retain(ctl_handler)?;

        // let id_str = id.to_string();
        let router_ = router.clone();
        let msg_handler = callback!(move |message: MessageEvent| {
            if let Err(err) = router_.handle_broadcast_msg(message) {
                log_error!("Error handling broadcast message: {err}");
            }
        });
        broadcast_msg_channel.set_onmessage(Some(msg_handler.as_ref()));
        router.inner.callbacks.retain(msg_handler)?;

        // ~
        let router_ = router.clone();
        router.declare_handler(
            &window,
            IPC_PROXY_POST_WITH_BORSH,
            callback!(move |message: JsValue| {
                router_.handle_post_with_borsh(message);
            }),
        )?;
        let router_ = router.clone();
        router.declare_handler(
            &window,
            IPC_PROXY_CALL_WITH_BORSH,
            callback!(move |message: JsValue| {
                router_.handle_call_with_borsh(message);
            }),
        )?;
        let router_ = router.clone();
        router.declare_handler(
            &window,
            IPC_PROXY_POST_WITH_SERDE,
            callback!(move |message: JsValue| {
                router_.handle_post_with_serde(message);
            }),
        )?;
        let router_ = router.clone();
        router.declare_handler(
            &window,
            IPC_PROXY_CALL_WITH_SERDE,
            callback!(move |message: JsValue| {
                router_.handle_call_with_serde(message);
            }),
        )?;

        let router_ = router.clone();
        let transfer_handler = callback!(move |message: JsValue| {
            router_.handle_transfer(message);
        });
        window.add_event_listener_with_callback("message", transfer_handler.as_ref())?;

        router.broadcast_ctl(Ctl::Attach { origin: id, meta })?;

        Ok(router)
    }

    pub fn declare_handler(
        &self,
        window: &Window,
        proxy: &str,
        handler: Callback<dyn FnMut(JsValue)>,
    ) -> Result<()> {
        if let Err(err) = window.set(proxy, handler.as_ref()) {
            panic!("Unable to assign to `{proxy}` property to window object: {err:?}")
        }
        self.inner.callbacks.retain(handler)?;
        Ok(())
    }

    pub fn get_call_handler(&self, id: &Id, name: &str) -> Result<Function> {
        let window = self.get(id)?;
        let handler = js_sys::Reflect::get(&window, &JsValue::from(name))?;
        Ok(handler.dyn_into::<Function>()?)
    }

    pub fn detach(&self) {
        self.inner.broadcast_ctl_channel.set_onmessage(None);
        self.inner.broadcast_msg_channel.set_onmessage(None);

        if let Err(err) = self.broadcast_ctl(Ctl::Detach { origin: self.id() }) {
            log_error!("IPC router is unable to broadcast detach ctl: {}", err);
        }

        if let Err(err) = self.map().delete(self.id().to_string().as_str()) {
            log_error!(
                "IPC unable to detach window from the global map object: {:?}",
                err
            );
        }

        let window = web_sys::window().unwrap();

        [
            IPC_PROXY_CALL_WITH_BORSH,
            IPC_PROXY_CALL_WITH_SERDE,
            IPC_PROXY_POST_WITH_BORSH,
            IPC_PROXY_POST_WITH_SERDE,
        ]
        .iter()
        .for_each(|n| {
            window
                .delete(n)
                .map_err(|err| {
                    log_error!("IPC unable to remove window message handler `{n}`: {err:?}");
                })
                .ok();
        });

        let handlers = self.inner.handlers.inner();
        for (_, handler) in handlers.iter() {
            window
                .remove_event_listener_with_callback("message", handler.get_fn())
                .map_err(|err| {
                    log_error!("IPC unable to remove message handler: {:?}", err);
                })
                .ok();
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Id {
        self.inner.id
    }

    #[inline(always)]
    pub fn map(&self) -> &Object {
        &self.inner.map
    }

    pub fn get(&self, id: &Id) -> Result<Window> {
        let window: Window = js_sys::Reflect::get(self.map(), &JsValue::from(id.to_string()))?
            .dyn_into::<Window>()
            .map_err(|_| Error::IpcTargetNotFound(*id))?;
        Ok(window)
    }

    pub fn broadcast_ctl(&self, ctl: Ctl) -> Result<()> {
        // let js_msg = to_value(&ctl)?;
        self.inner
            .broadcast_ctl_channel
            .post_message(&ctl.try_into()?)?;
        //send(js_msg).map_err(|err| {}).ok();
        Ok(())
    }

    pub fn handle_broadcast_ctl(&self, message: MessageEvent) {
        if self.inner.ctl_channel.sender.receiver_count() > 1 {
            let data = message.data();
            if let Ok(ctl) = Ctl::try_from(data).map_err(|err| {
                log_error!("IPC router unable to deserialize ctl message: {:?}", err);
            }) {
                self.inner
                    .ctl_channel
                    .sender
                    .try_send(ctl)
                    .map_err(|err| {
                        log_error!(
                            "IPC router unable to relay ctl message to channel: {:?}",
                            err
                        );
                    })
                    .ok();
            };
        }
    }

    pub fn handle_broadcast_msg(&self, message: MessageEvent) -> Result<()> {
        let data = message.data();
        let data = Object::try_from(&data).ok_or(Error::BroadcastDataNotObject)?;
        let target = data.get("target")?;
        let target = target
            .as_string()
            .ok_or("IPC broadcast message target is not a string")?;
        let kind: BroadcastMessageKind = data.get("type")?.try_into()?;

        match kind {
            BroadcastMessageKind::Request => {
                let id: Id = target.as_str().try_into()?;
                if id != self.inner.id {
                    return Ok(());
                }

                let _origin = data.get("origin")?;
                let _payload = data.get("payload")?;

                todo!();
            }

            BroadcastMessageKind::Response => {
                let id: Id = data.get("id")?.try_into()?;
                if let Some(pending) = self.inner.pending.lock().unwrap().remove(&id) {
                    let payload = data.get("payload")?;
                    if let Err(err) = (pending.callback)(Ok(JsValueSend(payload))) {
                        log_error!("Error while handling IPC response: {}", err);
                    }
                }
            }

            BroadcastMessageKind::Post => {
                match target.as_str() {
                    "*" => {}
                    _ => {
                        let id: Id = target.as_str().try_into()?;
                        if id != self.inner.id {
                            return Ok(());
                        }
                    }
                }
                if self.inner.msg_channel.sender.receiver_count() > 1 {
                    let payload = data.get("payload")?;
                    let msg = Message { payload };
                    self.inner
                        .msg_channel
                        .sender
                        .try_send(msg)
                        .map_err(|err| {
                            log_error!(
                                "IPC router unable to relay broadcast message to channel: {:?}",
                                err
                            );
                        })
                        .ok();
                }
            }
        }

        Ok(())
    }

    pub fn handle_post_with_borsh(&self, _message: JsValue) {
        todo!();
    }

    pub fn handle_post_with_serde(&self, _message: JsValue) {
        todo!();
    }

    pub fn handle_call_with_borsh(&self, _message: JsValue) -> JsValue {
        todo!();
    }

    pub fn handle_call_with_serde(&self, _message: JsValue) -> JsValue {
        todo!();
    }

    pub fn handle_transfer(&self, _message: JsValue) {
        todo!();
    }

    /// post messasge with transfer
    #[allow(dead_code)]
    fn post_with_transfer<T>(&self, destination: &Id, payload: T) -> Result<()>
    where
        T: BorshSerialize,
    {
        let window = self.get(destination)?;
        let (object, buffer) = Self::construct_transferrable(payload)?;
        window.post_message_with_transfer(&object, "*", &buffer)?;
        Ok(())
    }

    /// construct transferrable object for [`post_with_transfer`]
    fn construct_transferrable<T>(payload: T) -> Result<(Object, ArrayBuffer)>
    where
        T: BorshSerialize,
    {
        let payload_vec = payload.try_to_vec()?;
        let payload_array = Uint8Array::from(&payload_vec[..]);
        let buffer = payload_array.buffer();
        let object = Object::new();
        Reflect::set(
            &object,
            &JsValue::from("payload"),
            &JsValue::from(&payload_array),
        )?;
        Ok((object, buffer))
    }

    fn borsh_serialize<T>(data: &T) -> Result<Uint8Array>
    where
        T: BorshSerialize,
    {
        let vec = data.try_to_vec()?;
        let array = Uint8Array::from(&vec[..]);
        Ok(array)
    }

    fn borsh_deserialize<T>(data: Uint8Array) -> Result<T>
    where
        T: BorshDeserialize,
    {
        let vec = data.to_vec();
        let data = T::try_from_slice(&vec)?;
        Ok(data)
    }

    fn post_with_proxy_borsh<Msg>(&self, id: &Id, message: &Msg) -> Result<()>
    where
        Msg: BorshSerialize,
    {
        let handler = self.get_call_handler(id, IPC_PROXY_POST_WITH_BORSH)?;
        let message = Self::borsh_serialize(&message)?;
        handler.call1(&JsValue::NULL, &message)?;
        Ok(())
    }

    fn call_with_proxy_borsh<Req, Resp>(&self, id: &Id, req: &Req) -> Result<Resp>
    where
        Req: BorshSerialize,
        Resp: BorshDeserialize,
    {
        let handler = self.get_call_handler(id, IPC_PROXY_CALL_WITH_BORSH)?;
        let req = Self::borsh_serialize(&req)?;
        let resp = handler.call1(&JsValue::NULL, &req)?;
        let resp = resp.dyn_into::<Uint8Array>()?;
        Self::borsh_deserialize(resp)
    }

    fn post_with_proxy_serde<Msg>(&self, id: &Id, message: &Msg) -> Result<()>
    where
        Msg: Serialize,
    {
        let handler = self.get_call_handler(id, IPC_PROXY_POST_WITH_SERDE)?;
        let message = to_value(message)?;
        handler.call1(&JsValue::NULL, &message)?;
        Ok(())
    }

    fn call_with_proxy_serde<Req, Resp>(&self, id: &Id, req: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        let handler = self.get_call_handler(id, IPC_PROXY_CALL_WITH_SERDE)?;
        let req = to_value(&req)?;
        let resp = handler.call1(&JsValue::NULL, &req)?;
        Ok(from_value(resp)?)
    }

    #[allow(dead_code)]
    fn construct_broadcast_message<Payload>(
        &self,
        kind: BroadcastMessageKind,
        target: Option<&Id>,
        payload: &Payload,
    ) -> Result<Object>
    where
        Payload: Serialize,
    {
        // let kind_jsv : JsValue = kind.into();
        let target = match target {
            Some(target) => to_value(target)?,
            None => JsValue::from("*"),
        };
        // let origin = to_value(self.id())?;
        // let payload = to_value(payload)?;

        let object = Object::new();
        object.set_properties(&[
            ("type", &kind.into()),
            ("target", &target),
            ("origin", &self.id().into()), //&to_value(self.id())?),
            ("payload", &to_value(payload)?),
        ])?;

        Ok(object)
    }

    async fn post_with_broadcast_serde<Msg>(&self, target: Option<&Id>, payload: &Msg) -> Result<()>
    where
        Msg: Serialize,
    {
        let object =
            self.construct_broadcast_message(BroadcastMessageKind::Post, target, payload)?;
        self.inner.broadcast_msg_channel.post_message(&object)?;
        Ok(())
    }

    async fn call_with_broadcast_serde<Req, Resp>(&self, target: &Id, req: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        let object =
            self.construct_broadcast_message(BroadcastMessageKind::Request, Some(target), req)?;

        let id = Id::new();
        object.set_properties(&[("id", &JsValue::from(&id.to_string()))])?;

        let (sender, receiver) = oneshot();

        {
            let mut pending = self.inner.pending.lock().unwrap();
            pending.insert(
                id,
                Pending::new(Arc::new(Box::new(move |result| {
                    sender.try_send(result)?;
                    Ok(())
                }))),
            );
        }

        let resp = receiver.recv().await?.map_err(|err| err.0)?;
        let resp: Resp = from_value(resp.0)?;

        Ok(resp)
    }
}

impl Drop for Router {
    fn drop(&mut self) {
        self.detach();
    }
}

pub mod router {
    use super::*;
    pub static mut ROUTER: Option<(usize, Router)> = None;
    pub fn acquire(meta: JsValue) -> Result<Router> {
        if let Some(r) = unsafe { ROUTER.as_mut() } {
            r.0 += 1;
            Ok(r.1.clone())
        } else {
            let router = Router::try_new(meta)?;
            unsafe { ROUTER = Some((0, router.clone())) };
            Ok(router)
        }
    }
    pub fn release() {
        if let Some(r) = unsafe { ROUTER.as_mut() } {
            r.0 -= 1;
            if r.0 == 0 {
                unsafe { ROUTER = None };
            }
        }
    }
}

pub struct BorshIpc {
    router: Router,
}

impl Drop for BorshIpc {
    fn drop(&mut self) {
        router::release();
    }
}

impl BorshIpc {
    pub fn try_new(meta: JsValue) -> Result<Self> {
        let router = router::acquire(meta)?;
        let ipc = BorshIpc { router };
        Ok(ipc)
    }

    pub async fn post<Msg>(&self, id: &Id, msg: &Msg) -> Result<()>
    where
        Msg: BorshSerialize,
    {
        self.router.post_with_proxy_borsh(id, msg)
    }

    pub fn try_post<Msg>(&self, id: &Id, msg: &Msg) -> Result<()>
    where
        Msg: BorshSerialize,
    {
        self.router.post_with_proxy_borsh(id, msg)
    }

    pub async fn call<Req, Resp>(&self, id: &Id, req: &Req) -> Result<Resp>
    where
        Req: BorshSerialize,
        Resp: BorshDeserialize,
    {
        self.router.call_with_proxy_borsh(id, req)
    }

    pub fn try_call<Req, Resp>(&self, id: &Id, req: &Req) -> Result<Resp>
    where
        Req: BorshSerialize,
        Resp: BorshDeserialize,
    {
        self.router.call_with_proxy_borsh(id, req)
    }
}

pub struct SerdeIpc {
    router: Router,
}

impl Drop for SerdeIpc {
    fn drop(&mut self) {
        router::release();
    }
}

impl SerdeIpc {
    pub fn try_new(meta: JsValue) -> Result<Self> {
        let router = router::acquire(meta)?;
        let ipc = SerdeIpc { router };
        Ok(ipc)
    }

    pub async fn post<Msg>(&self, id: &Id, msg: &Msg) -> Result<()>
    where
        Msg: Serialize,
    {
        self.router.post_with_proxy_serde(id, msg)
    }

    pub fn try_post<Msg>(&self, id: &Id, msg: &Msg) -> Result<()>
    where
        Msg: Serialize,
    {
        self.router.post_with_proxy_serde(id, msg)
    }

    pub async fn call<Req, Resp>(&self, id: &Id, req: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        self.router.call_with_proxy_serde(id, req)
    }

    pub fn try_call<Req, Resp>(&self, id: &Id, req: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        self.router.call_with_proxy_serde(id, req)
    }
}

pub struct BroadcastIpc {
    router: Router,
    handler: Arc<Mutex<Option<Arc<dyn HandlerTrait>>>>,
}

impl Drop for BroadcastIpc {
    fn drop(&mut self) {
        router::release();
    }
}

impl BroadcastIpc {
    pub fn try_new(meta: JsValue) -> Result<Self> {
        let router = router::acquire(meta)?;
        let ipc = BroadcastIpc {
            router,
            handler: Arc::new(Mutex::new(None)),
        };
        Ok(ipc)
    }

    pub async fn post<Msg>(&self, target: Option<&Id>, payload: &Msg) -> Result<()>
    where
        Msg: Serialize,
    {
        self.router.post_with_broadcast_serde(target, payload).await
    }

    pub async fn call<Req, Resp>(&self, target: &Id, req: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        self.router.call_with_broadcast_serde(target, req).await
    }

    pub fn handler<Req, Resp>(&self, handler: RequestFn<Req, Resp>) -> Result<()>
    where
        Req: DeserializeOwned + Send + Sync + 'static,
        Resp: Serialize + Send + Sync + 'static,
    {
        let handler_: Arc<dyn HandlerTrait> = Arc::new(SerdeHandler { handler });
        self.handler.lock().unwrap().replace(handler_);
        Ok(())
    }
}

pub trait HandlerTrait {
    fn handle(&self, req: JsValue) -> Result<JsValue>;
}

pub struct SerdeHandler<Req, Resp>
where
    Req: DeserializeOwned,
    Resp: Serialize,
{
    handler: RequestFn<Req, Resp>,
}

impl<Req, Resp> HandlerTrait for SerdeHandler<Req, Resp>
where
    Req: DeserializeOwned,
    Resp: Serialize,
{
    fn handle(&self, req: JsValue) -> Result<JsValue> {
        let req: Req = from_value(req)?;
        let resp = (self.handler)(req)?;
        Ok(to_value(&resp)?)
    }
}
