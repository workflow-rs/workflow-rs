//!
//! Access to environment variables when running natively or
//! on top of Node.js (via `process.env`).
//!

use cfg_if::cfg_if;
use js_sys::{Function, Object, Reflect};
use std::env::VarError;
use wasm_bindgen::prelude::*;

pub fn var(key: &str) -> Result<String, VarError> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            if crate::runtime::is_node() {
                match get_nodejs_env_var(key)? {
                    Some(v) => return Ok(v),
                    None => {
                        return Err(VarError::NotPresent)
                    }
                }
            } else {
                panic!("workflow_core::env::var() is not supported on this platform (must be native of nodejs)");
            }
        } else {
            std::env::var(key)
        }
    }
}

#[allow(dead_code)]
fn get_nodejs_env_var(key: &str) -> Result<Option<String>, VarError> {
    let result = Function::new_no_args(
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
