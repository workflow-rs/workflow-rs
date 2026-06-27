//! [`Result`] type used by the `workflow_wasm` crate.

/// Convenience `Result` alias whose error type is the crate's [`crate::error::Error`].
pub type Result<T> = std::result::Result<T, crate::error::Error>;
