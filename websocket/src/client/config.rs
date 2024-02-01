//!
//! WebSocket client configuration options
//!

use super::{error::Error, result::Result};
use js_sys::Object;
use wasm_bindgen::prelude::*;
use workflow_wasm::extensions::object::*;

///
/// Configuration struct for WebSocket client (native Tungstenite and NodeJs connections only)
///
#[derive(Clone, Debug)]
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
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        WebSocketConfig {
            write_buffer_size: 128 * 1024,
            max_write_buffer_size: usize::MAX,
            max_message_size: Some(64 << 20),
            max_frame_size: Some(16 << 20),
            accept_unmasked_frames: false,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_WEBSOCKET_CONFIG: &'static str = r#"

/**
 * `WebSocketConfig` is used to configure the `WebSocket`.
 */
export interface IWebSocketConfig {
    maxReceivedFrameSize: number,
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object, typescript_type = "IWebSocketConfig | undefined")]
    pub type IWebSocketConfig;
}

impl Default for IWebSocketConfig {
    fn default() -> Self {
        //Object::new().dyn_into().expect("UNABLE to convert Object into IWebSocketConfig")//.dyn_into::<IWebSocketConfig>().unwrap()
        wasm_bindgen::JsCast::unchecked_into(Object::new())
    }
}

impl TryFrom<IWebSocketConfig> for WebSocketConfig {
    type Error = Error;
    fn try_from(args: IWebSocketConfig) -> Result<Self> {
        let options = if let Some(args) = args.dyn_ref::<Object>() {
            let max_frame_size = args
                .get_value("maxReceivedFrameSize")?
                .as_f64()
                .map(|f| f as usize)
                .unwrap_or(usize::MAX);
            WebSocketConfig {
                max_frame_size: Some(max_frame_size),
                ..Default::default()
            }
        } else {
            Default::default()
        };
        Ok(options)
    }
}

impl TryFrom<&WebSocketConfig> for IWebSocketConfig {
    type Error = Error;
    fn try_from(config: &WebSocketConfig) -> Result<Self> {
        let js_config: IWebSocketConfig = IWebSocketConfig::default();
        js_config.set(
            "maxReceivedFrameSize",
            &JsValue::from(config.max_frame_size.unwrap_or(usize::MAX)),
        )?;
        Ok(js_config)
    }
}
