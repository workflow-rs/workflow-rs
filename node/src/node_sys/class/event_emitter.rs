use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type EventEmitter;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EventEmitter;

    #[wasm_bindgen(method, getter, js_name = "defaultMaxListeners")]
    pub fn default_max_listeners(this: &EventEmitter) -> f64;

    #[wasm_bindgen(method, setter, js_name = "defaultMaxListeners")]
    pub fn set_default_max_listeners(this: &EventEmitter, value: f64);

    #[wasm_bindgen(method, js_name = "addListener")]
    pub fn add_listener(this: &EventEmitter, event_name: &str, listener: &Function)
    -> EventEmitter;

    #[wasm_bindgen(method, variadic)]
    pub fn emit(this: &EventEmitter, event_name: &str, args: Box<[JsValue]>) -> bool;

    #[wasm_bindgen(method, js_name = "eventNames")]
    pub fn event_names(this: &EventEmitter) -> Box<[JsValue]>;

    #[wasm_bindgen(method, js_name = "getMaxListeners")]
    pub fn get_max_listeners(this: &EventEmitter);

    #[wasm_bindgen(method, js_name = "listenerCount")]
    pub fn listener_count(this: &EventEmitter) -> f64;

    #[wasm_bindgen(method)]
    pub fn listeners(this: &EventEmitter, event_name: &str) -> Box<[JsValue]>;

    #[wasm_bindgen(method)]
    pub fn off(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "on")]
    pub fn on(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "once")]
    pub fn once(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "prependListener")]
    pub fn prepend_listener(
        this: &EventEmitter,
        event_name: &str,
        listener: &Function,
    ) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "prependOnceListener")]
    pub fn prepend_once_listener(
        this: &EventEmitter,
        event_name: &str,
        listener: &Function,
    ) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "removeAllListeners")]
    pub fn remove_all_listeners(this: &EventEmitter, event_name: Option<&str>) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "removeListener")]
    pub fn remove_listener(
        this: &EventEmitter,
        event_name: &str,
        listener: &Function,
    ) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "setMaxListeners")]
    pub fn set_max_listeners(this: &EventEmitter, n: f64) -> EventEmitter;

    #[wasm_bindgen(method, js_name = "rawListeners")]
    pub fn raw_listeners(this: &EventEmitter, event_name: &str) -> Box<[JsValue]>;
}
