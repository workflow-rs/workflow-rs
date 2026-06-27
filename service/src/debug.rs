//! global atomic debug flag (for developer testing)

use crate::imports::*;

static DEBUG: AtomicBool = AtomicBool::new(false);

/// Enables or disables the global developer debug flag, which causes the
/// runtime to print service lifecycle events.
pub fn enable(debug: bool) {
    DEBUG.store(debug, Ordering::SeqCst);
}

/// Returns whether the global developer debug flag is currently enabled.
pub fn debug() -> bool {
    DEBUG.load(Ordering::SeqCst)
}
