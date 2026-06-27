use crate::imports::*;

/// Configuration for launching an application, covering the window caption,
/// web canvas target, available modules and platform-specific eframe options.
pub struct Options<T> {
    /// Window title (native) caption for the application.
    pub caption: String,
    /// DOM id of the HTML canvas to mount the application into (web).
    pub canvas_id: String,
    /// Optional set of application modules to register.
    pub modules: Option<Vec<Module<T>>>,
    /// Identifier of the module to activate on startup, if any.
    pub default_module: Option<TypeId>,

    /// eframe native window options (native targets only).
    #[cfg(not(target_arch = "wasm32"))]
    pub native_options: eframe::NativeOptions,
    #[cfg(target_arch = "wasm32")]
    pub web_options: eframe::WebOptions,
}

impl<T> Options<T>
where
    T: App,
{
    /// Create options with the given window caption and web canvas id, leaving
    /// modules and platform options at their defaults.
    pub fn new(caption: String, canvas_id: String) -> Self {
        Options {
            caption,
            canvas_id,
            modules: None,
            default_module: None,
            #[cfg(not(target_arch = "wasm32"))]
            native_options: Default::default(),
            #[cfg(target_arch = "wasm32")]
            web_options: Default::default(), // .. Default::default()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Override the eframe native window options (native targets only).
    pub fn with_native_options(mut self, native_options: eframe::NativeOptions) -> Self {
        self.native_options = native_options;
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn with_web_options(mut self, web_options: eframe::WebOptions) -> Self {
        self.web_options = web_options;
        self
    }

    /// Register the application's modules and select the one identified by
    /// `default_module` as the initially active module.
    pub fn with_modules(mut self, default_module: TypeId, modules: Vec<Module<T>>) -> Self {
        self.default_module = Some(default_module);
        self.modules = Some(modules);
        self
    }
}
