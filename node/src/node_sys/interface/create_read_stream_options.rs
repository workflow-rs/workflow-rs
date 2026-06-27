use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateReadStreamOptions {
    auto_close: Option<bool>,
    emit_close: Option<bool>,
    encoding: Option<JsString>,
    end: Option<f64>,
    fd: Option<u32>,
    flags: Option<JsString>,
    high_water_mark: Option<f64>,
    mode: Option<u32>,
    start: Option<f64>,
}

#[wasm_bindgen]
impl CreateReadStreamOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        auto_close: Option<bool>,
        emit_close: Option<bool>,
        encoding: Option<JsString>,
        end: Option<f64>,
        fd: Option<u32>,
        flags: Option<JsString>,
        high_water_mark: Option<f64>,
        mode: Option<u32>,
        start: Option<f64>,
    ) -> CreateReadStreamOptions {
        CreateReadStreamOptions {
            auto_close,
            emit_close,
            encoding,
            end,
            fd,
            flags,
            high_water_mark,
            mode,
            start,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn auto_close(&self) -> Option<bool> {
        self.auto_close
    }

    #[wasm_bindgen(setter)]
    pub fn set_auto_close(&mut self, value: Option<bool>) {
        self.auto_close = value;
    }

    #[wasm_bindgen(getter)]
    pub fn emit_close(&self) -> Option<bool> {
        self.emit_close
    }

    #[wasm_bindgen(setter)]
    pub fn set_emit_close(&mut self, value: Option<bool>) {
        self.emit_close = value;
    }

    #[wasm_bindgen(getter)]
    pub fn encoding(&self) -> Option<JsString> {
        self.encoding.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_encoding(&mut self, value: Option<JsString>) {
        self.encoding = value;
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Option<f64> {
        self.end
    }

    #[wasm_bindgen(setter)]
    pub fn set_end(&mut self, value: Option<f64>) {
        self.end = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fd(&self) -> Option<u32> {
        self.fd
    }

    #[wasm_bindgen(setter)]
    pub fn set_fd(&mut self, value: Option<u32>) {
        self.fd = value;
    }

    #[wasm_bindgen(getter)]
    pub fn flags(&self) -> Option<JsString> {
        self.flags.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flags(&mut self, value: Option<JsString>) {
        self.flags = value;
    }

    #[wasm_bindgen(getter)]
    pub fn high_water_mark(&self) -> Option<f64> {
        self.high_water_mark
    }

    #[wasm_bindgen(setter)]
    pub fn set_high_water_mark(&mut self, value: Option<f64>) {
        self.high_water_mark = value;
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Option<u32> {
        self.mode
    }

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, value: Option<u32>) {
        self.mode = value;
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Option<f64> {
        self.start
    }

    #[wasm_bindgen(setter)]
    pub fn set_start(&mut self, value: Option<f64>) {
        self.start = value;
    }
}
