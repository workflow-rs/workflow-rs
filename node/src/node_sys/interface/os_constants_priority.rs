use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type OsConstantsPriority;

    #[wasm_bindgen(method, getter)]
    pub fn PRIORITY_LOW(this: &OsConstantsPriority) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn PRIORITY_BELOW_NORMAL(this: &OsConstantsPriority) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn PRIORITY_NORMAL(this: &OsConstantsPriority) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn PRIORITY_ABOVE_NORMAL(this: &OsConstantsPriority) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn PRIORITY_HIGH(this: &OsConstantsPriority) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn PRIORITY_HIGHEST(this: &OsConstantsPriority) -> i32;
}
