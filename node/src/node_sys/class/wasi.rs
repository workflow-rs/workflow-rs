use crate::node_sys::interface::WasiOptions;
use js_sys::{Object, WebAssembly};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "wasi")]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = "WASI")]
    #[derive(Clone, Debug)]
    pub type Wasi;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<WasiOptions>) -> Wasi;

    #[wasm_bindgen(method)]
    pub fn start(this: &Wasi, instance: &WebAssembly::Instance);

    #[wasm_bindgen(method, getter, js_name = "wasiImport")]
    pub fn wasi_import(this: &Wasi) -> Object;
}
