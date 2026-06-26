use crate::node_sys::interface::{FormatInputPathObject, ParsedPath};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "path")]
extern "C" {
    pub static delimiter: JsString;

    pub static sep: JsString;

    pub fn basename(path: &JsString, ext: Option<JsString>) -> JsString;

    pub fn dirname(path: &JsString) -> JsString;

    pub fn extname(path: &JsString) -> JsString;

    pub fn format(object: FormatInputPathObject) -> JsString;

    pub fn is_absolute(path: &JsString) -> bool;

    #[wasm_bindgen(variadic)]
    pub fn join(paths: Box<[JsValue]>) -> JsString;

    pub fn normalize(path: &JsString) -> JsString;

    pub fn parse(string: &JsString) -> ParsedPath;

    pub fn relative(from: &JsString, to: &JsString) -> JsString;

    #[wasm_bindgen(variadic)]
    pub fn resolve(path_segments: Box<[JsValue]>) -> JsString;

    // FIXME: path.posix

    // FIXME: path.win32
}
