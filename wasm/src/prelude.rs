//! Common imports for the `workflow_wasm` crate.
pub use crate::abi::ref_from_abi;
pub use crate::callback::{
    callback, AsCallback, Callback, CallbackClosure, CallbackClosureWithoutResult, CallbackId,
    CallbackMap,
};
pub use crate::extensions::*;
pub use workflow_core::sendable::Sendable;
