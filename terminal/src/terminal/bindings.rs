//!
//! [xterm.js](http://xtermjs.org) [`mod@wasm_bindgen`] interface and plugin bindings
//!

use std::fmt::Debug;
use std::fmt::Formatter;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=["window", "FitAddon"], js_name="FitAddon")]
    pub type FitAddon;

    #[wasm_bindgen(
        constructor,
        js_class = "window.FitAddon.FitAddon",
        js_name = "FitAddon"
    )]
    pub fn new() -> FitAddon;

    #[wasm_bindgen(method, js_name = "proposeDimensions")]
    pub fn propose_dimensions(this: &FitAddon);

    #[wasm_bindgen(method, js_name = "fit")]
    pub fn fit(this: &FitAddon);
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=["window","WebLinksAddon"], js_name="WebLinksAddon")]
    pub type WebLinksAddon;

    #[wasm_bindgen(
        constructor,
        js_class = "window.WebLinksAddon.WebLinksAddon",
        js_name = "WebLinksAddon"
    )]
    pub fn new(callback: JsValue) -> WebLinksAddon;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type XtermCoreImpl;
    #[wasm_bindgen(method, js_name = "_setTheme")]
    pub fn set_theme(this: &XtermCoreImpl, them: js_sys::Object);

    #[wasm_bindgen(extends = js_sys::Object)]
    pub type XtermEvent;

    #[wasm_bindgen(method, getter, js_name = "domEvent")]
    pub fn get_dom_event(this: &XtermEvent) -> web_sys::KeyboardEvent;
    #[wasm_bindgen(method, getter, js_name = "key")]
    pub fn get_key(this: &XtermEvent) -> String;

    #[wasm_bindgen(js_namespace=window, js_name="Terminal")]
    pub type XtermImpl;

    #[wasm_bindgen(constructor, js_class = "Terminal")]
    pub fn new(opt: js_sys::Object) -> XtermImpl;

    #[wasm_bindgen(method)]
    pub fn focus(this: &XtermImpl);

    #[wasm_bindgen(method, getter)]
    pub fn number(this: &XtermImpl) -> u32;

    #[wasm_bindgen(method, getter, js_name = "_core")]
    pub fn core(this: &XtermImpl) -> XtermCoreImpl;

    #[wasm_bindgen(method)]
    pub fn open(this: &XtermImpl, el: &Element);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_option(this: &XtermImpl, name: &str, option: JsValue);

    #[wasm_bindgen(method, js_name = "getOption")]
    pub fn get_option(this: &XtermImpl, name: &str) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn refresh(this: &XtermImpl, start: u32, stop: u32);

    #[wasm_bindgen(method, getter, js_name = "rows")]
    pub fn rows(this: &XtermImpl) -> u32;

    #[wasm_bindgen(method, getter, js_name = "cols")]
    pub fn cols(this: &XtermImpl) -> u32;

    #[wasm_bindgen(method, js_name = "onKey")]
    pub fn on_key(this: &XtermImpl, f: &js_sys::Function);

    #[wasm_bindgen(method, js_name = "write")]
    fn _write(this: &XtermImpl, text: String);

    // #[wasm_bindgen(method, js_name="paste")]
    // fn _paste(this: &XtermImpl, text:String);

    #[wasm_bindgen(method, js_name = "loadAddon")]
    pub fn load_addon(this: &XtermImpl, addon: JsValue);

    #[wasm_bindgen(method, getter, js_name = "element")]
    pub fn get_element(this: &XtermImpl) -> Element;

    #[wasm_bindgen(method, js_name = "getSelection")]
    pub fn get_selection(this: &XtermImpl) -> String;

    #[wasm_bindgen(method, js_name = "registerLinkMatcher")]
    pub fn register_link_matcher(
        this: &XtermImpl,
        regexp: &js_sys::RegExp,
        callback: &js_sys::Function,
    );

    // future versions of xterm.js
    // #[wasm_bindgen(method, js_name = "getSelectionService")]
    // pub fn get_selection_service(this: &XtermImpl) -> SelectionService;
    // #[wasm_bindgen(extends = js_sys::Object)]
    // pub type SelectionService;
    // #[wasm_bindgen(method, js_name = "getSelection")]
    // pub fn get_selection(this: &SelectionService) -> String;

}

impl Debug for XtermImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Workflow Xterm")?;
        Ok(())
    }
}

impl XtermImpl {
    pub fn write<T: Into<String>>(&self, text: T) {
        self._write(text.into());
    }

    pub fn set_theme(&self, theme: js_sys::Object) {
        self.set_option("theme", theme.into());
        //self.core().set_theme(theme);
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object , js_name = ResizeObserver , typescript_type = "ResizeObserver")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ResizeObserver;
    #[wasm_bindgen(catch, constructor, js_class = "ResizeObserver")]
    pub fn new(callback: &::js_sys::Function) -> std::result::Result<ResizeObserver, JsValue>;
    #[wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = disconnect)]
    pub fn disconnect(this: &ResizeObserver);
    #[wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = observe)]
    pub fn observe(this: &ResizeObserver, target: &Element);
    // # [wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = observe)]
    // pub fn observe_with_options(
    //     this: &ResizeObserver,
    //     target: &Element,
    //     options: &ResizeObserverOptions,
    // );
    // # [wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = unobserve)]
    pub fn unobserve(this: &ResizeObserver, target: &Element);
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen (extends = web_sys::Event , extends = :: js_sys :: Object , js_name = ClipboardEvent , typescript_type = "ClipboardEvent")]
//     #[derive(Debug, Clone, PartialEq, Eq)]
//     pub type ClipboardEvent;
//     #[wasm_bindgen (structural , method , getter , js_class = "ClipboardEvent" , js_name = clipboardData)]
//     pub fn clipboard_data(this: &ClipboardEvent) -> Option<web_sys::DataTransfer>;
//     #[wasm_bindgen(catch, constructor, js_class = "ClipboardEvent")]
//     pub fn new(type_: &str) -> Result<ClipboardEvent, JsValue>;
//     // #[wasm_bindgen(catch, constructor, js_class = "ClipboardEvent")]
//     // pub fn new_with_event_init_dict(
//     //     type_: &str,
//     //     event_init_dict: &ClipboardEventInit,
//     // ) -> Result<ClipboardEvent, JsValue>;
// }
