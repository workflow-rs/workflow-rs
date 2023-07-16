use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(getter, static_method_of = Global, js_class = global, js_name = global)]
    fn get_global() -> Object;

    #[wasm_bindgen(getter, catch, static_method_of = Global, js_class = globalThis, js_name = globalThis)]
    fn get_global_this() -> Result<Object, JsValue>;

}

pub fn global() -> Object {
    Global::get_global()
}

pub fn global_this() -> Object {
    Global::get_global()
}
