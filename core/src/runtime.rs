//! Provides functions that allow to detect the runtime environment. These functions can be used to
//! detect whether the code is running in a browser, node.js or native OS, the type of the underlying OS
//! (Windows,Linux,MacOS,*BSD) as well as the type of a web environment (Browser or NWJS).
//! This is useful for an application of an API to detect which environment it is operating under
//! and subsequently restrict the functionality to the capabilities to this environment.

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")]{
        use js_sys::Object;
        use wasm_bindgen::prelude::*;

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

                cfg_if! {
                    if #[cfg(feature = "no-unsafe-eval")] {
                        let global = js_sys::global();
                        let mut flags = (false, false);
                        let _ = js_sys::Reflect::get(&global, &"process".into())
                            .and_then(|process|js_sys::Reflect::get(&process, &"versions".into()))
                            .and_then(|versions|js_sys::Reflect::get(&versions, &"node".into()))
                            .and_then(|_node|{
                                flags.0 = true;
                                js_sys::Reflect::get(&global, &"nw".into())
                            })
                            .and_then(|nw|js_sys::Reflect::get(&nw, &"Window".into()))
                            .map(|_window|{
                                flags.1 = true;
                            });

                        unsafe { DETECT = Some(flags) };
                        flags
                    } else {

                        let result = js_sys::Function::new_no_args( // no-unsafe-eval
                            "
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
                        ").call0(&wasm_bindgen::JsValue::undefined());

                        let flags = match result {
                            Ok(value) => {
                                if value.is_undefined() {
                                    (false, false)
                                } else {
                                    let info: __NodeJsNodeWebkitInfo__ = value.into();
                                    (info.is_node_js(), info.is_node_webkit())
                                }
                            }
                            Err(_) => {
                                (false, false)
                            }
                        };

                        unsafe { DETECT = Some(flags) };
                        flags
                    }
                }


            })

        }

        /// Helper to test whether the application is running under
        /// NodeJs-compatible environment.
        pub fn is_node() -> bool {
            detect().0
        }

        /// Helper to test whether the application is running under
        /// NW environment.
        pub fn is_nw() -> bool {
            detect().1
        }

        /// Helper to test whether the application is running under
        /// in a regular browser environment (not NodeJs and not NW).
        pub fn is_web()->bool{
            !is_node() && !is_nw()
        }

    }else{

        /// Helper to test whether the application is running under
        /// NodeJs-compatible environment.
        pub fn is_node() -> bool {
            false
        }

        /// Helper to test whether the application is running under
        /// NW environment.
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Android,
    IOS,
    Unknown,
    Other(String),
}

impl Platform {
    #[cfg(not(feature = "no-unsafe-eval"))]
    pub fn from_node() -> Self {
        let result = js_sys::Function::new_no_args(
            // no-unsafe-eval
            "
            return process.platform;
        ",
        )
        .call0(&wasm_bindgen::JsValue::undefined())
        .expect("Unable to get nodejs process.platform");

        let platform = match result
            .as_string()
            .expect("nodejs process.platform is not a string")
            .as_str()
        {
            "win32" => Platform::Windows,
            "darwin" => Platform::MacOS,
            "linux" => Platform::Linux,
            "openbsd" => Platform::OpenBSD,
            "freebsd" => Platform::FreeBSD,
            v => Platform::Other(v.to_string()),
        };

        platform
    }

    pub fn from_web() -> Self {
        let window = if let Some(window) = web_sys::window() {
            window
        } else {
            return Platform::Unknown;
        };

        let user_agent = if let Ok(user_agent) = window.navigator().user_agent() {
            user_agent.to_lowercase()
        } else {
            return Platform::Unknown;
        };

        if user_agent.contains("win") {
            Platform::Windows
        } else if user_agent.contains("mac") {
            Platform::MacOS
        } else if user_agent.contains("linux") {
            Platform::Linux
        } else if user_agent.contains("android") {
            Platform::Android
        } else if user_agent.contains("ios")
            || user_agent.contains("iphone")
            || user_agent.contains("ipad")
        {
            Platform::IOS
        } else if user_agent.contains("freebsd") {
            Platform::FreeBSD
        } else if user_agent.contains("openbsd") {
            Platform::OpenBSD
        } else if user_agent.contains("netbsd") {
            Platform::NetBSD
        } else {
            Platform::Unknown
        }
    }
}

static mut PLATFORM: Option<Platform> = None;

pub fn platform() -> Platform {
    if let Some(platform) = unsafe { PLATFORM.as_ref() } {
        platform.clone()
    } else {
        cfg_if! {
            if #[cfg(target_os = "windows")] {
                let platform = Platform::Windows;
            } else if #[cfg(target_os = "macos")] {
                let platform = Platform::MacOS;
            } else if #[cfg(target_os = "linux")] {
                let platform = Platform::Linux;
            } else if #[cfg(all(target_arch = "wasm32", feature="no-unsafe-eval"))] {
                let platform = Platform::from_web();
            } else if #[cfg(target_arch = "wasm32")] {
                let platform = if is_node() {
                    Platform::from_node()
                } else {
                    Platform::from_web()
                };
            }
        }

        unsafe { PLATFORM.replace(platform.clone()) };
        platform
    }
}

pub fn is_windows() -> bool {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            true
        } else {
            platform() == Platform::Windows
        }
    }
}

pub fn is_macos() -> bool {
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            true
        } else {
            platform() == Platform::MacOS
        }
    }
}

pub fn is_linux() -> bool {
    cfg_if! {
        if #[cfg(target_os = "linux")] {
            true
        } else {
            platform() == Platform::Linux
        }
    }
}

pub fn is_freebsd() -> bool {
    cfg_if! {
        if #[cfg(target_os = "freebsd")] {
            true
        } else {
            platform() == Platform::FreeBSD
        }
    }
}

pub fn is_openbsd() -> bool {
    cfg_if! {
        if #[cfg(target_os = "openbsd")] {
            true
        } else {
            platform() == Platform::OpenBSD
        }
    }
}

pub fn is_netbsd() -> bool {
    cfg_if! {
        if #[cfg(target_os = "netbsd")] {
            true
        } else {
            platform() == Platform::NetBSD
        }
    }
}

pub fn is_ios() -> bool {
    platform() == Platform::IOS
}

pub fn is_android() -> bool {
    platform() == Platform::Android
}

pub fn is_unix() -> bool {
    is_macos() || is_linux() || is_freebsd() || is_openbsd() || is_netbsd()
}

pub fn is_chrome_extension() -> bool {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {

            static mut IS_CHROME_EXTENSION : Option<bool> = None;

            unsafe {
                *IS_CHROME_EXTENSION.get_or_insert_with(||{
                    let global = js_sys::global();
                    let location = js_sys::Reflect::get(&global, &"location".into()).unwrap();
                    let protocol = js_sys::Reflect::get(&location, &"protocol".into()).unwrap();

                    protocol == "chrome-extension:"
                })
            }

        } else {
            false
        }
    }
}
