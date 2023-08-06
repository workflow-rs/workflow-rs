//! Common imports for the `workflow_wasm` crate.
pub use crate::callback::{
    callback, AsCallback, Callback, CallbackClosure, CallbackClosureWithoutResult, CallbackId,
    CallbackMap,
};
pub use crate::object::ObjectTrait;
pub use workflow_core::sendable::Sendable;
