//!
//! WebSocket client configuration options
//!

use super::{error::Error, result::Result, Handshake, Resolver};
use cfg_if::cfg_if;
use js_sys::Object;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use workflow_wasm::extensions::object::*;

///
/// Configuration struct for WebSocket client (native Tungstenite and NodeJs connections only)
///
#[derive(Clone)]
pub struct WebSocketConfig {
    /// The target minimum size of the write buffer to reach before writing the data
    /// to the underlying stream.
    /// The default value is 128 KiB.
    ///
    /// If set to `0` each message will be eagerly written to the underlying stream.
    /// It is often more optimal to allow them to buffer a little, hence the default value.
    ///
    /// Note: [`flush`](WebSocket::flush) will always fully write the buffer regardless.
    pub write_buffer_size: usize,
    /// The max size of the write buffer in bytes. Setting this can provide backpressure
    /// in the case the write buffer is filling up due to write errors.
    /// The default value is unlimited.
    ///
    /// Note: The write buffer only builds up past [`write_buffer_size`](Self::write_buffer_size)
    /// when writes to the underlying stream are failing. So the **write buffer can not
    /// fill up if you are not observing write errors even if not flushing**.
    ///
    /// Note: Should always be at least [`write_buffer_size + 1 message`](Self::write_buffer_size)
    /// and probably a little more depending on error handling strategy.
    pub max_write_buffer_size: usize,
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
    /// The capacity of the channel used to queue incoming messages from WebSocket.
    pub receiver_channel_cap: Option<usize>,
    /// The capacity of the channel used to queue outgoing messages to WebSocket.
    pub sender_channel_cap: Option<usize>,
    /// Handshake handler for WebSocket connections. If supplied, it will be called
    /// when the connection is established. The handshake handler can be used to
    /// perform additional validation or setup before the connection is used.
    pub handshake: Option<Arc<dyn Handshake>>,
    /// Resolver for WebSocket connections. If supplied, it will be called to resolve
    /// the URL before the connection is established. The resolver can be used as
    /// an alternative to supplying the URL and will be invoked each time the
    /// websocket needs to be connected or reconnected.
    pub resolver: Option<Arc<dyn Resolver>>,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        WebSocketConfig {
            write_buffer_size: 128 * 1024,
            max_write_buffer_size: usize::MAX,
            max_message_size: Some(64 << 20),
            max_frame_size: Some(16 << 20),
            accept_unmasked_frames: false,
            receiver_channel_cap: None,
            sender_channel_cap: None,
            handshake: None,
            resolver: None,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "wasm32-sdk")] {

        #[wasm_bindgen(typescript_custom_section)]
        const TS_WEBSOCKET_CONFIG: &'static str = r#"

        /**
         * `WebSocketConfig` is used to configure the `WebSocket`.
         * 
         * @category WebSocket
         */
        export interface IWebSocketConfig {
            /** Maximum size of the WebSocket message. */
            maxMessageSize: number,
            /** Maximum size of the WebSocket frame. */
            maxFrameSize: number,
        }
        "#;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(extends = js_sys::Object, typescript_type = "IWebSocketConfig | undefined")]
            pub type IWebSocketConfig;
        }

        impl TryFrom<IWebSocketConfig> for WebSocketConfig {
            type Error = Error;
            fn try_from(args: IWebSocketConfig) -> Result<Self> {
                let config = if let Some(args) = args.dyn_ref::<Object>() {
                    let mut config = WebSocketConfig::default();
                    if let Some(max_frame_size) = args.get_value("maxFrameSize")?.as_f64() {
                        config.max_frame_size = Some(max_frame_size as usize);
                    }
                    if let Some(max_message_size) = args.get_value("maxMessageSize")?.as_f64() {
                        config.max_message_size = Some(max_message_size as usize);
                    }
                    config
                } else {
                    Default::default()
                };
                Ok(config)
            }
        }
    }
}

pub(crate) struct WebSocketNodeJsConfig {
    pub protocols: JsValue,
    pub origin: JsValue,
    pub headers: JsValue,
    pub request_options: JsValue,
    pub client_config: JsValue,
}

impl Default for WebSocketNodeJsConfig {
    fn default() -> Self {
        Self {
            protocols: JsValue::UNDEFINED,
            origin: JsValue::UNDEFINED,
            headers: JsValue::UNDEFINED,
            request_options: JsValue::UNDEFINED,
            client_config: JsValue::UNDEFINED,
        }
    }
}

impl TryFrom<&WebSocketConfig> for WebSocketNodeJsConfig {
    type Error = Error;
    fn try_from(config: &WebSocketConfig) -> Result<Self> {
        let client_config = Object::new();
        if let Some(max_frame_size) = config.max_frame_size {
            client_config.set("maxReceivedFrameSize", &JsValue::from(max_frame_size))?;
        }
        if let Some(max_message_size) = config.max_message_size {
            client_config.set("maxReceivedMessageSize", &JsValue::from(max_message_size))?;
        }

        let nodejs_config = WebSocketNodeJsConfig {
            protocols: JsValue::UNDEFINED,
            origin: JsValue::UNDEFINED,
            headers: JsValue::UNDEFINED,
            request_options: JsValue::UNDEFINED,
            client_config: client_config.into(),
        };

        Ok(nodejs_config)
    }
}
