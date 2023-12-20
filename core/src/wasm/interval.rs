//!
//! `Interval` stream backed by the JavaScript `setInterval()` and `clearInterval()` APIs.
//!

#![allow(dead_code)]

use futures::{task::AtomicWaker, Stream};
use instant::Duration;
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (catch, js_name = setInterval)]
    pub fn set_interval(
        closure: &Closure<dyn FnMut()>,
        timeout: u32,
    ) -> std::result::Result<JsValue, JsValue>;
    #[wasm_bindgen (catch, js_name = clearInterval)]
    pub fn clear_interval(interval: &JsValue) -> std::result::Result<(), JsValue>;
}

type IntervalClosure = Closure<dyn FnMut()>;

struct IntervalContext {
    period: Duration,
    instance: JsValue,
    // this closue, while not read
    // must be retained for the lifetime
    // of this context.
    #[allow(dead_code)]
    closure: IntervalClosure,
}

unsafe impl Sync for IntervalContext {}
unsafe impl Send for IntervalContext {}

struct Inner {
    ready: AtomicBool,
    waker: AtomicWaker,
    ctx: Mutex<Option<IntervalContext>>,
}

/// 
/// `Interval` stream used by the `interval()` function to provide a
/// a time interval stream. The stream is backed by tokio interval 
/// stream on native platforms and by by the JavaScript `setInterval()`
/// and `clearInterval()` APIs in WASM32 environment. 
/// 
/// This Interval stream has an advantage of having `Send` and `Sync` markers.
/// 
/// Please note that the `Interval` fires upon creation to mimic
/// the tokio-backed Interval stream available on the native target.
/// 
#[derive(Clone)]
pub struct Interval {
    inner: Arc<Inner>,
}

impl Interval {
    /// Create a new `Interval` stream that will resolve each given duration.
    pub fn new(period: Duration) -> Self {
        let inner = Arc::new(Inner {
            // Interval is made to fire immediately
            // to mimic the behavior of tokio interval.
            ready: AtomicBool::new(true),
            ctx: Mutex::new(None),
            waker: AtomicWaker::new(),
        });

        let inner_ = inner.clone();
        let closure = Closure::new(move || {
            inner_.ready.store(true, Ordering::SeqCst);
            if let Some(waker) = inner_.waker.take() {
                waker.wake();
            }
        });

        let instance = set_interval(&closure, period.as_millis() as u32).unwrap();

        inner.ctx.lock().unwrap().replace(IntervalContext {
            period,
            instance,
            closure,
        });

        Interval { inner }
    }

    /// Obtain the current interval period
    #[inline]
    pub fn period(&self) -> Duration {
        self.inner.ctx.lock().unwrap().as_ref().unwrap().period
    }

    /// Change period function will result in immediate cancellation of the underlying
    /// timer and a restart of the timer starting from the moment of [`change_period()`] invocation.
    #[inline]
    pub fn change_period(&self, period: Duration) {
        if let Some(ctx) = self.inner.ctx.lock().unwrap().as_mut() {
            clear_interval(ctx.instance.as_ref()).unwrap();
            let instance = set_interval(&ctx.closure, period.as_millis() as u32).unwrap();
            ctx.instance = instance;
        }
    }

    #[inline]
    fn clear(&self) {
        if let Some(ctx) = self.inner.ctx.lock().unwrap().take() {
            clear_interval(ctx.instance.as_ref()).unwrap();
        }
    }

    /// Cancel the current timeout.
    pub fn cancel(&self) {
        self.clear();
    }
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.ready.load(Ordering::SeqCst) {
            true => {
                self.inner.ready.store(false, Ordering::SeqCst);
                Poll::Ready(Some(()))
            }
            false => {
                self.inner.waker.register(cx.waker());

                // this will not occur in a single-threaded context
                // but just being safe in case in the future the
                // functionality changes
                if self.inner.ready.load(Ordering::SeqCst) {
                    self.inner.ready.store(false, Ordering::SeqCst);
                    Poll::Ready(Some(()))
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        self.clear();
    }
}

/// `async interval()` function backed by the JavaScript `createInterval()`
pub fn interval(duration: Duration) -> Interval {
    Interval::new(duration)
}
