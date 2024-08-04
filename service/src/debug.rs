//! global atomic debug flag (for developer testing)

use crate::imports::*;

static DEBUG: AtomicBool = AtomicBool::new(false);

pub fn enable(debug: bool) {
    DEBUG.store(debug, Ordering::SeqCst);
}

pub fn debug() -> bool {
    DEBUG.load(Ordering::SeqCst)
}
