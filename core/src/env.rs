//!
//! Access to environment variables when running natively or
//! on top of Node.js (via `process.env`).
//!

use cfg_if::cfg_if;
use std::env::VarError;

pub fn var(_key: &str) -> Result<String, VarError> {
    cfg_if! {
        if #[cfg(all(feature = "no-unsafe-eval", target_arch = "wasm32"))] {
            Err(VarError::NotPresent)
        } else if #[cfg(target_arch = "wasm32")] {
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
#[cfg(not(feature = "no-unsafe-eval"))]
fn get_nodejs_env_var(key: &str) -> Result<Option<String>, VarError> {
    use js_sys::{Object, Reflect};
    use wasm_bindgen::prelude::*;

    let result = js_sys::Function::new_no_args(
        // no-unsafe-eval
        "
        return process.env;
    ",
    )
    .call0(&JsValue::undefined())
    .expect("Unable to get nodejs process.env");

    let object = Object::from(result);
    let value =
        Reflect::get(&object, &JsValue::from_str(key)).map_err(|_err| VarError::NotPresent)?;
    Ok(value.as_string())
}
