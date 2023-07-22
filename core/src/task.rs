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
//! <div class="example-wrap compile_fail"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//! Blocking spawn is not available as a part of this framework as WASM-browser environment can
//! not block task execution due to a single-threaded async application environment.
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
                tokio::task::spawn(future);
            }

            pub fn dispatch<F, T>(_future: F)
            where
                F: Future<Output = T> + 'static,
                T: 'static,
            {
                unimplemented!()
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
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // wasm32 spawn shim
                // spawn and spawn_local are currently not available on wasm32 architectures
                // ironically, block_on is but it spawns a task instead of blocking it
                // unfortunately access to [`async_std::task::Builder::local()`] is
                // private.
                async_std::task::block_on(_future);
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
                // wasm32 spawn shim
                // spawn and spawn_local are currently not available on wasm32 architectures
                // ironically, block_on is but it spawns a task instead of blocking it
                // unfortunately access to [`async_std::task::Builder::local()`] is
                // private.
                async_std::task::block_on(_future);
            } else {
                panic!("workflow_core::task::wasm::spawn() is not allowed on non-wasm target");
            }
        }
    }

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            pub use crate::sleep::sleep;
            pub use crate::executor::yield_executor;
            pub use async_std::task::yield_now;
        } else {
            pub use async_std::task::sleep;
            pub use async_std::task::yield_now;
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
