use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type OsConstantsLibuv;

    // FIXME: throws an exception
    #[wasm_bindgen(method, getter)]
    pub fn UV_UDP_REUSEADDR(this: &OsConstantsLibuv) -> i32;
}
