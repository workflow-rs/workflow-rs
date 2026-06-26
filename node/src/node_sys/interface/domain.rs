use crate::node_sys::{class::EventEmitter, interface::Timer};
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type Domain;

    #[wasm_bindgen(method, js_name = "add")]
    pub fn add_with_emitter(this: &Domain, emitter: &EventEmitter);

    #[wasm_bindgen(method, js_name = "add")]
    pub fn add_with_timer(this: &Domain, timer: &Timer);

    #[wasm_bindgen(method)]
    pub fn bind(this: &Domain, callback: &Function) -> Function;

    #[wasm_bindgen(method)]
    pub fn intercept(this: &Domain, callback: &Function) -> Function;

    #[wasm_bindgen(method)]
    pub fn remove_with_emitter(this: &Domain, emitter: &EventEmitter);

    #[wasm_bindgen(method)]
    pub fn remove_with_timer(this: &Domain, timer: &Timer);

    #[wasm_bindgen(method, variadic)]
    pub fn run(this: &Domain, callback: &Function, args: Box<[JsValue]>) -> JsValue;
}
