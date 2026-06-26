use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type OsConstantsDlopen;

    // #[wasm_bindgen(method, getter)]
    // pub fn RTLD_DEEPBIND(this: &OsConstantsDlopen) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn RTLD_GLOBAL(this: &OsConstantsDlopen) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn RTLD_LAZY(this: &OsConstantsDlopen) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn RTLD_LOCAL(this: &OsConstantsDlopen) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn RTLD_NOW(this: &OsConstantsDlopen) -> i32;
}
