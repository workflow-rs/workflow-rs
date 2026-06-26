//!
//! [`task`](self) module provides helper functions for use with async closures that *operate uniformly*
//! in native ([`tokio`](https://crates.io/crates/tokio)-backed) and WASM
//! (`wasm-bindgen-futures`-backed) environments
//! (i.e. a web browser).
//!
//! Following functions are are available:
//! - [`spawn()`] - non-blocking spawn of the supplied async closure
//! - [`sleep()`] - suspends the task for a given Duration
//! - [`yield_now()`] - yields rust executor
//! - [`yield_executor()`] - yields to top-level executor (browser async loop)
//!
//! <div class="example-wrap compile_fail"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//! Blocking spawn is not available as a part of this framework as WASM-browser environment can
//! not block task execution due to a single-threaded async application environment.
//! </pre></div>
//!

#[allow(unused_imports)]
use cfg_if::cfg_if;
use futures::Future;

/// Runtime-agnostic cooperative yield, equivalent to the async-std /
/// futures-lite `yield_now`: re-schedules the task to the back of the
/// executor queue, giving other tasks room to progress. No runtime coupling.
#[doc(hidden)]
pub mod __yield {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    pub async fn yield_now() {
        struct YieldNow(bool);
        impl Future for YieldNow {
            type Output = ();
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                if self.0 {
                    Poll::Ready(())
                } else {
                    self.0 = true;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
        YieldNow(false).await
    }
}

cfg_if! {
    if #[cfg(not(any(target_arch = "wasm32", target_arch = "bpf")))] {

        pub mod native {
            //! native implementation
            pub use super::*;

            // yield_executor functionality is browser-specific
            // hence we create a stub in a form of `yield_now()`
            // for native platforms
            pub use tokio::task::yield_now as yield_executor;
            pub use tokio::task::yield_now;
            pub use tokio::time::sleep;
            pub use crate::native::interval::{interval,Interval};

            pub fn spawn<F, T>(future: F)
            where
                F: Future<Output = T> + Send + 'static,
                T: Send + 'static,
            {
                tokio::task::spawn(future);
            }

            pub fn dispatch<F, T>(_future: F)
            where
                F: Future<Output = T> + 'static,
                T: 'static,
            {
                unreachable!()
            }

            pub use workflow_core_macros::call_async_no_send;
        }

        pub use native::*;
    }
}

pub mod wasm {
    //! WASM implementation
    pub use super::*;

    pub fn spawn<F, T>(_future: F)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // wasm32 spawn shim: spawn the future onto the browser event loop.
                // (async-std's wasm `block_on` delegated to
                // `wasm_bindgen_futures::spawn_local` internally; we call it directly.
                // async-std's task-local wrapper is dropped - verified unused across
                // the SDK and its consumers.)
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = _future.await;
                });
            } else {
                panic!("workflow_core::task::wasm::spawn() is not allowed on non-wasm target");
            }
        }
    }

    // `dispatch()` is similar to `spawn()` but does not
    // impose `Send` requirement on the supplied future
    // when building for the `wasm32` target.
    pub fn dispatch<F, T>(_future: F)
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // wasm32 spawn shim: spawn the future onto the browser event loop.
                // (async-std's wasm `block_on` delegated to
                // `wasm_bindgen_futures::spawn_local` internally; we call it directly.
                // async-std's task-local wrapper is dropped - verified unused across
                // the SDK and its consumers.)
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = _future.await;
                });
            } else {
                panic!("workflow_core::task::wasm::spawn() is not allowed on non-wasm target");
            }
        }
    }

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            pub use crate::wasm::{
                overrides::disable_persistent_timer_overrides,
                interval::{interval,Interval},
                yield_executor::{yield_executor,Yield},
                sleep::{sleep,Sleep}
            };
            pub use crate::task::__yield::yield_now;
            pub use workflow_core_macros::call_async_no_send;
        } else {
            pub use crate::native::{
                overrides::disable_persistent_timer_overrides,
                interval::{interval,Interval},
            };
            pub use tokio::time::sleep;
            pub use tokio::task::yield_now;
            pub use workflow_core_macros::call_async_no_send;
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
