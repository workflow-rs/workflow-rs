use crate::node_sys::class::stream;
use js_sys::{Function, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "http")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ClientRequest;

    // FIXME: event overloads

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn abort(this: &ClientRequest);

    #[wasm_bindgen(method)]
    pub fn end(
        this: &ClientRequest,
        data: JsValue,
        encoding: Option<&JsString>,
        callback: Option<&Function>,
    ) -> ClientRequest;

    #[wasm_bindgen(method, js_name = "flushHeaders")]
    pub fn flush_headers(this: &ClientRequest);

    #[wasm_bindgen(method, js_name = "getHeader")]
    pub fn get_header(this: &ClientRequest, name: &JsString) -> JsValue;

    #[wasm_bindgen(method, js_name = "removeHeader")]
    pub fn remove_header(this: &ClientRequest, name: &JsString);

    #[wasm_bindgen(method, js_name = "setHeader")]
    pub fn set_header(this: &ClientRequest, name: &JsString, value: &JsValue);

    #[wasm_bindgen(method)]
    pub fn set_no_delay(this: &ClientRequest, no_delay: Option<bool>);

    #[wasm_bindgen(method)]
    pub fn set_socket_keep_alive(
        this: &ClientRequest,
        enable: Option<bool>,
        initial_delay: Option<f64>,
    );

    #[wasm_bindgen(method)]
    pub fn set_timeout(this: &ClientRequest, callback: Option<&Function>) -> ClientRequest;

    #[wasm_bindgen(method)]
    pub fn write(
        this: &ClientRequest,
        chunk: &JsValue,
        encoding: Option<&JsString>,
        callback: Option<&Function>,
    ) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn aborted(this: &ClientRequest) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn finished(this: &ClientRequest) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn max_headers_count(this: &ClientRequest) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &ClientRequest) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn reused_socket(this: &ClientRequest) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn socket(this: &ClientRequest) -> stream::Duplex;

    #[wasm_bindgen(method, getter)]
    pub fn writable_ended(this: &ClientRequest) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn writable_finished(this: &ClientRequest) -> bool;
}
