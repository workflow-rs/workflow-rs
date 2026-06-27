use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormatInputPathObject {
    base: Option<JsString>,
    dir: Option<JsString>,
    ext: Option<JsString>,
    name: Option<JsString>,
    root: Option<JsString>,
}

#[wasm_bindgen]
impl FormatInputPathObject {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        base: Option<JsString>,
        dir: Option<JsString>,
        ext: Option<JsString>,
        name: Option<JsString>,
        root: Option<JsString>,
    ) -> FormatInputPathObject {
        FormatInputPathObject {
            base,
            dir,
            ext,
            name,
            root,
        }
    }

    pub fn new() -> FormatInputPathObject {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Option<JsString> {
        self.base.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_base(&mut self, value: Option<JsString>) {
        self.base = value;
    }

    #[wasm_bindgen(getter)]
    pub fn dir(&self) -> Option<JsString> {
        self.dir.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_dir(&mut self, value: Option<JsString>) {
        self.dir = value;
    }

    #[wasm_bindgen(getter)]
    pub fn ext(&self) -> Option<JsString> {
        self.ext.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_ext(&mut self, value: Option<JsString>) {
        self.ext = value;
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<JsString> {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: Option<JsString>) {
        self.name = value;
    }

    #[wasm_bindgen(getter)]
    pub fn root(&self) -> Option<JsString> {
        self.root.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_root(&mut self, value: Option<JsString>) {
        self.root = value;
    }
}
