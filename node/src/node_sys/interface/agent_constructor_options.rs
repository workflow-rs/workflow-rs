use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AgentConstructorOptions {
    keep_alive_msecs: f64,
    keep_alive: bool,
    max_free_sockets: f64,
    max_sockets: f64,
    timeout: f64,
}

#[wasm_bindgen]
impl AgentConstructorOptions {
    #[wasm_bindgen(getter)]
    pub fn keep_alive_msecs(&self) -> f64 {
        self.keep_alive_msecs
    }

    #[wasm_bindgen(setter)]
    pub fn set_keep_alive_msecs(&mut self, value: f64) {
        self.keep_alive_msecs = value;
    }

    #[wasm_bindgen(getter)]
    pub fn keep_alive(&self) -> bool {
        self.keep_alive
    }

    #[wasm_bindgen(setter)]
    pub fn set_keep_alive(&mut self, value: bool) {
        self.keep_alive = value;
    }

    #[wasm_bindgen(getter)]
    pub fn max_free_sockets(&self) -> f64 {
        self.max_free_sockets
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_free_sockets(&mut self, value: f64) {
        self.max_free_sockets = value;
    }

    #[wasm_bindgen(getter)]
    pub fn max_sockets(&self) -> f64 {
        self.max_sockets
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_sockets(&mut self, value: f64) {
        self.max_sockets = value;
    }

    #[wasm_bindgen(getter)]
    pub fn timeout(&self) -> f64 {
        self.timeout
    }

    #[wasm_bindgen(setter)]
    pub fn set_timeout(&mut self, value: f64) {
        self.timeout = value;
    }
}
