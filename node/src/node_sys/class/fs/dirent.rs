use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Dirent;

    // FIXME: event overloads

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "isFile")]
    pub fn is_file(this: &Dirent) -> bool;

    #[wasm_bindgen(method, js_name = "isDirectory")]
    pub fn is_directory(this: &Dirent) -> bool;

    #[wasm_bindgen(method, js_name = "isBlockDevice")]
    pub fn is_block_device(this: &Dirent) -> bool;

    #[wasm_bindgen(method, js_name = "isCharacterDevice")]
    pub fn is_character_device(this: &Dirent) -> bool;

    #[wasm_bindgen(method, js_name = "isSymbolic")]
    pub fn is_symbolic_link(this: &Dirent) -> bool;

    #[wasm_bindgen(method, js_name = "isFIFO")]
    pub fn is_fifo(this: &Dirent) -> bool;

    #[wasm_bindgen(method, js_name = "isSocket")]
    pub fn is_socket(this: &Dirent) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Dirent) -> JsString;
}
