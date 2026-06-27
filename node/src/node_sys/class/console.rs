use crate::node_sys::interface::ConsoleConstructorOptions;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "console")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Console;

    #[wasm_bindgen(constructor)]
    pub fn new(options: ConsoleConstructorOptions) -> Console;
}
