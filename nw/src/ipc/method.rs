//! Module containing IPC [`Method`] closure wrappers
use crate::ipc::imports::*;

/// Base trait representing an IPC method, used to retain
/// method structures in IPC map without generics.
#[async_trait]
pub(crate) trait MethodTrait: Send + Sync + 'static {
    async fn call_with_borsh(&self, data: &[u8]) -> ResponseResult<Vec<u8>>;
}

/// IPC method function type
pub type MethodFn<Req, Resp> =
    Arc<Box<dyn Send + Sync + Fn(Req) -> MethodFnReturn<Resp> + 'static>>;

/// IPC method function return type
pub type MethodFnReturn<T> = Pin<Box<(dyn Send + 'static + Future<Output = ResponseResult<T>>)>>;

/// IPC method wrapper. Contains the method closure function.
pub struct Method<Req, Resp>
where
    Req: MsgT,
    Resp: MsgT,
{
    method: MethodFn<Req, Resp>,
}

impl<Req, Resp> Method<Req, Resp>
where
    Req: MsgT,
    Resp: MsgT,
{
    pub fn new<FN>(method_fn: FN) -> Method<Req, Resp>
    where
        FN: Send + Sync + Fn(Req) -> MethodFnReturn<Resp> + 'static,
    {
        Method {
            method: Arc::new(Box::new(method_fn)),
        }
    }
}

#[async_trait]
impl<Req, Resp> MethodTrait for Method<Req, Resp>
where
    Req: MsgT,
    Resp: MsgT,
{
    async fn call_with_borsh(&self, data: &[u8]) -> ResponseResult<Vec<u8>> {
        let req = Req::try_from_slice(data)?;
        let resp = (self.method)(req).await;
        let vec = <ResponseResult<Resp> as BorshSerialize>::try_to_vec(&resp)?;
        Ok(vec)
    }
}
