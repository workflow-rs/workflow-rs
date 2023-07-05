/// Configuration struct for WebSocket client (native Tungstenite connections only)
/// This `WebSocketConfig` is mirrored from Tungstenite, and has no effect when
/// used in the WASM (browser) environment due to lack of control in browser
/// websockets.
#[derive(Default, Clone, Debug)]
pub struct WebSocketConfig {
    /// The size of the send queue. You can use it to turn on/off the backpressure features. `None`
    /// means here that the size of the queue is unlimited. The default value is the unlimited
    /// queue.
    pub max_send_queue: Option<usize>,
    /// The maximum size of a message. `None` means no size limit. The default value is 64 MiB
    /// which should be reasonably big for all normal use-cases but small enough to prevent
    /// memory eating by a malicious user.
    pub max_message_size: Option<usize>,
    /// The maximum size of a single message frame. `None` means no size limit. The limit is for
    /// frame payload NOT including the frame header. The default value is 16 MiB which should
    /// be reasonably big for all normal use-cases but small enough to prevent memory eating
    /// by a malicious user.
    pub max_frame_size: Option<usize>,
    /// When set to `true`, the server will accept and handle unmasked frames
    /// from the client. According to the RFC 6455, the server must close the
    /// connection to the client in such cases, however it seems like there are
    /// some popular libraries that are sending unmasked frames, ignoring the RFC.
    /// By default this option is set to `false`, i.e. according to RFC 6455.
    pub accept_unmasked_frames: bool,
}
