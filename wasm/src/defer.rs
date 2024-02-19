//! The [`defer`] utility function.

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(not(feature = "no-unsafe-eval"))] {

        /// Deferred promise - an object that has `resolve()` and `reject()`
        /// functions that can be called outside of the promise body.
        /// WARNING: This function uses `eval` and can not be used in environments
        /// where dynamically-created code can not be executed such as web browser
        /// extensions.
        /// @category General
        #[wasm_bindgen]
        pub fn defer() -> js_sys::Promise {
            js_sys::Function::new_no_args(
                r###"
                let resolve, reject;
                const p = new Promise((resolve_, reject_) => {
                    resolve = resolve_;
                    reject = reject_;
                });
                p.resolve = resolve;
                p.reject = reject;
                return p;
            "###,
            )
            .call0(&JsValue::undefined())
            .unwrap()
            .into()
        }
    }
}
