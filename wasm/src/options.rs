//!
//! Helper trait for managing options struct which extends [Object](js_sys::Object)
//! ```
//! // create MyOptions struct
//!
//! #[wasm_bindgen]
//! extern "C" {
//!     #[wasm_bindgen(extends = js_sys::Object)]
//!     #[derive(Debug, Clone, PartialEq, Eq)]
//!     pub type MyOptions;
//! }
//!
//! impl workflow_wasm::options::OptionsExt for MyOptions{}
//!
//! //impl methods as you need
//! impl MyOptions{
//!     /// Set title
//!     pub fn title(mut self, title:&str)->Self{
//!         self.set("title", JsValue::from(title))
//!     }
//!
//!     /// Set active
//!     pub fn active(mut self, active:bool)->Self{
//!         self.set("active", JsValue::from(active))
//!     }
//! }
//!
//! // use MyOptions
//!
//! let options = MyOptions::new()
//!     .title("title text")
//!     .active(true);
//!
//! ```
//!

use js_sys::Object;
use wasm_bindgen::prelude::*;

pub trait OptionsExt {
    /// "Construct a new `Options`.
    ///
    fn new() -> Self
    where
        Self: wasm_bindgen::JsCast,
    {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(Object::new());
        ret = ret.initialize();
        ret
    }

    fn initialize(self) -> Self
    where
        Self: wasm_bindgen::JsCast,
    {
        self
    }

    fn set(self, mut key: &str, value: JsValue) -> Self
    where
        Self: wasm_bindgen::JsCast,
    {
        let mut target = self.as_ref().clone();

        if key.contains('.') {
            let mut name_parts: Vec<&str> = key.split('.').collect();
            key = name_parts.pop().unwrap();

            for name in name_parts {
                //log_info!("name: {}, target: {:?}", name, target);
                let r = ::js_sys::Reflect::get(&target, &JsValue::from(name));

                match r {
                    Ok(r) => {
                        if !r.is_undefined() {
                            target = r
                        } else {
                            let object = Object::new();
                            let new_target = JsValue::from(object);
                            //log_info!("new_target: {:?}", new_target);
                            let _ =
                                ::js_sys::Reflect::set(&target, &JsValue::from(name), &new_target);

                            target = new_target;
                        }
                    }
                    Err(err) => {
                        panic!(
                            "OptionsExt::set(): unable to find property `{name}`, err: {:?}",
                            err
                        );
                    }
                }
            }

            //log_info!("final: key: {}, target: {:?}", key, target);
        }

        let r = ::js_sys::Reflect::set(&target, &JsValue::from(key), &value);
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;

        self
    }
}
