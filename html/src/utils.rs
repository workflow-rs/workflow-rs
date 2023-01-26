pub use wasm_bindgen::prelude::JsValue;
pub use web_sys::{Document, Element, Window};

pub type ElementResult<T> = std::result::Result<T, JsValue>;

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("unable to get `document` node")
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn get_element_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}
