use super::error::Error;
use super::result::Result;
use super::Handshake;
use js_sys::Object;
use std::str::FromStr;
use std::sync::Arc;
use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use workflow_core::time::Duration;
use workflow_wasm::extensions::object::*;

#[derive(Default)]
pub struct Options {
    // placeholder for future settings
    // TODO review if it makes sense to impl `reconnect_interval`
    pub receiver_channel_cap: Option<usize>,
    pub sender_channel_cap: Option<usize>,
    pub handshake: Option<Arc<dyn Handshake>>,
}

/// `ConnectionStrategy` specifies how the WebSocket `async fn connect()`
/// function should behave during the first-time connectivity phase.
#[wasm_bindgen]
#[derive(Default, Clone, Debug)]
pub enum ConnectStrategy {
    /// Continuously attempt to connect to the server. This behavior will
    /// block `connect()` function until the connection is established.
    #[default]
    Retry,
    /// Causes `connect()` to return immediately if the first-time connection
    /// has failed.
    Fallback,
}

impl FromStr for ConnectStrategy {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "retry" => Ok(ConnectStrategy::Retry),
            "fallback" => Ok(ConnectStrategy::Fallback),
            _ => Err(Error::InvalidConnectStrategyArg(s.to_string())),
        }
    }
}

impl ConnectStrategy {
    pub fn new(retry: bool) -> Self {
        if retry {
            ConnectStrategy::Retry
        } else {
            ConnectStrategy::Fallback
        }
    }

    pub fn is_fallback(&self) -> bool {
        matches!(self, ConnectStrategy::Fallback)
    }
}

impl TryFrom<JsValue> for ConnectStrategy {
    type Error = Error;
    fn try_from(value: JsValue) -> Result<Self> {
        if value.is_undefined() || value.is_null() {
            Ok(ConnectStrategy::default())
        } else if let Some(string) = value.as_string() {
            Ok(string.parse()?)
        } else {
            Ok(ConnectStrategy::try_from_js_value(value)?)
        }
    }
}

///
/// `ConnectOptions` is used to configure the `WebSocket` connectivity behavior.
///
#[derive(Clone, Debug)]
pub struct ConnectOptions {
    /// Indicates if the `async fn connect()` method should return immediately
    /// or block until the connection is established.
    pub block_async_connect: bool,
    /// [`ConnectStrategy`] used to configure the retry or fallback behavior.
    pub strategy: ConnectStrategy,
    /// Optional `url` that will change the current URL of the WebSocket.
    pub url: Option<String>,
    /// Optional `timeout` that will change the timeout of the WebSocket connection process.
    /// `Timeout` is the period after which the async connection attempt is aborted. `Timeout`
    /// is followed by the retry delay if the [`ConnectionStrategy`] is set to `Retry`.
    pub connect_timeout: Option<Duration>,
    /// Retry interval denotes the time to wait before attempting to reconnect.
    pub retry_interval: Option<Duration>,
}

pub const DEFAULT_CONNECT_TIMEOUT_MILLIS: u64 = 5_000;
pub const DEFAULT_CONNECT_RETRY_MILLIS: u64 = 5_000;

impl Default for ConnectOptions {
    fn default() -> Self {
        Self {
            block_async_connect: true,
            strategy: ConnectStrategy::Retry,
            url: None,
            connect_timeout: None,
            retry_interval: None,
        }
    }
}

impl ConnectOptions {
    pub fn fallback() -> Self {
        Self {
            block_async_connect: true,
            strategy: ConnectStrategy::Fallback,
            url: None,
            connect_timeout: None,
            retry_interval: None,
        }
    }
    pub fn reconnect_defaults() -> Self {
        Self {
            block_async_connect: true,
            strategy: ConnectStrategy::Retry,
            url: None,
            connect_timeout: None,
            retry_interval: None,
        }
    }

    pub fn passive_retry_with_defaults() -> Self {
        Self {
            block_async_connect: false,
            strategy: ConnectStrategy::Retry,
            url: None,
            connect_timeout: None,
            retry_interval: None,
        }
    }

    pub fn connect_timeout(&self) -> Duration {
        self.connect_timeout
            .unwrap_or(Duration::from_millis(DEFAULT_CONNECT_TIMEOUT_MILLIS))
    }

    pub fn retry_interval(&self) -> Duration {
        self.retry_interval
            .unwrap_or(Duration::from_millis(DEFAULT_CONNECT_RETRY_MILLIS))
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_CONNECT_OPTIONS: &'static str = r#"

/**
 * `ConnectOptions` is used to configure the `WebSocket` connectivity behavior.
 */
export interface IConnectOptions {
    // Indicates if the `async fn connect()` method should return immediately
    // or wait for connection to occur or fail before returning.
    blockAsyncConnect : boolean,
    // ConnectStrategy used to configure the retry or fallback behavior.
    // In retry mode, the WebSocket will continuously attempt to connect to the server.
    strategy: ConnectStrategy | string | undefined,
    // A custom URL that will change the current URL of the WebSocket.
    url: string | undefined,
    // A custom connection timeout in milliseconds.
    timeoutDurationMsec: number,
    // A custom retry interval in milliseconds.
    retryIntervalMsec: number,
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IConnectOptions | undefined")]
    pub type IConnectOptions;
}

impl TryFrom<&IConnectOptions> for ConnectOptions {
    type Error = Error;
    fn try_from(args: &IConnectOptions) -> Result<Self> {
        let options = if let Some(args) = args.dyn_ref::<Object>() {
            let url = args.get_value("url")?.as_string();
            let block_async_connect = args
                .get_value("blockAsyncConnect")?
                .as_bool()
                .unwrap_or(true);
            let strategy = ConnectStrategy::try_from(args.get_value("strategy")?)?;
            let timeout = args
                .get_value("timeoutDurationMsec")?
                .as_f64()
                .map(|f| Duration::from_millis(f as u64));
            let retry_interval = args
                .get_value("retryIntervalMsec")?
                .as_f64()
                .map(|f| Duration::from_millis(f as u64));

            ConnectOptions {
                block_async_connect,
                strategy,
                url,
                connect_timeout: timeout,
                retry_interval,
            }
        } else if let Some(retry) = args.as_bool() {
            ConnectOptions {
                block_async_connect: true,
                strategy: ConnectStrategy::new(retry),
                url: None,
                connect_timeout: None,
                retry_interval: None,
            }
        } else {
            ConnectOptions::default()
        };

        Ok(options)
    }
}
