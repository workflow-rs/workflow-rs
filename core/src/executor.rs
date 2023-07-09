//!
//! Implements `async fn yield_executor()` that internally calls
//! `requestAnimationFrame`
//!

use std::future::Future;
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context as FutureContext, Poll, Waker},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(callback: js_sys::Function) -> JsValue;
    #[wasm_bindgen(js_name = cancelAnimationFrame)]
    fn cancel_animation_frame(request_id: JsValue);
}

pub use async_std::task::yield_now;

pub async fn yield_executor() {
    if !unsafe { REQUEST_ANIMATION_FRAME_INITIALIZED } {
        init_request_animation_frame_fn();
        unsafe { REQUEST_ANIMATION_FRAME_INITIALIZED = true };
    } else {
        Yield::new().await
    }
}

static mut REQUEST_ANIMATION_FRAME_INITIALIZED: bool = false;

fn init_request_animation_frame_fn() {
    let _ = js_sys::Function::new_no_args(
        "
        if (!this.requestAnimationFrame){
            if (this.setImmediate)
                this.requestAnimationFrame = (callback)=>setImmediate(callback)
            else
                this.requestAnimationFrame = (callback)=>setTimeout(callback, 0)
        }
    ",
    )
    .call0(&JsValue::undefined());
}

struct Context {
    waker: Option<Waker>,
    #[allow(dead_code)]
    instance: JsValue,
    #[allow(dead_code)]
    closure: JsValue,
}

struct Inner {
    ready: Arc<AtomicBool>,
    ctx: Mutex<Option<Context>>,
}

/// `Sleep` future used by the `sleep()` function to provide a
/// timeout future that is backed by the JavaScript `createTimeout()`
/// and `clearTimeout()` APIs. The `Sleep` future is meant only for
/// use in WASM32 browser environments.
#[derive(Clone)]
pub struct Yield {
    inner: Arc<Inner>,
}

unsafe impl Sync for Yield {}
unsafe impl Send for Yield {}

impl Yield {
    /// Create a new `Sleep` future that will resolve after the given duration.
    pub fn new() -> Self {
        let inner = Arc::new(Inner {
            ready: Arc::new(AtomicBool::new(false)),
            ctx: Mutex::new(None),
        });

        let inner_ = inner.clone();
        let closure = Closure::once_into_js(move || {
            inner_.ready.store(true, Ordering::SeqCst);
            if let Some(mut ctx) = inner_.ctx.lock().unwrap().take() {
                if let Some(waker) = ctx.waker.take() {
                    waker.wake();
                }
            }
        });

        let instance = request_animation_frame(closure.clone().into());
        inner.ctx.lock().unwrap().replace(Context {
            waker: None,
            closure,
            instance,
        });

        Yield { inner }
    }

    #[inline]
    fn clear(&self) {
        if let Some(ctx) = self.inner.ctx.lock().unwrap().take() {
            cancel_animation_frame(ctx.instance);
        }
    }

    /// Cancel the current timeout.
    pub fn cancel(&self) {
        self.clear();
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut FutureContext<'_>) -> Poll<Self::Output> {
        match self.inner.ready.load(Ordering::SeqCst) {
            true => {
                self.inner.ctx.lock().unwrap().take();
                Poll::Ready(())
            }
            false => {
                if let Some(ctx) = self.inner.ctx.lock().unwrap().as_mut() {
                    ctx.waker.replace(cx.waker().clone());
                } else {
                    panic!("workflow_core::executor::yield_executor() missing context");
                }
                // self.inner.ctx.lock().unwrap().as_mut().waker.replace(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl Drop for Yield {
    fn drop(&mut self) {
        self.clear();
    }
}
