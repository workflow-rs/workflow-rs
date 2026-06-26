use crate::node_sys::class::EventEmitter;
use js_sys::{Function, JsString, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    pub type WritableStream;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    fn end(this: &WritableStream, cb: Option<&Function>);

    #[wasm_bindgen(method, js_name = "end")]
    fn end_with_data(this: &WritableStream, buffer: &Uint8Array, cb: Option<&Function>);

    #[wasm_bindgen(method, js_name = "end")]
    fn end_with_string(
        this: &WritableStream,
        string: &JsString,
        encoding: Option<&JsString>,
        cb: Option<&Function>,
    );

    #[wasm_bindgen(method)]
    fn write_with_data(this: &WritableStream, buffer: &Uint8Array, cb: Option<&Function>) -> bool;

    #[wasm_bindgen(method)]
    fn write_with_string(
        this: &WritableStream,
        string: &JsString,
        encoding: Option<&JsString>,
        cb: Option<&Function>,
    ) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    fn writeable(this: &WritableStream) -> bool;
}
