use js_sys::{Date, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Stats;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "isBlockDevice")]
    pub fn is_block_device(this: &Stats) -> bool;

    #[wasm_bindgen(method, js_name = "isCharacterDevice")]
    pub fn is_character_device(this: &Stats) -> bool;

    #[wasm_bindgen(method, js_name = "isDirectory")]
    pub fn is_directory(this: &Stats) -> bool;

    #[wasm_bindgen(method, js_name = "isFIFO")]
    pub fn is_fifo(this: &Stats) -> bool;

    #[wasm_bindgen(method, js_name = "isFile")]
    pub fn is_file(this: &Stats) -> bool;

    #[wasm_bindgen(method, js_name = "isSocket")]
    pub fn is_socket(this: &Stats) -> bool;

    #[wasm_bindgen(method, js_name = "isSymbolicLink")]
    pub fn is_symbolic_link(this: &Stats) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn dev(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn ino(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn mode(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn nlink(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn uid(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn gid(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rdev(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn blksize(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn blocks(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter, js_name = "atimeMs")]
    pub fn atime_ms(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter, js_name = "mtimeMs")]
    pub fn mtime_ms(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter, js_name = "ctimeMs")]
    pub fn ctime_ms(this: &Stats) -> f64;

    #[wasm_bindgen(method, getter, js_name = "birthtimeMs")]
    pub fn birthtime_ms(this: &Stats) -> f64;

    // NOTE: requires bigint
    #[wasm_bindgen(method, getter, js_name = "atimeNs")]
    pub fn atime_ns(this: &Stats) -> u64;

    // NOTE: requires bigint
    #[wasm_bindgen(method, getter, js_name = "mtimeNs")]
    pub fn mtime_ns(this: &Stats) -> u64;

    // NOTE: requires bigint
    #[wasm_bindgen(method, getter, js_name = "ctimeNs")]
    pub fn ctime_ns(this: &Stats) -> u64;

    // NOTE: requires bigint
    #[wasm_bindgen(method, getter, js_name = "birthtimeNs")]
    pub fn birthtime_ns(this: &Stats) -> u64;

    #[wasm_bindgen(method, getter)]
    pub fn atime(this: &Stats) -> Date;

    #[wasm_bindgen(method, getter)]
    pub fn mtime(this: &Stats) -> Date;

    #[wasm_bindgen(method, getter)]
    pub fn ctime(this: &Stats) -> Date;

    #[wasm_bindgen(method, getter)]
    pub fn birthtime(this: &Stats) -> Date;
}
