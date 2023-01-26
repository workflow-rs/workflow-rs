pub use crate::callback::{
    callback, AsCallback, Callback, CallbackClosure, CallbackClosureWithoutResult, CallbackId,
    CallbackMap,
};

pub use crate::timers::{
    clear_interval, clear_timeout, set_interval, set_timeout, IntervalHandle, TimeoutHandle,
};
