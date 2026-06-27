use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetNameOptions {
    family: Option<u8>,
    host: JsString,
    local_address: JsString,
    port: u32,
}

#[wasm_bindgen]
impl GetNameOptions {
    pub fn new(
        family: Option<u8>,
        host: JsString,
        local_address: JsString,
        port: u32,
    ) -> GetNameOptions {
        GetNameOptions {
            family,
            host,
            local_address,
            port,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn family(&self) -> Option<u8> {
        self.family
    }

    #[wasm_bindgen(setter)]
    pub fn set_family(&mut self, value: Option<u8>) {
        self.family = value;
    }

    #[wasm_bindgen(getter)]
    pub fn host(&self) -> JsString {
        self.host.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_host(&mut self, value: JsString) {
        self.host = value;
    }

    #[wasm_bindgen(getter)]
    pub fn local_address(&self) -> JsString {
        self.local_address.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_local_address(&mut self, value: JsString) {
        self.local_address = value;
    }

    #[wasm_bindgen(getter)]
    pub fn port(&self) -> u32 {
        self.port
    }

    #[wasm_bindgen(setter)]
    pub fn set_port(&mut self, value: u32) {
        self.port = value;
    }
}
