use wasm_bindgen::prelude::*;
use web_sys::{Event,DataTransfer};
use crate::result::Result;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="readText")]
    pub async fn read_text_impl() -> std::result::Result<JsValue,JsValue>;
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="read")]
    pub async fn read_impl() -> std::result::Result<JsValue,JsValue>;
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="writeText")]
    pub async fn write_text(text: &str) -> Result<()>;
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="write")]
    pub async fn write(data: JsValue) -> Result<()>;
}



// static mut READ_TEXT: Option<Function>   = None;
// pub async fn read_text() -> Result<Option<String>> {
//     unsafe {
//         let p = READ_TEXT.get_or_insert_with(|| {
//             js_sys::Function::new_no_args(r###"
//                 async ()=> {
//                     await navigator.clipboard.readText()
//                 }
//             "###
//             )
//         });

//         let result = wasm_bindgen_futures::JsFuture::from(promise).await?;

//         let v = f.call0(&JsValue::UNDEFINED)?;
//         Ok(v.as_string())

//         // HOME_DIR.clone()
//     }
// }


pub async fn read_text() -> Result<Option<String>> {
    workflow_log::log_info!("read_text starting...");
    let value = read_text_impl().await?;
    workflow_log::log_info!("read_text finished...");
    Ok(value.as_string())
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = Event , extends = :: js_sys :: Object , js_name = ClipboardEvent , typescript_type = "ClipboardEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ClipboardEvent;
    #[wasm_bindgen (structural , method , getter , js_class = "ClipboardEvent" , js_name = clipboardData)]
    pub fn clipboard_data(this: &ClipboardEvent) -> Option<DataTransfer>;
}

