cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        /// OS signal handling (e.g. graceful shutdown on Ctrl-C / SIGTERM).
        pub mod signals;
        /// Panic hooks that log application panics and trigger shutdown.
        pub mod panic;
    } else {
        // ...
    }
}

/// Repaint-aware async channel types used to communicate with the UI thread.
pub mod channel;
/// Runtime and application event types delivered through the event loop.
pub mod events;
/// Globally-registered, shareable payload slots for passing data between tasks.
pub mod payload;
mod repaint;
/// The [`Service`](service::Service) trait for long-running runtime components.
pub mod service;

#[allow(clippy::module_inception)]
mod runtime;
pub use runtime::*;
