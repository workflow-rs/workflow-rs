use crate::imports::*;

pub struct Options<T> {
    pub caption: String,
    pub canvas_id: String,
    pub modules: Option<Vec<Module<T>>>,
    pub default_module: Option<TypeId>,

    #[cfg(not(target_arch = "wasm32"))]
    pub native_options: eframe::NativeOptions,
    #[cfg(target_arch = "wasm32")]
    pub web_options: eframe::WebOptions,
}

impl<T> Options<T>
where
    T: App,
{
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
    pub fn with_native_options(mut self, native_options: eframe::NativeOptions) -> Self {
        self.native_options = native_options;
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn with_web_options(mut self, web_options: eframe::WebOptions) -> Self {
        self.web_options = web_options;
        self
    }

    pub fn with_modules(mut self, default_module: TypeId, modules: Vec<Module<T>>) -> Self {
        self.default_module = Some(default_module);
        self.modules = Some(modules);
        self
    }
}
