//! Lightweight runtime for hosting and gracefully shutting down a set of
//! long-running native services. This crate is only available on non-wasm32
//! targets.

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        pub mod debug;
        /// Error and result types used across the service runtime.
        pub mod error;
        mod imports;
        /// Re-exports of the crate's commonly used types.
        pub mod prelude;
        /// The crate [`Result`](result::Result) alias.
        pub mod result;
        /// The service [`Runtime`](runtime::Runtime) that hosts and supervises services.
        pub mod runtime;
        /// The [`Service`](service::Service) trait implemented by hosted services.
        pub mod service;
        /// OS signal handling for triggering graceful runtime shutdown.
        pub mod signals;

    } else {
        pub mod prelude { }
    }
}
