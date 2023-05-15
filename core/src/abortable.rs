//!
//! Abortable trigger, can be used to cancel (abort) an asyncronous task.
//!

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

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
/// ```
/// let abortable = Abortable::default();
/// let result = my_task(abortable).await?;
/// // ... elsewhere
/// abortable.abort();
/// ```
///
#[derive(Default, Clone)]
pub struct Abortable(Arc<AtomicBool>);

impl Abortable {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    #[inline]
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
}
