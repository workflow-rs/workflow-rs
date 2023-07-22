use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (js_namespace=["navigator", "clipboard"], js_name="readText")]
    pub async fn read_text() -> JsValue;
    #[wasm_bindgen (js_namespace=["navigator", "clipboard"], js_name="read")]
    pub async fn read() -> JsValue;
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="writeText")]
    pub async fn write_text(text: &str) -> Result<(), JsValue>;
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="write")]
    pub async fn write(data: JsValue) -> Result<(), JsValue>;
}
