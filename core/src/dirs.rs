//!
//! Access to home and data folder (windows) when running natively or
//! within Node.js
//!

use cfg_if::cfg_if;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            if crate::runtime::is_node() {
                nodejs::home_dir()
            } else {
                panic!("workflow_core::dirs::home_dir() is not supported on this platform (must be native of nodejs)");
            }
        } else {
            dirs::home_dir()
        }
    }
}

pub fn data_dir() -> Option<PathBuf> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            if crate::runtime::is_node() {
                nodejs::data_dir()
            } else {
                panic!("workflow_core::dirs::home_dir() is not supported on this platform (must be native of nodejs)");
            }
        } else {
            dirs::data_dir()
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod nodejs {
            use std::path::{Path,PathBuf};
            use wasm_bindgen::prelude::*;
            use js_sys::Reflect;

            #[wasm_bindgen]
            extern "C" {
                pub fn require(s: &str) -> JsValue;
            }

            static mut HOME_DIR: Option<PathBuf> = None;
            pub fn home_dir() -> Option<PathBuf> {
                unsafe {
                    HOME_DIR.get_or_insert_with(|| {
                        Reflect::get(&require("os"), &JsValue::from_str("homedir"))
                            .expect("Unable to get homedir")
                            .dyn_into::<js_sys::Function>()
                            .expect("os.homedir is not a function")
                            .call0(&JsValue::UNDEFINED)
                            .expect("Unable to get homedir")
                            .as_string()
                            .as_ref()
                            .map(Path::new)
                            .map(PathBuf::from)
                            .expect("Unable to get nodejs homedir")
                    });
                    HOME_DIR.clone()
                }
            }

            static mut DATA_DIR: Option<PathBuf> = None;
            pub fn data_dir() -> Option<PathBuf> {
                unsafe {
                    DATA_DIR.get_or_insert_with(|| {
                        if crate::runtime::is_windows() {
                            crate::env::var("LOCALAPPDATA")
                                .ok()
                                .map(PathBuf::from)
                                .expect("Unable to get LOCALAPPDATA")

                        } else {
                            home_dir()
                                .expect("Unable to get nodejs data_dir (unable to get home_dir)")
                        }
                    });
                    DATA_DIR.clone()
                }
            }
        }
    }
}
