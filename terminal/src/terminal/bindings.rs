//!
//! [xterm.js](http://xtermjs.org) [`mod@wasm_bindgen`] interface and plugin bindings
//!

use std::fmt::Debug;
use std::fmt::Formatter;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {

    /// Binding to the xterm.js `FitAddon` plugin that resizes the terminal to
    /// fit its container element.
    #[wasm_bindgen(js_namespace=["window", "FitAddon"], js_name="FitAddon")]
    pub type FitAddon;

    /// Constructs a new `FitAddon` instance.
    #[wasm_bindgen(
        constructor,
        js_class = "window.FitAddon.FitAddon",
        js_name = "FitAddon"
    )]
    pub fn new() -> FitAddon;

    /// Computes the terminal dimensions that fit the current container.
    #[wasm_bindgen(method, js_name = "proposeDimensions")]
    pub fn propose_dimensions(this: &FitAddon);

    /// Resizes the terminal to fit its container element.
    #[wasm_bindgen(method, js_name = "fit")]
    pub fn fit(this: &FitAddon);
}

#[wasm_bindgen]
extern "C" {

    /// Binding to the xterm.js `WebLinksAddon` plugin that turns URLs into
    /// clickable links.
    #[wasm_bindgen(js_namespace=["window","WebLinksAddon"], js_name="WebLinksAddon")]
    pub type WebLinksAddon;

    /// Constructs a new `WebLinksAddon` with the given link-click callback.
    #[wasm_bindgen(
        constructor,
        js_class = "window.WebLinksAddon.WebLinksAddon",
        js_name = "WebLinksAddon"
    )]
    pub fn new(callback: JsValue) -> WebLinksAddon;
}

#[wasm_bindgen]
extern "C" {
    /// Binding to the internal `_core` object of an xterm.js terminal.
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type XtermCoreImpl;
    /// Applies the given theme object directly to the terminal core.
    #[wasm_bindgen(method, js_name = "_setTheme")]
    pub fn set_theme(this: &XtermCoreImpl, them: js_sys::Object);

    /// Binding to the payload of an xterm.js key event.
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type XtermEvent;

    /// Returns the underlying DOM `KeyboardEvent` for this event.
    #[wasm_bindgen(method, getter, js_name = "domEvent")]
    pub fn get_dom_event(this: &XtermEvent) -> web_sys::KeyboardEvent;
    /// Returns the key string associated with this event.
    #[wasm_bindgen(method, getter, js_name = "key")]
    pub fn get_key(this: &XtermEvent) -> String;

    /// Binding to the xterm.js `Terminal` instance.
    #[wasm_bindgen(js_namespace=window, js_name="Terminal")]
    pub type XtermImpl;

    /// Constructs a new xterm.js `Terminal` with the given options object.
    #[wasm_bindgen(constructor, js_class = "Terminal")]
    pub fn new(opt: js_sys::Object) -> XtermImpl;

    /// Moves keyboard focus to the terminal.
    #[wasm_bindgen(method)]
    pub fn focus(this: &XtermImpl);

    /// Returns the terminal instance's number.
    #[wasm_bindgen(method, getter)]
    pub fn number(this: &XtermImpl) -> u32;

    /// Returns the terminal's internal `_core` object.
    #[wasm_bindgen(method, getter, js_name = "_core")]
    pub fn core(this: &XtermImpl) -> XtermCoreImpl;

    /// Opens (renders) the terminal within the given DOM element.
    #[wasm_bindgen(method)]
    pub fn open(this: &XtermImpl, el: &Element);

    /// Sets the named terminal option to the given value.
    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_option(this: &XtermImpl, name: &str, option: JsValue);

    /// Returns the value of the named terminal option.
    #[wasm_bindgen(method, js_name = "getOption")]
    pub fn get_option(this: &XtermImpl, name: &str) -> JsValue;

    /// Redraws the terminal rows in the inclusive range `start..=stop`.
    #[wasm_bindgen(method)]
    pub fn refresh(this: &XtermImpl, start: u32, stop: u32);

    /// Returns the number of rows currently displayed.
    #[wasm_bindgen(method, getter, js_name = "rows")]
    pub fn rows(this: &XtermImpl) -> u32;

    /// Returns the number of columns currently displayed.
    #[wasm_bindgen(method, getter, js_name = "cols")]
    pub fn cols(this: &XtermImpl) -> u32;

    /// Registers a callback invoked on each key event.
    #[wasm_bindgen(method, js_name = "onKey")]
    pub fn on_key(this: &XtermImpl, f: &js_sys::Function);

    #[wasm_bindgen(method, js_name = "write")]
    fn _write(this: &XtermImpl, text: String);

    // #[wasm_bindgen(method, js_name="paste")]
    // fn _paste(this: &XtermImpl, text:String);

    /// Loads the given addon into the terminal.
    #[wasm_bindgen(method, js_name = "loadAddon")]
    pub fn load_addon(this: &XtermImpl, addon: JsValue);

    /// Returns the DOM element hosting the terminal.
    #[wasm_bindgen(method, getter, js_name = "element")]
    pub fn get_element(this: &XtermImpl) -> Element;

    /// Returns the currently selected text.
    #[wasm_bindgen(method, js_name = "getSelection")]
    pub fn get_selection(this: &XtermImpl) -> String;

    /// Registers a regular-expression link matcher with the given click callback.
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
    /// Writes the given text to the terminal display.
    pub fn write<T: ToString>(&self, text: T) {
        self._write(text.to_string());
    }

    /// Applies the given object as the terminal's `theme` option.
    pub fn set_theme(&self, theme: js_sys::Object) {
        self.set_option("theme", theme.into());
        //self.core().set_theme(theme);
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object , js_name = ResizeObserver , typescript_type = "ResizeObserver")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Binding to the DOM [`ResizeObserver`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver) API.
    pub type ResizeObserver;
    /// Constructs a new `ResizeObserver` that invokes `callback` on size changes.
    #[wasm_bindgen(catch, constructor, js_class = "ResizeObserver")]
    pub fn new(callback: &::js_sys::Function) -> std::result::Result<ResizeObserver, JsValue>;
    /// Stops observing all target elements.
    #[wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = disconnect)]
    pub fn disconnect(this: &ResizeObserver);
    /// Begins observing size changes of the given element.
    #[wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = observe)]
    pub fn observe(this: &ResizeObserver, target: &Element);
    // # [wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = observe)]
    // pub fn observe_with_options(
    //     this: &ResizeObserver,
    //     target: &Element,
    //     options: &ResizeObserverOptions,
    // );
    /// Stops observing size changes of the given element.
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
