pub use wasm_bindgen::prelude::JsValue;
pub use web_sys::{Document, Element, Window};

/// Result type for DOM operations, carrying a [`JsValue`] error on failure.
pub type ElementResult<T> = std::result::Result<T, JsValue>;

/// Returns the global `document` node, panicking if no browser context exists.
pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("unable to get `document` node")
}

/// Returns the global `window` object, panicking if no browser context exists.
pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

/// Returns the DOM element with the given `id`, or `None` if not found.
pub fn get_element_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}
