//!
//! Access to environment variables when running natively or
//! on top of Node.js (via `process.env`).
//!

use cfg_if::cfg_if;
use std::env::VarError;

pub fn var(_key: &str) -> Result<String, VarError> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            if crate::runtime::is_node() {
                match get_nodejs_env_var(_key)? {
                    Some(v) => Ok(v),
                    None => {
                        Err(VarError::NotPresent)
                    }
                }
            } else {
                panic!("workflow_core::env::var() is not supported on this platform (must be native of nodejs)");
            }
        } else {
            std::env::var(_key)
        }
    }
}

#[allow(dead_code)]
fn get_nodejs_env_var(key: &str) -> Result<Option<String>, VarError> {
    use js_sys::{Object, Reflect};
    use wasm_bindgen::prelude::*;

    let process = Reflect::get(&js_sys::global(), &"process".into())
        .expect("Unable to get nodejs process global");
    let env = Reflect::get(&process, &"env".into()).expect("Unable to get nodejs process.env");
    let object = Object::from(env);
    let value =
        Reflect::get(&object, &JsValue::from_str(key)).map_err(|_err| VarError::NotPresent)?;
    Ok(value.as_string())
}
