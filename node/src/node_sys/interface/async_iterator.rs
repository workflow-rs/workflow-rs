use js_sys::Promise;
use wasm_bindgen::{JsCast, prelude::*};

// FIXME: only keep this here until AsyncIterator is available in releae upstream
fn looks_like_iterator(it: &JsValue) -> bool {
    #[wasm_bindgen]
    extern "C" {
        type MaybeIterator;
        #[wasm_bindgen(method, getter)]
        fn next(this: &MaybeIterator) -> JsValue;
    }
    if !it.is_object() {
        return false;
    }
    let it = it.unchecked_ref::<MaybeIterator>();
    it.next().is_function()
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(is_type_of = looks_like_iterator)]
    pub type AsyncIterator;

    #[wasm_bindgen(catch, method, structural)]
    pub fn next(this: &AsyncIterator) -> Result<Promise, JsValue>;
}
