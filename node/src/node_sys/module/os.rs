use crate::node_sys::interface::{OsConstants, UserInfoOptions};
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "os")]
extern "C" {
    //******************//
    // Instance Methods //
    //******************//

    pub fn arch() -> JsString;

    pub fn cpus() -> Box<[JsValue]>;

    pub fn endianness() -> JsString;

    pub fn freemem() -> f64;

    #[wasm_bindgen(js_name = "getPriority")]
    pub fn get_priority(pid: Option<u32>) -> u32;

    pub fn homedir() -> JsString;

    pub fn hostname() -> JsString;

    pub fn loadavg() -> Box<[JsValue]>;

    #[wasm_bindgen(js_name = "networkInterfaces")]
    pub fn network_interfaces() -> Object;

    pub fn platform() -> JsString;

    pub fn release() -> JsString;

    #[wasm_bindgen(js_name = "setPriority")]
    pub fn set_priority(priority: i32);

    #[wasm_bindgen(js_name = "setPriority")]
    pub fn set_priority_for_pid(pid: u32, priority: i32);

    pub fn tmpdir() -> JsString;

    pub fn totalmem() -> f64;

    #[wasm_bindgen(js_name = "type")]
    pub fn type_() -> JsString;

    pub fn uptime() -> f64;

    #[wasm_bindgen(js_name = "userInfo")]
    pub fn user_info(options: Option<UserInfoOptions>) -> Object;

    //************//
    // Properties //
    //************//

    pub static constants: OsConstants;

    pub static EOL: JsString;
}
