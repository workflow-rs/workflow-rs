use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct Printable(JsValue);

unsafe impl Send for Printable {}
unsafe impl Sync for Printable {}

impl Printable {
    pub fn new(value: JsValue) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Printable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(string) = self.0.as_string() {
            write!(f, "{}", string)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

impl AsRef<JsValue> for Printable {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for Printable {
    fn from(value: JsValue) -> Self {
        Self(value)
    }
}

impl From<JsError> for Printable {
    fn from(value: JsError) -> Self {
        Self(value.into())
    }
}

impl From<&Printable> for JsValue {
    fn from(value: &Printable) -> Self {
        value.0.clone()
    }
}
