/// re-exports triggered crate as well as
/// two wrappers SingleTrigger and ReqRespTrigger
pub use triggered::*;

// use triggered::{Trigger,Listener};

/// Wrapper containing a single Trigger instance
#[derive(Debug, Clone)]
pub struct SingleTrigger {
    /// Handle used to fire the trigger.
    pub trigger: Trigger,
    /// Handle awaited by consumers waiting for the trigger to fire.
    pub listener: Listener,
}

impl SingleTrigger {
    /// Creates a new trigger/listener pair wrapped in a [`SingleTrigger`].
    pub fn new() -> SingleTrigger {
        let (trigger, listener) = triggered::trigger();
        SingleTrigger { trigger, listener }
    }
}

impl Default for SingleTrigger {
    fn default() -> Self {
        Self::new()
    }
}

/// Bi-directional trigger meant to function in
/// request/response fashion
#[derive(Debug, Clone)]
pub struct ReqRespTrigger {
    /// Trigger fired to signal a request.
    pub request: SingleTrigger,
    /// Trigger fired to signal the corresponding response.
    pub response: SingleTrigger,
}

impl ReqRespTrigger {
    /// Creates a new bi-directional request/response trigger pair.
    pub fn new() -> ReqRespTrigger {
        ReqRespTrigger {
            request: SingleTrigger::new(),
            response: SingleTrigger::new(),
        }
    }
}

impl Default for ReqRespTrigger {
    fn default() -> Self {
        Self::new()
    }
}
