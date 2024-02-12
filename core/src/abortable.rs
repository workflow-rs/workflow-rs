//!
//! Abortable trigger, can be used to cancel (abort) an asynchronous task.
//!
use wasm_bindgen::prelude::*;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// @category General
#[wasm_bindgen]
pub struct Aborted;

impl std::error::Error for Aborted {}

impl std::fmt::Debug for Aborted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "task aborted")
    }
}

impl std::fmt::Display for Aborted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "task aborted")
    }
}

///
/// Abortable trigger wraps an `Arc<AtomicBool>`, which can be cloned
/// to signal task terminating using an atomic bool.
///
/// ```text
/// let abortable = Abortable::default();
/// let result = my_task(abortable).await?;
/// // ... elsewhere
/// abortable.abort();
/// ```
///
/// @category General
#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Abortable(Arc<AtomicBool>);

#[wasm_bindgen]
impl Abortable {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    #[inline]
    #[wasm_bindgen(js_name=isAborted)]
    pub fn is_aborted(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn abort(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn check(&self) -> Result<(), Aborted> {
        if self.is_aborted() {
            Err(Aborted)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn reset(&self) {
        self.0.store(false, Ordering::SeqCst);
    }
}
