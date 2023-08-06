//! [`Result`] type used by the `workflow_wasm` crate.

pub type Result<T> = std::result::Result<T, crate::error::Error>;
