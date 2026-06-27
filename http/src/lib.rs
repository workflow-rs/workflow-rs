//! Minimal async HTTP client utilities for the workflow-rs workspace,
//! providing simple GET helpers that work on both native and wasm32 targets.

/// Error types returned by this crate.
pub mod error;
/// Result type alias used throughout this crate.
pub mod result;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        pub use wasm::*;
    } else {
        mod native;
        pub use native::*;
    }
}

/// Re-exports the crate's commonly used types for convenient glob import.
pub mod prelude {
    pub use super::*;
}
