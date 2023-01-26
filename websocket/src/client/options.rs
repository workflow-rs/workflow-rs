use super::Handshake;
use std::sync::Arc;

#[derive(Default)]
pub struct Options {
    // placeholder for future settings
    // TODO review if it makes sense to impl `reconnect_interval`
    pub receiver_channel_cap: Option<usize>,
    pub sender_channel_cap: Option<usize>,
    pub handshake: Option<Arc<dyn Handshake>>,
}
