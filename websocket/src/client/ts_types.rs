use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(typescript_custom_section)]
const IConnectOptions: &'static str = r#"

/**
 * Each Duration is composed of a whole number of seconds and a 
 * fractional part represented in nanoseconds. 
 * If the underlying system does not support nanosecond-level precision, 
 * APIs binding a system timeout will typically round up the number of 
 * nanoseconds.
 */
export interface Duration {
    secs:number
    nanos:number
}

/**
 * Retry: Continuously attempt to connect to the server. This behavior will
 * block `connect()` function until the connection is established.
 * Fallback: Causes `connect()` to return immediately if the first-time connection
 * has failed.
 */
export enum ConnectStrategy {
    Retry,
    Fallback
}

/**
 * `ConnectOptions` is used to configure the `WebSocket` connectivity behavior.
 */
export interface IConnectOptions {
    strategy: ConnectStrategy
    url: string
    connect_timeout: Duration
    retry_interval: Duration
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IConnectOptions | undefined")]
    pub type IConnectOptionsOrUndefined;
}
