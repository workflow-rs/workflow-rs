use js_sys::Function;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

/// Deferred promise - an object that has `resolve()` and `reject()`
/// functions that can be called outside of the promise body.
#[wasm_bindgen]
pub fn defer() -> Promise {
    Function::new_no_args(r###"
        let resolve, reject;
        const p = new Promise((resolve_, reject_) => {
            resolve = resolve_;
            reject = reject_;
        });
        p.resolve = resolve;
        p.reject = reject;
        return p;
    "###).call0(&JsValue::undefined()).unwrap().into()
}