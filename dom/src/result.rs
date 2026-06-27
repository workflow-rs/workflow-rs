//! Result type used by the [`workflow_dom`](super) module
use wasm_bindgen::JsValue;

/// Result type used by the [`workflow_dom`](super) module
// pub type Result<T> = std::result::Result<T, JsValue>;
pub type JsResult<T> = std::result::Result<T, JsValue>;
/// Result type used by the [`workflow_dom`](super) module, with the error
/// fixed to the crate's [`Error`](crate::error::Error) type.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
