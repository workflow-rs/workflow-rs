use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Read the textual contents of the system clipboard via
    /// `navigator.clipboard.readText()`.
    #[wasm_bindgen (js_namespace=["navigator", "clipboard"], js_name="readText")]
    pub async fn read_text() -> JsValue;
    /// Read the contents of the system clipboard as `ClipboardItem`s via
    /// `navigator.clipboard.read()`.
    #[wasm_bindgen (js_namespace=["navigator", "clipboard"], js_name="read")]
    pub async fn read() -> JsValue;
    /// Write the given text to the system clipboard via
    /// `navigator.clipboard.writeText()`.
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="writeText")]
    pub async fn write_text(text: &str) -> Result<(), JsValue>;
    /// Write the given `ClipboardItem` data to the system clipboard via
    /// `navigator.clipboard.write()`.
    #[wasm_bindgen (catch, js_namespace=["navigator", "clipboard"], js_name="write")]
    pub async fn write(data: JsValue) -> Result<(), JsValue>;
}
