use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = "FSWatcher")]
    #[derive(Clone, Debug)]
    pub type FsWatcher;

    // FIXME: event overloads

    #[wasm_bindgen(method)]
    pub fn close(this: &FsWatcher);
}
