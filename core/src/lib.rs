//!
//! [`workflow_core`] is a part of the [`workflow-rs`](https://crates.io/workflow-rs)
//! framework, subset of which is designed to function uniformally across multiple
//! environments including native Rust, WASM-browser and Solana OS targets.
//!
//! This is a general-purpose crate that provides platform-uniform (native and WASM) abstractions for:
//! - async channels
//! - task spawn and sleep functions
//! - random identifiers
//! - async-friendly and threadsafe event triggers
//! - time (Instant and Duration) structs
//! - dynamic async_trait attribute macros
//!

use cfg_if::cfg_if;
extern crate self as workflow_core;

pub mod enums;
pub mod lookup;
pub mod runtime;
pub mod utils;

// pub use workflow_core_macros::describe_enum;
// pub use workflow_core_macros::Describe;
pub use workflow_core_macros::seal;

cfg_if! {
    if #[cfg(not(target_os = "solana"))] {
        // Generic 8-byte identifier
        pub mod id;
        // task re-exports and shims
        pub mod task;
        // channel re-exports and shims
        pub mod channel;

        /// trigger re-exports and shims
        pub mod trigger;

        pub mod time {
            //! re-export of [`instant`] crate supporting native and WASM implementations
            pub use instant::*;
        }

        // /// dynamically configured re-export of async_trait as workflow_async_trait
        // /// that imposes `Send` restriction in native (non-WASM) and removes `Send`
        // /// restriction in WASM builds.
        // #[cfg(target_arch = "wasm32")]
        // pub use workflow_async_trait::async_trait_without_send as workflow_async_trait;
        // /// dynamically configured re-export of async_trait as workflow_async_trait
        // /// that imposes `Send` restriction in native (non-WASM) and removes `Send`
        // /// restriction in WASM builds.
        // #[cfg(not(target_arch = "wasm32"))]
        // pub use workflow_async_trait::async_trait_with_send as workflow_async_trait;

        // /// async_trait that supports (?Send) restriction as a parameter
        // pub use workflow_async_trait::async_trait;
        // /// async_trait that imposes `Send` restriction
        // pub use workflow_async_trait::async_trait_with_send;
        // /// async_trait that ignores `Send` restriction
        // pub use workflow_async_trait::async_trait_without_send;
    }
}
