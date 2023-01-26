//! Helper functions for accessing DOM environment
use web_sys::{Document, Element, Window};

/// Return the current browser [`web_sys::Window`] element
pub fn window() -> Window {
    web_sys::window().unwrap()
}

/// Return the current browser [`web_sys::Document`] element
pub fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Return the `body` element of the current document
pub fn body() -> std::result::Result<Element, String> {
    let b = document()
        .query_selector("body")
        .unwrap()
        .ok_or_else(|| "Unable to get body element".to_string())?;
    Ok(b)
}
