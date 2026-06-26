use js_sys::Function;
use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub struct CreateHookCallbacks {
    init: Function,
    before: Function,
    after: Function,
    destroy: Function,
    promise_resolve: Function,
}

#[wasm_bindgen]
impl CreateHookCallbacks {
    #[wasm_bindgen(constructor)]
    pub fn new(
        init: &Function,
        before: &Function,
        after: &Function,
        destroy: &Function,
        promise_resolve: &Function,
    ) -> CreateHookCallbacks {
        CreateHookCallbacks {
            init: init.clone(),
            before: before.clone(),
            after: after.clone(),
            destroy: destroy.clone(),
            promise_resolve: promise_resolve.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn init(&self) -> Function {
        self.init.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_init(&mut self, value: Function) {
        self.init = value;
    }

    #[wasm_bindgen(getter)]
    pub fn before(&self) -> Function {
        self.before.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_before(&mut self, value: Function) {
        self.before = value;
    }

    #[wasm_bindgen(getter)]
    pub fn after(&self) -> Function {
        self.after.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_after(&mut self, value: Function) {
        self.after = value;
    }

    #[wasm_bindgen(getter)]
    pub fn destroy(&self) -> Function {
        self.destroy.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_destroy(&mut self, value: Function) {
        self.destroy = value;
    }

    #[wasm_bindgen(getter)]
    pub fn promise_resolve(&self) -> Function {
        self.promise_resolve.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_promise_resolve(&mut self, value: Function) {
        self.promise_resolve = value;
    }
}
