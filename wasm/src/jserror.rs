use wasm_bindgen::prelude::*;

pub trait JsErrorTrait {
    fn message(&self) -> String;
}

impl JsErrorTrait for JsError {
    fn message(&self) -> String {
        let inner = JsValue::from(self.clone());
        let msg = js_sys::Reflect::get(&inner, &JsValue::from_str("message"))
            .expect("unable to get error message");
        msg.as_string()
            .expect("unable to convert error message to string")
    }
}

pub trait JsValueErrorTrait {
    fn error_message(&self) -> String;
}

impl JsValueErrorTrait for JsValue {
    fn error_message(&self) -> String {
        let msg = js_sys::Reflect::get(self, &JsValue::from_str("message"))
            .expect("unable to get error message");
        msg.as_string()
            .expect("unable to convert error message to string")
    }
}
