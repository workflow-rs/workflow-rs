use crate::node_sys::{
    class::EventEmitter,
    interface::{AsyncIterator, PipeOptions, WritableStream},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter, extends = AsyncIterator)]
    #[derive(Clone, Debug)]
    pub type ReadableStream;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn is_paused(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn pause(this: &ReadableStream) -> ReadableStream;

    #[wasm_bindgen(method)]
    pub fn pipe(this: &ReadableStream, dest: &WritableStream, opts: PipeOptions) -> bool;

    #[wasm_bindgen(method)]
    pub fn read(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn resume(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn set_encoding(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn shift(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn unpipe(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn unshift(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method)]
    pub fn wrap(this: &ReadableStream) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn readable(this: &ReadableStream) -> bool;

    // FIXME: async iterator
}
