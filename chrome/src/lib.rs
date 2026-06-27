//! Rust integration for Chrome extension APIs, providing async wrappers
//! around the `chrome.*` JavaScript interfaces (currently extension storage).

/// Error types produced by the crate.
pub mod error;
/// Crate-specific [`Result`](result::Result) alias.
pub mod result;
/// Async wrappers around the Chrome extension `storage` API.
pub mod storage;
