use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")]{
        use js_sys::{Function, Object};
        use wasm_bindgen::prelude::*;
        //use workflow_log::log_trace;
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(extends = Object)]
            #[derive(Debug)]
            type __NodeJsNodeWebkitInfo__;

            #[wasm_bindgen(method, getter)]
            fn is_node_js(this: &__NodeJsNodeWebkitInfo__) -> bool;

            #[wasm_bindgen(method, getter)]
            fn is_node_webkit(this: &__NodeJsNodeWebkitInfo__) -> bool;
        }

        static mut DETECT: Option<(bool,bool)> = None;
        fn detect() -> (bool, bool) {
            unsafe { DETECT }.unwrap_or_else(||{

                let result = Function::new_no_args("
                    let is_node_js = (
                        typeof process === 'object' && 
                        typeof process.versions === 'object' && 
                        typeof process.versions.node !== 'undefined'
                    );

                    let is_node_webkit = false;
                    if(is_node_js) {
                        is_node_webkit = (typeof nw !== 'undefined' && typeof nw.Window !== 'undefined');
                    }

                    return {
                        is_node_js,
                        is_node_webkit
                    }
                ").call0(&JsValue::undefined());

                let flags = match result {
                    Ok(value) => {
                        if value.is_undefined() {
                            (false, false)
                        } else {
                            //log_trace!("value: {:?}", value);
                            let info: __NodeJsNodeWebkitInfo__ = value.into();
                            //log_trace!("info: {:?}", info);
                            //log_trace!("is_node_js: {:?}", info.is_node_js());
                            //log_trace!("is_node_webkit: {:?}", info.is_node_webkit());
                            (info.is_node_js(), info.is_node_webkit())
                        }
                    }
                    Err(_) => {
                        (false, false)
                    }
                };

                unsafe { DETECT = Some(flags) };
                flags
            })

        }

        /// Helper to test whether the application is running under
        /// NodeJs-compatible environment.
        pub fn is_node() -> bool {
            detect().0
        }

        /// Helper to test whether the application is running under
        /// Node Webkit environment.
        pub fn is_nw() -> bool {
            detect().1
        }

        /// Helper to test whether the application is running under
        /// in a regular browser environment.
        pub fn is_web()->bool{
            !is_node()
        }

    }else{

        /// Helper to test whether the application is running under
        /// NodeJs-compatible environment.
        pub fn is_node() -> bool {
            false
        }

        /// Helper to test whether the application is running under
        /// Node Webkit environment.
        pub fn is_nw() -> bool {
            false
        }

        /// Helper to test whether the application is running under
        /// in a regular browser environment.
        pub fn is_web()->bool{
            false
        }
    }
}

/// Helper to test whether the application is running under
/// Solana OS.
pub fn is_solana() -> bool {
    cfg_if! {
        if #[cfg(target_os = "solana")]{
            true
        }else{
            false
        }
    }
}

/// Helper to test whether the application is running under
/// WASM32 architecture.
pub fn is_wasm() -> bool {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")]{
            true
        }else{
            false
        }
    }
}

/// Helper to test whether the application is running under
/// native runtime which is not a Solana OS and architecture is not WASM32
pub fn is_native() -> bool {
    cfg_if! {
        if #[cfg(any(target_os = "solana", target_arch = "wasm32"))] {
            false
        }else{
            true
        }
    }
}

/// application runtime info
#[derive(Debug)]
pub enum Runtime {
    Native,
    Solana,
    NW,
    Node,
    Web,
}

impl From<&Runtime> for String {
    fn from(value: &Runtime) -> Self {
        match value {
            Runtime::Native => "Native",
            Runtime::Solana => "Solana",
            Runtime::NW => "NW",
            Runtime::Node => "Node",
            Runtime::Web => "Web",
        }
        .to_string()
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = self.into();
        f.write_str(&str)
    }
}

impl Runtime {
    /// get Runtime object
    pub fn get() -> Self {
        if is_solana() {
            Runtime::Solana
        } else if is_wasm() {
            if is_nw() {
                Runtime::NW
            } else if is_node() {
                Runtime::Node
            } else {
                Runtime::Web
            }
        } else {
            Runtime::Native
        }
    }
}
