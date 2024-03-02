//! Common imports for the `workflow_wasm` crate.
pub use crate::callback::{
    callback, AsCallback, Callback, CallbackClosure, CallbackClosureWithoutResult, CallbackId,
    CallbackMap,
};
pub use crate::convert::{Cast, CastFromJs, TryCastFromJs, TryCastJsInto};
pub use crate::extensions::*;
pub use std::ops::Deref;
pub use workflow_core::sendable::Sendable;
