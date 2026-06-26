use crate::node_sys::interface::ReadableStream;
use js_sys::Function;
use wasm_bindgen::{JsCast, prelude::*};

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(extends = ReadableStream)]
    #[derive(Clone, Debug)]
    pub type ReadStream;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, getter, js_name = "bytesRead")]
    pub fn bytes_read(this: &ReadStream) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &ReadStream) -> JsValue; // Buffer | string

    #[wasm_bindgen(method, getter)]
    pub fn pending(this: &ReadStream) -> bool;
}

//******************************//
// Instance Methods (Overloads) //
//******************************//

// Plain Rust convenience wrappers (not `#[wasm_bindgen]`): they call the
// inherited extern methods and `unchecked_into()`. wasm-bindgen >= 0.2.126
// rejects a `#[wasm_bindgen] impl` on an *imported* extern type ("class
// ReadStream referenced by an impl block does not match any exported struct").
impl ReadStream {
    pub fn add_listener_with_open(&self, listener: &Function) -> ReadStream {
        self.add_listener("open", listener).unchecked_into()
    }

    pub fn add_listener_with_close(&self, listener: &Function) -> ReadStream {
        self.add_listener("close", listener).unchecked_into()
    }

    pub fn on_with_open(&self, listener: &Function) -> ReadStream {
        self.on("open", listener).unchecked_into()
    }

    pub fn on_with_close(&self, listener: &Function) -> ReadStream {
        self.on("close", listener).unchecked_into()
    }

    pub fn once_with_open(&self, listener: &Function) -> ReadStream {
        self.once("open", listener).unchecked_into()
    }

    pub fn once_with_close(&self, listener: &Function) -> ReadStream {
        self.once("close", listener).unchecked_into()
    }

    pub fn prepend_listener_with_open(&self, listener: &Function) -> ReadStream {
        self.prepend_listener("open", listener).unchecked_into()
    }

    pub fn prepend_listener_with_close(&self, listener: &Function) -> ReadStream {
        self.prepend_listener("close", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_open(&self, listener: &Function) -> ReadStream {
        self.prepend_once_listener("open", listener)
            .unchecked_into()
    }

    pub fn prepend_once_listener_with_close(&self, listener: &Function) -> ReadStream {
        self.prepend_once_listener("close", listener)
            .unchecked_into()
    }
}
