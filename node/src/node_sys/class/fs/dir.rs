use crate::node_sys::{class::fs::Dirent, interface::AsyncIterator};
use js_sys::{Function, JsString, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = AsyncIterator)]
    #[derive(Clone, Debug)]
    pub type Dir;

    // FIXME: event overloads

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn close(this: &Dir, callback: Option<&Function>) -> Promise;

    #[wasm_bindgen(method, js_name = "closeSync")]
    pub fn close_sync(this: &Dir);

    #[wasm_bindgen(method)]
    pub fn read(this: &Dir) -> Promise;

    #[wasm_bindgen(method, js_name = "readSync")]
    pub fn read_sync(this: &Dir) -> Dirent;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &Dir) -> JsString;
}
