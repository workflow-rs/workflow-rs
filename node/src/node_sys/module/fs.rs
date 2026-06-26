pub use crate::node_sys::{
    class::{Buffer, fs::*},
    interface::{AppendFileOptions, MkdtempSyncOptions, WriteFileSyncOptions},
};
use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    pub fn access(path: &JsString, mode: Option<u32>, callback: &Function);

    #[wasm_bindgen(js_name = "accessSync")]
    pub fn access_sync(path: &JsString, mode: Option<u32>);

    #[wasm_bindgen(js_name = "appendFile")]
    pub fn append_file(
        path: &JsString,
        data: &Buffer,
        options: Option<AppendFileOptions>,
        callback: &Function,
    );

    #[wasm_bindgen(js_name = "mkdtempSync")]
    pub fn mkdtemp_sync(prefix: &JsString, options: Option<MkdtempSyncOptions>) -> JsString;

    #[wasm_bindgen(js_name = "writeFileSync")]
    pub fn write_file_sync(file: &JsValue, data: &JsValue, options: Option<WriteFileSyncOptions>);
}
