use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(getter, static_method_of = Global, js_class = global, js_name = global)]
    fn get_global() -> Object;
}

pub fn global() -> Object {
    Global::get_global()
}
