use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct WasiOptions {
    args: Option<Box<[JsValue]>>,
    env: Option<Object>,
    preopens: Object,
}

#[wasm_bindgen]
impl WasiOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        args: Option<Box<[JsValue]>>,
        env: Option<Object>,
        preopens: Object,
    ) -> WasiOptions {
        WasiOptions {
            args,
            env,
            preopens,
        }
    }

    pub fn new(preopens: Object) -> WasiOptions {
        let args = Default::default();
        let env = Default::default();
        WasiOptions::new_with_values(args, env, preopens)
    }

    #[wasm_bindgen(getter)]
    pub fn args(&self) -> Option<Box<[JsValue]>> {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_args(&mut self, value: Option<Box<[JsValue]>>) {
        self.args = value;
    }

    #[wasm_bindgen(getter)]
    pub fn env(&self) -> Option<Object> {
        self.env.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_env(&mut self, value: Option<Object>) {
        self.env = value;
    }

    #[wasm_bindgen(getter)]
    pub fn preopens(&self) -> Object {
        self.preopens.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_preopens(&mut self, value: Object) {
        self.preopens = value;
    }
}
