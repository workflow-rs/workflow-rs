use crate::node_sys::class::stream;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ConsoleConstructorOptions {
    stdout: stream::Writable,
    stderr: stream::Writable,
    ignore_errors: Option<bool>,
    color_mod: JsValue,
    inspect_options: Option<Object>,
}

#[wasm_bindgen]
impl ConsoleConstructorOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        stdout: stream::Writable,
        stderr: stream::Writable,
        ignore_errors: Option<bool>,
        color_mod: JsValue,
        inspect_options: Option<Object>,
    ) -> ConsoleConstructorOptions {
        ConsoleConstructorOptions {
            stdout,
            stderr,
            ignore_errors,
            color_mod,
            inspect_options,
        }
    }

    pub fn new(stdout: stream::Writable, stderr: stream::Writable) -> ConsoleConstructorOptions {
        let ignore_errors = Default::default();
        let color_mod = JsValue::UNDEFINED;
        let inspect_options = Default::default();
        ConsoleConstructorOptions::new_with_values(
            stdout,
            stderr,
            ignore_errors,
            color_mod,
            inspect_options,
        )
    }

    #[wasm_bindgen(getter)]
    pub fn stdout(&self) -> stream::Writable {
        self.stdout.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_stdout(&mut self, value: stream::Writable) {
        self.stdout = value;
    }

    #[wasm_bindgen(getter)]
    pub fn stderr(&self) -> stream::Writable {
        self.stderr.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_stderr(&mut self, value: stream::Writable) {
        self.stderr = value;
    }

    #[wasm_bindgen(getter)]
    pub fn ignore_errors(&self) -> Option<bool> {
        self.ignore_errors
    }

    #[wasm_bindgen(setter)]
    pub fn set_ignore_errors(&mut self, value: Option<bool>) {
        self.ignore_errors = value;
    }

    #[wasm_bindgen(getter)]
    pub fn color_mod(&self) -> JsValue {
        self.color_mod.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_color_mod(&mut self, value: JsValue) {
        self.color_mod = value;
    }

    #[wasm_bindgen(getter)]
    pub fn inspect_options(&self) -> Option<Object> {
        self.inspect_options.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_inspect_options(&mut self, value: Option<Object>) {
        self.inspect_options = value;
    }
}
