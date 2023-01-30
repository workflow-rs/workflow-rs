//!
//! [`task`](self) module provides helper functions for use with async closures that *operate uniformly*
//! in native ([`tokio`](https://crates.io/crates/tokio)-backed) and WASM ([`async_std`]-backed) environments
//! (i.e. a web browser).
//!
//! Following functions are are available:
//! - [`spawn()`] - non-blocking spawn of the supplied async closure
//! - [`sleep()`] - suspends the task for a given Duration
//! - [`yield_now()`] - yields rust executor
//! - [`yield_executor()`] - yields to top-level executor (browser async loop)
//!
//! A [`Task`](workflow_task::Task) struct is also available and allows spawning an async closures while
//! providing it with an argument, a return value and a channel that signals termination.
//! Once started, the task can be externally terminated and/or waited until completion.
//!
//! <div class="example-wrap compile_fail"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//! Blocking spawn is not available as browser-WASM can
//! not block task execution due to a single-threaded async environment.
//! </pre></div>
//!

#[allow(unused_imports)]
use cfg_if::cfg_if;
use futures::Future;

cfg_if! {
    if #[cfg(not(any(target_arch = "wasm32", target_os = "solana")))] {

        pub mod native {
            //! native implementation
            pub use super::*;

            pub use tokio::task::yield_now;
            pub use tokio::task::yield_now as yield_executor;
            pub use tokio::time::sleep;

            pub fn spawn<F, T>(future: F)
            where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
            {
                tokio::task::spawn(async {
                // async_std::task::spawn(async {
                    future.await
                });
            }
        }

        pub use native::*;
    }
}

pub mod wasm {
    //! WASM implementation
    pub use super::*;

    pub fn spawn<F, T>(_future: F)
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // wasm32 spawn shim
                // spawn and spawn_local are currently not available on wasm32 architectures
                // ironically, block_on is but it spawns a task instead of blocking it
                // unfortunately access to [`async_std::task::Builder::local()`] is
                // private.
                async_std::task::block_on(async move { _future.await });
                // async_std::task::Builder::new().local(_future).unwrap();
            } else {
                panic!("workflow_core::task::wasm::spawn() is not allowed on non-wasm target");
            }
        }
    }

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use std::sync::{Arc, Mutex};
            use wasm_bindgen::prelude::*;
            use instant::Duration;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen (catch, js_name = setTimeout)]
                pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout: u32) -> std::result::Result<u32, JsValue>;
                #[wasm_bindgen (catch, js_name = clearTimeout)]
                pub fn clear_timeout(interval: u32) -> std::result::Result<(), JsValue>;
                #[wasm_bindgen(js_name = requestAnimationFrame)]
                fn request_animation_frame(callback:js_sys::Function);
            }

            type SleepClosure = Closure<dyn FnMut()>;
            /// Suspends current task for the given [`Duration`]
            pub async fn sleep(duration : Duration) {
                let (sender, receiver) = crate::channel::oneshot::<()>();
                let interval = {
                    let mutex_init : Arc<Mutex<Option<SleepClosure>>> = Arc::new(Mutex::new(None));
                    let mutex_clear = mutex_init.clone();
                    let closure = Closure::new(move ||{
                        sender.try_send(()).unwrap();
                        *mutex_clear.clone().lock().unwrap() = None;
                    });
                    let interval = set_timeout(&closure, duration.as_millis() as u32).unwrap();
                    *mutex_init.lock().unwrap() = Some(closure);
                    interval
                };
                receiver.recv().await.unwrap();
                clear_timeout(interval).unwrap();
            }

            pub use async_std::task::yield_now;
            pub async fn yield_executor() {
                if !unsafe { REQUEST_ANIMATION_FRAME_INITIALIZED } {
                    init_yield();
                    unsafe { REQUEST_ANIMATION_FRAME_INITIALIZED = true };
                } else {
                    let promise = js_sys::Promise::new(&mut |res, _|{
                        request_animation_frame(res);
                    });
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                }
            }

            static mut REQUEST_ANIMATION_FRAME_INITIALIZED: bool = false;

            fn init_yield(){
                let _ = js_sys::Function::new_no_args("
                    if (!this.requestAnimationFrame){
                        if (this.setImmediate)
                            this.requestAnimationFrame = (callback)=>setImmediate(callback)
                        else
                            this.requestAnimationFrame = (callback)=>setTimeout(callback, 0)
                    }
                ")
                .call0(&JsValue::undefined());
            }

        } else {
            pub use async_std::task::sleep;
            pub use async_std::task::yield_now;
        }
    }
}
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
