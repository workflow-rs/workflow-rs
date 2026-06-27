use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct AssertionErrorOptions {
    message: Option<JsString>,
    actual: JsValue,
    expected: JsValue,
    operator: JsString,
}

#[wasm_bindgen]
impl AssertionErrorOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        message: Option<JsString>,
        actual: JsValue,
        expected: JsValue,
        operator: JsString,
    ) -> AssertionErrorOptions {
        AssertionErrorOptions {
            message,
            actual,
            expected,
            operator,
        }
    }

    /// If provided, the error message is set to this value.
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<JsString> {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, value: Option<JsString>) {
        self.message = value;
    }

    /// The actual property on the error instance.
    #[wasm_bindgen(getter)]
    pub fn actual(&self) -> JsValue {
        self.actual.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_actual(&mut self, value: JsValue) {
        self.actual = value;
    }

    /// The expected property on the error instance.
    #[wasm_bindgen(getter)]
    pub fn expected(&self) -> JsValue {
        self.expected.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_expected(&mut self, value: JsValue) {
        self.expected = value;
    }

    /// The operator property on the error instance.
    #[wasm_bindgen(getter)]
    pub fn operator(&self) -> JsString {
        self.operator.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_operator(&mut self, value: JsString) {
        self.operator = value;
    }
}
