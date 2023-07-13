use crate::ipc::imports::*;
use crate::ipc::method::*;
use crate::ipc::notification::*;
use crate::ipc::target::*;

pub type IpcId = Id64;

struct Pending<F> {
    _timestamp: Instant,
    callback: F,
}
impl<F> Pending<F> {
    fn new(callback: F) -> Self {
        Self {
            _timestamp: Instant::now(),
            callback,
        }
    }
}

type PendingMap<Id, F> = Arc<Mutex<AHashMap<Id, Pending<F>>>>;

pub type BorshResponseFn = Arc<
    Box<(dyn Fn(Vec<u8>, ResponseResult<Vec<u8>>, Option<&Duration>) -> Result<()> + Sync + Send)>,
>;

struct Inner<Ops>
where
    Ops: OpsT,
{
    target: IpcTarget,
    identifier: String,
    handler: Mutex<Option<Arc<dyn AsCallback>>>,
    methods: Mutex<AHashMap<Ops, Arc<dyn MethodTrait>>>,
    notifications: Mutex<AHashMap<Ops, Arc<dyn NotificationTrait>>>,
}

pub struct Ipc<Ops>
where
    Ops: OpsT,
{
    inner: Arc<Inner<Ops>>,
    _ops: PhantomData<Ops>,
}

unsafe impl<Ops> Send for Ipc<Ops> where Ops: OpsT {}
unsafe impl<Ops> Sync for Ipc<Ops> where Ops: OpsT {}

impl<Ops> Drop for Ipc<Ops>
where
    Ops: OpsT,
{
    fn drop(&mut self) {
        self.unregister_handler().ok();
    }
}

impl<Ops> Ipc<Ops>
where
    Ops: OpsT,
{
    pub fn try_new_global_binding<Ident>(identifier: Ident) -> Result<Arc<Self>>
    where
        Ident: ToString,
    {
        let target = IpcTarget::new(global::global().as_ref());
        let ipc = Self::try_new_binding(&target, identifier)?;

        unsafe {
            if IPC_HANDLER_SOURCE.is_some() {
                panic!("global ipc handler already registered");
            }
            IPC_HANDLER_SOURCE.replace(target.clone());
        }

        Ok(ipc)
    }

    pub fn try_new_window_binding<Ident>(
        window: &Arc<Window>,
        identifier: Ident,
    ) -> Result<Arc<Self>>
    where
        Ident: ToString,
    {
        let window = window.window();
        let target = IpcTarget::new(window.as_ref());
        Self::try_new_binding(&target, identifier)
    }

    fn try_new_binding<Ident>(target: &IpcTarget, identifier: Ident) -> Result<Arc<Self>>
    where
        Ident: ToString,
    {
        let ipc = Arc::new(Ipc {
            inner: Arc::new(Inner {
                target: target.clone(),
                identifier: identifier.to_string(),
                handler: Mutex::new(None),
                methods: Mutex::new(AHashMap::default()),
                notifications: Mutex::new(AHashMap::default()),
            }),
            _ops: PhantomData,
        });

        ipc.register_handler()?;

        Ok(ipc)
    }

    fn register_handler(self: &Arc<Self>) -> Result<()> {
        let this = self.clone();
        let handler = Arc::new(callback!(move |message: ArrayBuffer, source: JsValue| {
            let this = this.clone();

            let message = Uint8Array::new(&message);
            let vec = message.to_vec();

            let source = if source == JsValue::NULL {
                None
            } else {
                Some(IpcTarget::new(source.as_ref()))
            };

            spawn(async move {
                match BorshMessage::<IpcId>::try_from(&vec) {
                    Ok(message) => {
                        if let Err(err) = this.handle_message(message, source).await {
                            log_error!("IPC: handler error: {:?}", err);
                        }
                    }
                    Err(err) => {
                        log_error!("Failed to deserialize ipc message: {:?}", err);
                    }
                }
            })
        }));

        js_sys::Reflect::set(
            self.inner.target.as_ref(),
            &JsValue::from_str("ipc_handler"),
            handler.get_fn(),
        )?;
        js_sys::Reflect::set(
            self.inner.target.as_ref(),
            &JsValue::from_str("ipc_identifier"),
            &JsValue::from(&self.inner.identifier),
        )?;

        self.inner.handler.lock().unwrap().replace(handler);

        Ok(())
    }

    fn unregister_handler(&self) -> Result<()> {
        if let Some(_handler) = self.inner.handler.lock().unwrap().take() {
            let object = Object::from(self.inner.target.as_ref().clone());
            js_sys::Reflect::delete_property(&object, &JsValue::from_str("ipc_handler"))?;
            js_sys::Reflect::delete_property(&object, &JsValue::from_str("ipc_identifier"))?;
        }

        Ok(())
    }

    pub async fn handle_message<'data>(
        &self,
        message: BorshMessage<'data, IpcId>,
        source: Option<IpcTarget>,
    ) -> Result<()> {
        let BorshMessage::<IpcId> { header, payload } = message;
        let BorshHeader::<IpcId> { op, id, kind } = header;
        match kind {
            MessageKind::Request => {
                let source = source.unwrap_or_else(|| {
                    panic!("ipc received a call request with no source: {:?}", op)
                });

                let op = Ops::try_from_slice(&op)?;

                let method = self.inner.methods.lock().unwrap().get(&op).cloned();
                if let Some(method) = method {
                    let result = method.call_with_borsh(payload).await;
                    let buffer = result.try_to_vec()?;
                    source.call_ipc(
                        to_msg::<Ops, IpcId>(BorshHeader::response(id, op), &buffer)?.as_ref(),
                        None,
                    )?;
                } else {
                    log_error!("ipc method handler not found: {:?}", op);
                    let resp: ResponseResult<()> = Err(ResponseError::NotFound);
                    source.call_ipc(
                        to_msg::<Ops, IpcId>(BorshHeader::response(id, op), &resp.try_to_vec()?)?
                            .as_ref(),
                        None,
                    )?;
                }
            }
            MessageKind::Notification => {
                let op = Ops::try_from_slice(&op)?;

                let notification = self.inner.notifications.lock().unwrap().get(&op).cloned();

                if let Some(notification) = notification {
                    match notification.call_with_borsh(payload).await {
                        Ok(_resp) => {}
                        Err(err) => {
                            log_error!("ipc notification error: {:?}", err);
                        }
                    }
                } else {
                    log_error!("ipc notification handler not found: {:?}", op);
                }
            }
            MessageKind::Response => {
                let id = id.expect("ipc missing success response id");
                // let id = Id64::from(id);
                let mut pending = pending().lock().unwrap();
                if let Some(pending) = pending.remove(&id) {
                    let resp = ResponseResult::<Vec<u8>>::try_from_slice(&payload)?;
                    (pending.callback)(op, resp, None)?;
                } else {
                    log_error!("ipc response id not found: {:?}", id);
                }
            }
        }

        Ok(())
    }

    pub fn method<Req, Resp>(&self, op: Ops, method: Method<Req, Resp>)
    where
        Ops: Debug + Clone,
        Req: MsgT,
        Resp: MsgT,
    {
        let method: Arc<dyn MethodTrait> = Arc::new(method);
        if self
            .inner
            .methods
            .lock()
            .unwrap()
            .insert(op.clone(), method)
            .is_some()
        {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn notification<Msg>(&self, op: Ops, method: Notification<Msg>)
    where
        Ops: Debug + Clone,
        Msg: MsgT,
    {
        let method: Arc<dyn NotificationTrait> = Arc::new(method);
        if self
            .inner
            .notifications
            .lock()
            .unwrap()
            .insert(op.clone(), method)
            .is_some()
        {
            panic!("RPC notification {op:?} is declared multiple times")
        }
    }
}

trait IpcHandler {
    fn call_ipc(&self, data: &JsValue, source: Option<&IpcTarget>) -> Result<()>;
}

impl IpcHandler for IpcTarget {
    fn call_ipc(&self, data: &JsValue, source: Option<&IpcTarget>) -> Result<()> {
        let target_fn = js_sys::Reflect::get(self.as_ref(), &JsValue::from_str("ipc_handler"))?;

        let target_fn = target_fn.clone().unchecked_into::<js_sys::Function>();

        if let Some(source) = source {
            target_fn.call2(
                &JsValue::UNDEFINED,
                &JsValue::from(data),
                &JsValue::from(source.as_ref()),
            )?;
        } else {
            target_fn.call2(&JsValue::UNDEFINED, &JsValue::from(data), &JsValue::NULL)?;
        }

        Ok(())
    }
}

static mut PENDING: Option<PendingMap<IpcId, BorshResponseFn>> = None; //PendingMap::default();
fn pending() -> &'static mut PendingMap<IpcId, BorshResponseFn> {
    unsafe {
        if PENDING.is_none() {
            PENDING = Some(PendingMap::default());
        }
        PENDING.as_mut().unwrap()
    }
}

static mut IPC_HANDLER_SOURCE: Option<IpcTarget> = None;

#[async_trait]
pub trait IpcDispatch {
    fn as_target(&self) -> IpcTarget;

    async fn notify<Ops, Msg>(&self, op: Ops, payload: Msg) -> Result<()>
    where
        Ops: OpsT,
        Msg: BorshSerialize + Send + Sync + 'static,
    {
        let payload = payload.try_to_vec().map_err(|_| Error::BorshSerialize)?;
        self.as_target().call_ipc(
            to_msg::<Ops, IpcId>(BorshHeader::notification::<Ops>(op), &payload)?.as_ref(),
            None,
        )?;
        Ok(())
    }

    async fn call<Ops, Req, Resp>(&self, op: Ops, req: Req) -> Result<Resp>
    where
        Ops: OpsT,
        Req: MsgT,
        Resp: MsgT,
    {
        let source = unsafe {
            IPC_HANDLER_SOURCE
                .as_ref()
                .cloned()
                .expect("missing ipc handler source (please register a local IPC object)")
        };
        self.call_with_source(op, req, &source).await
    }

    async fn call_with_source<Ops, Req, Resp>(
        &self,
        op: Ops,
        req: Req,
        source: &IpcTarget,
    ) -> Result<Resp>
    where
        Ops: OpsT,
        Req: MsgT,
        Resp: MsgT,
    {
        let payload = req.try_to_vec().map_err(|_| Error::BorshSerialize)?;

        let id = Id64::generate();
        let (sender, receiver) = oneshot();

        {
            let mut pending = pending().lock().unwrap();
            pending.insert(
                id.clone(),
                Pending::new(Arc::new(Box::new(move |op, result, _duration| {
                    sender.try_send((op, result.map(|data| data.to_vec())))?;
                    Ok(())
                }))),
            );
        }

        self.as_target().call_ipc(
            to_msg::<Ops, IpcId>(BorshHeader::request::<Ops>(Some(id), op.clone()), &payload)?
                .as_ref(),
            Some(source),
        )?;

        let (op_, data) = receiver.recv().await?;

        let op_ = Ops::try_from_slice(&op_)?;
        if op != op_ {
            return Err(Error::Custom(format!(
                "ipc op mismatch: expected {:?}, got {:?}",
                op, op_
            )));
        }

        let resp = ResponseResult::<Resp>::try_from_slice(data?.as_ref())
            .map_err(|e| Error::BorshDeserialize(e.to_string()))?;

        Ok(resp?)
    }
}

impl IpcDispatch for IpcTarget {
    fn as_target(&self) -> IpcTarget {
        self.clone()
    }
}

impl IpcDispatch for nw_sys::Window {
    fn as_target(&self) -> IpcTarget {
        IpcTarget::new(self.window().as_ref())
    }
}

pub async fn get_ipc_target<Ident>(identifier: Ident) -> crate::result::Result<Option<IpcTarget>>
where
    Ident: ToString,
{
    let ipc_identifier = JsValue::from("ipc_identifier");
    let ident: String = identifier.to_string();

    // let global = js_sys::global();
    let global = global::global();
    let prop = js_sys::Reflect::get(&global, &ipc_identifier)?;
    if let Some(ipc_ident) = prop.as_string() {
        if ipc_ident == ident {
            return Ok(Some(IpcTarget::new(global.as_ref())));
        }
    }

    let windows = crate::window::get_all_async().await?;

    for window in windows.iter() {
        let prop = js_sys::Reflect::get(window.window().as_ref(), &ipc_identifier)?;
        if let Some(ipc_ident) = prop.as_string() {
            if ipc_ident == ident {
                return Ok(Some(IpcTarget::new(window.as_ref())));
            }
        }
    }

    Ok(None)
}
