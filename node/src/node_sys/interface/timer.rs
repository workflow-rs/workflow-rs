use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Timer;

    #[wasm_bindgen(method)]
    pub fn has_ref(this: &Timer) -> bool;

    #[wasm_bindgen(method)]
    pub fn r#ref(this: &Timer) -> Timer;

    #[wasm_bindgen(method)]
    pub fn refresh(this: &Timer) -> Timer;

    #[wasm_bindgen(method)]
    pub fn unref(this: &Timer) -> Timer;
}
