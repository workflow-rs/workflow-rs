//! Provides functions that allow to detect the runtime environment. These functions can be used to
//! detect whether the code is running in a browser, node.js or native OS, the type of the underlying OS
//! (Windows,Linux,MacOS,*BSD) as well as the type of a web environment (Browser or NWJS).
//! This is useful for an application of an API to detect which environment it is operating under
//! and subsequently restrict the functionality to the capabilities to this environment.

use cfg_if::cfg_if;
use std::sync::OnceLock;

cfg_if! {
    if #[cfg(target_arch = "wasm32")]{
        use js_sys::Reflect;
        use wasm_bindgen::prelude::JsValue;

        #[derive(Clone, Copy)]
        struct JavaScriptRuntime {
            // NodeJs environment
            nodejs : bool,
            // NWJS environment
            nwjs : bool,
            // Electron environment (browser or renderer)
            electron : bool,
            // Browser-capable environment
            browser : bool,
            // Pure web environment (no NodeJs, no NWJS, no Electron)
            web : bool,
        }

        #[inline(always)]
        fn exists(property: &str) -> bool {
            js_sys::Reflect::get(&js_sys::global(), &property.into()).map(|v|!v.is_falsy()).unwrap_or(false)
        }

        fn exists_prop(object : &JsValue, property: &str) -> bool {
            js_sys::Reflect::get(object, &property.into()).map(|v|!v.is_falsy()).unwrap_or(false)
        }

        #[inline]
        fn detect() -> &'static JavaScriptRuntime {
            static JAVASCRIPT_RUNTIME: OnceLock<JavaScriptRuntime> = OnceLock::new();
            JAVASCRIPT_RUNTIME.get_or_init(|| {
                let global = js_sys::global();

                let mut browser = exists("window") && exists("document") && exists("location") && exists("navigator");

                let process = Reflect::get(&global, &"process".into());
                let versions = process
                    .clone()
                    .and_then(|process|Reflect::get(&process, &"versions".into()));

                let nodejs = versions
                    .clone()
                    .map(|versions|exists_prop(&versions, "node")).unwrap_or(false);

                let electron = versions
                    .clone()
                    .map(|versions|exists_prop(&versions, "electron")).unwrap_or(false);


                if electron {
                    if let Ok(process_type) = process.and_then(|process|Reflect::get(&process, &"type".into())) {
                        browser = process_type.as_string().map(|v|v.as_str() == "renderer").unwrap_or(false);
                    }
                }

                let nwjs = Reflect::get(&global, &"nw".into())
                    .map(|nw|exists_prop(&nw, "Window")).unwrap_or(false);

                let web = !nodejs && !nwjs && !electron;

                JavaScriptRuntime {
                    nodejs,
                    nwjs,
                    electron,
                    browser,
                    web,
                }
            })
        }

        /// Helper to test whether the application is running under
        /// NodeJs-compatible environment.
        pub fn is_node() -> bool {
            detect().nodejs
        }

        /// Helper to test whether the application is running under
        /// NW environment.
        pub fn is_nw() -> bool {
            detect().nwjs
        }

        /// Helper to test whether the application is running under
        /// Electron.
        pub fn is_electron() -> bool {
            detect().electron
        }

        /// Helper to test whether the application is running under
        /// Electron backend.
        pub fn is_electron_server() -> bool {
            detect().electron && !detect().browser
        }

        /// Helper to test whether the application is running under
        /// Electron backend.
        pub fn is_electron_client() -> bool {
            detect().electron && detect().browser
        }

        /// Identifies web-capable environment (browser, NWJS window, Electron client)
        pub fn is_web_capable() -> bool {
            detect().browser
        }

        /// Helper to test whether the application is running under
        /// in a regular browser environment (not NodeJs and not NW).
        pub fn is_web()->bool{
            detect().web
        }

        /// Helper to test whether the application is running
        /// in a cross-origin isolated browser environment (Flutter).
        #[inline(always)]
        pub fn is_cross_origin_isolated()->bool{
            static CROSS_ORIGIN: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            *CROSS_ORIGIN.get_or_init(|| {
                js_sys::Reflect::get(&js_sys::global(), &"crossOriginIsolated".into())
                .map(|v| !v.is_falsy())
                .unwrap_or(false)
            })
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
        /// Electron.
        pub fn is_electron() -> bool {
            false
        }

        /// Helper to test whether the application is running under
        /// Electron backend.
        pub fn is_electron_server() -> bool {
            false
        }

        /// Helper to test whether the application is running under
        /// Electron backend.
        pub fn is_electron_client() -> bool {
            false
        }

        /// Identifies web-capable environment (browser, NWJS window, Electron client)
        pub fn is_web_capable() -> bool {
            false
        }

        /// Helper to test whether the application is running under
        /// in a regular browser environment.
        pub fn is_web()->bool {
            false
        }

        /// Helper to test whether the application is running
        /// in a cross-origin isolated browser environment (Flutter).
        #[inline(always)]
        pub fn is_cross_origin_isolated()->bool{
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

/// Helper to test (at runtime) whether the
/// application is running under WASM32 architecture.
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
    pub fn from_node() -> Self {
        let process = js_sys::Reflect::get(&js_sys::global(), &"process".into())
            .expect("Unable to get nodejs process global");
        let platform = js_sys::Reflect::get(&process, &"platform".into())
            .expect("Unable to get nodejs process.platform");

        let platform = match platform
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

static PLATFORM: OnceLock<Platform> = OnceLock::new();

pub fn platform() -> Platform {
    PLATFORM
        .get_or_init(|| {
            cfg_if! {
                if #[cfg(target_os = "windows")] {
                    Platform::Windows
                } else if #[cfg(target_os = "macos")] {
                    Platform::MacOS
                } else if #[cfg(target_os = "linux")] {
                    Platform::Linux
                } else if #[cfg(target_os = "android")] {
                    Platform::Android
                } else if #[cfg(target_os = "ios")] {
                    Platform::IOS
                } else if #[cfg(target_arch = "wasm32")] {
                    if is_node() {
                        Platform::from_node()
                    } else {
                        Platform::from_web()
                    }
                }
            }
        })
        .clone()
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

pub fn is_mobile() -> bool {
    is_ios() || is_android()
}

pub fn is_chrome_extension() -> bool {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {

            static IS_CHROME_EXTENSION : OnceLock<bool> = OnceLock::new();

            *IS_CHROME_EXTENSION.get_or_init(||{
                if is_web() {
                    js_sys::Reflect::get(&js_sys::global(), &"location".into())
                    .and_then(|location| { js_sys::Reflect::get(&location, &"protocol".into()) })
                    .map(|protocol|protocol == "chrome-extension:")
                    .unwrap_or(false)
                } else {
                    false
                }
            })

        } else {
            false
        }
    }
}
