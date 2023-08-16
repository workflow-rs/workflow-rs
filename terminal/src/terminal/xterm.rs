use super::bindings::*;
use super::{LinkMatcherHandlerFn, Modifiers};
use crate::keys::Key;
use crate::terminal::Event;
use crate::terminal::EventHandlerFn;
use crate::terminal::Options;
use crate::terminal::TargetElement;
use crate::terminal::Terminal;
use crate::Result;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, MutexGuard};
use wasm_bindgen::JsValue;
use web_sys::Element;
use workflow_core::channel::{unbounded, Receiver, Sender};
use workflow_core::runtime;
use workflow_dom::clipboard;
use workflow_dom::inject::*;
use workflow_dom::utils::body;
use workflow_dom::utils::*;
use workflow_log::*;
use workflow_wasm::jserror::*;
use workflow_wasm::prelude::*;
use workflow_wasm::utils::*;

#[derive(Default)]
pub struct Theme {
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub selection: Option<String>,
    pub cursor: Option<String>,
}

pub enum ThemeOption {
    Background,
    Foreground,
    Selection,
    Cursor,
}
impl ThemeOption {
    pub fn list() -> Vec<Self> {
        Vec::from([
            Self::Background,
            Self::Foreground,
            Self::Selection,
            Self::Cursor,
        ])
    }
}

impl std::fmt::Display for ThemeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Background => write!(f, "Background"),
            Self::Foreground => write!(f, "Foreground"),
            Self::Selection => write!(f, "Selection"),
            Self::Cursor => write!(f, "Cursor"),
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get(&self, key: &ThemeOption) -> Option<String> {
        match key {
            ThemeOption::Background => self.background.clone(),
            ThemeOption::Foreground => self.foreground.clone(),
            ThemeOption::Selection => self.selection.clone(),
            ThemeOption::Cursor => self.cursor.clone(),
        }
    }
    pub fn set(&mut self, key: ThemeOption, value: Option<String>) {
        match key {
            ThemeOption::Background => {
                self.background = value;
            }
            ThemeOption::Foreground => {
                self.foreground = value;
            }
            ThemeOption::Selection => {
                self.selection = value;
            }
            ThemeOption::Cursor => {
                self.cursor = value;
            }
        }
    }
}

enum Ctl {
    SinkEvent(SinkEvent),
    Copy(Option<String>),
    Paste(Option<String>),
    Close,
}

#[derive(Debug)]
pub struct SinkEvent {
    key: String,
    term_key: String,
    ctrl_key: bool,
    alt_key: bool,
    meta_key: bool,
}

impl SinkEvent {
    fn new(key: String, term_key: String, ctrl_key: bool, alt_key: bool, meta_key: bool) -> Self {
        Self {
            key,
            term_key,
            ctrl_key,
            alt_key,
            meta_key,
        }
    }
}

#[derive(Clone)]
pub struct Sink {
    receiver: Receiver<Ctl>,
    sender: Sender<Ctl>,
}

impl Default for Sink {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        Sink { receiver, sender }
    }
}

pub struct ResizeObserverInfo {
    #[allow(dead_code)]
    observer: ResizeObserver,
    #[allow(dead_code)]
    callback: Callback<CallbackClosure<JsValue>>,
}

impl ResizeObserverInfo {
    pub fn new(observer: ResizeObserver, callback: Callback<CallbackClosure<JsValue>>) -> Self {
        Self { observer, callback }
    }
}

pub struct XtermOptions {
    pub font_size: Option<f64>,
    pub scrollback: Option<u32>,
}

///
/// # Xterm
///
/// Wrapper around XtermJS - <https://github.com/xtermjs/xterm.js>
///
/// TODO enhance API to match <https://github.com/xtermjs/xterm.js/blob/4.14.1/typings/xterm.d.ts>
///
///
pub struct Xterm {
    pub element: Element,
    xterm: Arc<Mutex<Option<XtermImpl>>>,
    terminal: Arc<Mutex<Option<Arc<Terminal>>>>,
    listener: Arc<Mutex<Option<Callback<CallbackClosure<XtermEvent>>>>>,
    sink: Arc<Sink>,
    resize: Arc<Mutex<Option<ResizeObserverInfo>>>,
    fit: Arc<Mutex<Option<FitAddon>>>,
    _web_links: Arc<Mutex<Option<WebLinksAddon>>>,
    terminate: Arc<AtomicBool>,
    disable_clipboard_handling: bool,
    callbacks: CallbackMap,
    defaults: XtermOptions,
    event_handler: Arc<Mutex<Option<EventHandlerFn>>>,
}

unsafe impl Send for Xterm {}
unsafe impl Sync for Xterm {}

impl Xterm {
    pub fn try_new() -> Result<Self> {
        Self::try_new_with_options(&Options::default())
    }

    pub fn try_new_with_options(options: &Options) -> Result<Self> {
        let el = match &options.element {
            TargetElement::Body => body().expect("Unable to get 'body' element"),
            TargetElement::Element(el) => el.clone(),
            TargetElement::TagName(tag) => document()
                .get_elements_by_tag_name(tag)
                .item(0)
                .ok_or("Unable to locate parent element for terminal")?,
            TargetElement::Id(id) => document()
                .get_element_by_id(id)
                .ok_or("Unable to locate parent element for terminal")?,
        };
        Self::try_new_with_element(&el, options)
    }

    pub fn try_new_with_element(parent: &Element, options: &Options) -> Result<Self> {
        let element = document().create_element("div")?;
        element.set_attribute("class", "terminal")?;
        parent.append_child(&element)?;
        let defaults = XtermOptions {
            font_size: options.font_size,
            scrollback: options.scrollback,
        };
        let terminal = Xterm {
            element,
            listener: Arc::new(Mutex::new(None)),
            xterm: Arc::new(Mutex::new(None)),
            terminal: Arc::new(Mutex::new(None)),
            sink: Arc::new(Sink::default()),
            resize: Arc::new(Mutex::new(None)),
            // addons: Arc::new(Mutex::new(Vec::new())),
            fit: Arc::new(Mutex::new(None)),
            _web_links: Arc::new(Mutex::new(None)),
            terminate: Arc::new(AtomicBool::new(false)),
            disable_clipboard_handling: options.disable_clipboard_handling,
            callbacks: CallbackMap::default(),
            event_handler: Arc::new(Mutex::new(None)),
            defaults,
        };
        Ok(terminal)
    }

    fn init_xterm(defaults: &XtermOptions) -> Result<XtermImpl> {
        let theme = js_sys::Object::new();
        let theme_opts = Vec::from([
            ("background", JsValue::from("rgba(255,255,255,1)")),
            ("foreground", JsValue::from("#000")),
            ("selection", JsValue::from("rgba(0,0,0,0.25)")),
            // ("background", JsValue::from("rgba(0,0,0,1)")),
            // ("foreground", JsValue::from("#FFF")),
            ("cursor", JsValue::from("#000")),
        ]);
        for (k, v) in theme_opts {
            js_sys::Reflect::set(&theme, &k.into(), &v)?;
        }

        let options = js_sys::Object::new();
        let opts = Vec::from([
            ("allowTransparency", JsValue::from(true)),
            (
                "fontFamily",
                JsValue::from("Consolas, Ubuntu Mono, courier-new, courier, monospace"),
            ),
            (
                "fontSize",
                JsValue::from(defaults.font_size.unwrap_or(20.0)),
            ),
            ("cursorBlink", JsValue::from(true)),
            ("theme", JsValue::from(theme)),
        ]);
        for (k, v) in opts {
            js_sys::Reflect::set(&options, &k.into(), &v)?;
        }
        if let Some(scrollback) = defaults.scrollback {
            js_sys::Reflect::set(&options, &"scrollback".into(), &JsValue::from(scrollback))?;
        }

        let term = XtermImpl::new(options);
        // log_trace!("term: {:?}", term);

        Ok(term)
    }

    pub fn xterm(&self) -> MutexGuard<Option<XtermImpl>> {
        self.xterm.lock().unwrap()
    }

    pub fn update_theme(&self) -> Result<()> {
        let el = self
            .xterm
            .lock()
            .unwrap()
            .as_ref()
            .expect("xterm is missing")
            .get_element();
        let css = window().get_computed_style(&el)?;
        //log_trace!("css: {:?}", css);
        if let Some(css) = css {
            let keys = Vec::from([
                ("background", "--workflow-terminal-background"),
                ("foreground", "--workflow-terminal-foreground"),
                ("cursor", "--workflow-terminal-cursor"),
                ("selection", "--workflow-terminal-selection"),
            ]);
            let theme_obj = js_sys::Object::new();
            for (key, css_var) in keys {
                if let Ok(value) = css.get_property_value(css_var) {
                    log_trace!("workflow-terminal: `{}`: {:?}", key, value);
                    js_sys::Reflect::set(
                        &theme_obj,
                        &JsValue::from(key),
                        &JsValue::from(value.trim()),
                    )?;
                }
            }

            let term = self.xterm.lock().unwrap();
            let term = term.as_ref().expect("xterm is missing");
            term.set_theme(theme_obj);
        }

        Ok(())
    }
    pub fn set_theme(&self, theme: Theme) -> Result<()> {
        let theme_obj = js_sys::Object::new();
        let properties = ThemeOption::list();

        for key in properties {
            if let Some(v) = theme.get(&key) {
                js_sys::Reflect::set(
                    &theme_obj,
                    &JsValue::from(key.to_string().to_lowercase()),
                    &JsValue::from(v),
                )?;
            }
        }

        let term = self.xterm.lock().unwrap();
        let term = term.as_ref().expect("xterm is missing");
        term.set_theme(theme_obj);
        Ok(())
    }

    fn init_addons(&self, xterm: &XtermImpl) -> Result<()> {
        let fit = FitAddon::new();
        xterm.load_addon(fit.clone());
        *self.fit.lock().unwrap() = Some(fit);
        Ok(())
    }

    pub async fn init(self: &Arc<Self>, terminal: &Arc<Terminal>) -> Result<()> {
        load_scripts().await?;

        let xterm = Self::init_xterm(&self.defaults)?;

        self.init_addons(&xterm)?;

        xterm.open(&self.element);
        xterm.focus();

        self.init_kbd_listener(&xterm)?;
        self.init_resize_observer()?;

        *self.xterm.lock().unwrap() = Some(xterm);
        *self.terminal.lock().unwrap() = Some(terminal.clone());

        Ok(())
    }

    pub fn set_option(&self, name: &str, option: JsValue) -> Result<()> {
        let xterm = self.xterm();
        let xterm = xterm.as_ref().expect("unable to get xterm");
        xterm.set_option(name, option);
        Ok(())
    }

    pub fn get_option(&self, name: &str) -> Result<JsValue> {
        let xterm = self.xterm();
        let xterm = xterm.as_ref().expect("unable to get xterm");
        Ok(xterm.get_option(name))
    }

    pub fn refresh(&self, start: u32, stop: u32) {
        let xterm = self.xterm();
        let xterm = xterm.as_ref().expect("unable to get xterm");
        xterm.refresh(start, stop);
    }

    fn event_handler(&self) -> Option<EventHandlerFn> {
        self.event_handler.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    pub(super) fn register_event_handler(self: &Arc<Self>, handler: EventHandlerFn) -> Result<()> {
        self.event_handler.lock().unwrap().replace(handler);
        Ok(())
    }

    #[allow(dead_code)]
    pub(super) fn register_link_matcher(
        self: &Arc<Self>,
        regexp: &js_sys::RegExp,
        handler: LinkMatcherHandlerFn,
    ) -> Result<()> {
        let xterm = self.xterm();
        let xterm = xterm.as_ref().expect("unable to get xterm");

        #[rustfmt::skip]
        let callback = callback!(
            move |e: web_sys::MouseEvent, link: String| -> std::result::Result<(), JsValue> {
                let modifiers = Modifiers {
                    shift: e.shift_key(),
                    ctrl: e.ctrl_key(),
                    alt: e.alt_key(),
                    meta: e.meta_key(),
                };
                handler(modifiers, link.as_str());
                Ok(())
            }
        );
        xterm.register_link_matcher(regexp, callback.as_ref());
        self.callbacks.retain(callback)?;

        Ok(())
    }

    pub fn paste(&self, text: Option<String>) -> Result<()> {
        self.sink
            .sender
            .try_send(Ctl::Paste(text))
            .map_err(|_| "Unable to send paste Ctl")?;
        Ok(())
    }

    fn init_resize_observer(self: &Arc<Self>) -> Result<()> {
        let this = self.clone();
        let resize_callback = callback!(move |_| -> std::result::Result<(), JsValue> {
            if let Err(err) = this.resize() {
                log_error!("Resize error: {:?}", err);
            }
            Ok(())
        });
        let resize_observer = ResizeObserver::new(resize_callback.as_ref())?;
        resize_observer.observe(&self.element);
        *self.resize.lock().unwrap() =
            Some(ResizeObserverInfo::new(resize_observer, resize_callback));

        Ok(())
    }

    fn init_kbd_listener(self: &Arc<Self>, xterm: &XtermImpl) -> Result<()> {
        let this = self.clone();
        let callback = callback!(move |e: XtermEvent| -> std::result::Result<(), JsValue> {
            //let term_key = try_get_string(&e, "key")?;
            let term_key = e.get_key();
            let dom_event = e.get_dom_event();
            let key = dom_event.key();
            let ctrl_key = dom_event.ctrl_key();
            let alt_key = dom_event.alt_key();
            let meta_key = dom_event.meta_key();

            if !this.disable_clipboard_handling {
                if (key == "v" || key == "v") && (ctrl_key || meta_key) {
                    this.sink
                        .sender
                        .try_send(Ctl::Paste(None))
                        .expect("Unable to send paste Ctl");
                    return Ok(());
                }
                if (key == "c" || key == "C") && (ctrl_key || meta_key) {
                    let text = this.xterm().as_ref().unwrap().get_selection();
                    log_info!("XXX getting clipboard text: '{text}'");

                    this.sink
                        .sender
                        .try_send(Ctl::Copy(Some(text)))
                        .expect("Unable to send copy Ctl");
                    return Ok(());
                }
            }

            this.sink
                .sender
                .try_send(Ctl::SinkEvent(SinkEvent::new(
                    key, term_key, ctrl_key, alt_key, meta_key,
                )))
                .unwrap();

            Ok(())
        });

        xterm.on_key(callback.as_ref());
        *self.listener.lock().unwrap() = Some(callback);

        Ok(())
    }

    pub fn terminal(&self) -> Arc<Terminal> {
        self.terminal.lock().unwrap().as_ref().unwrap().clone()
    }

    pub async fn run(self: &Arc<Self>) -> Result<()> {
        self.intake(&self.terminate).await?;
        Ok(())
    }

    pub async fn intake(self: &Arc<Self>, terminate: &Arc<AtomicBool>) -> Result<()> {
        loop {
            if terminate.load(Ordering::SeqCst) {
                break;
            }

            let event = self.sink.receiver.recv().await?;
            match event {
                Ctl::SinkEvent(event) => {
                    self.sink(event).await?;
                }
                Ctl::Copy(text) => {
                    let text =
                        text.unwrap_or_else(|| self.xterm().as_ref().unwrap().get_selection());
                    if runtime::is_nw() {
                        let clipboard = nw_sys::clipboard::get();
                        clipboard.set(&text);
                    } else if let Err(err) = clipboard::write_text(&text).await {
                        log_error!("{:?}", JsErrorData::from(err));
                    }

                    if let Some(handler) = self.event_handler() {
                        handler(Event::Copy);
                    }
                }
                Ctl::Paste(text) => {
                    if let Some(text) = text {
                        self.terminal().inject(text)?;
                    } else if runtime::is_nw() {
                        let clipboard = nw_sys::clipboard::get();
                        let text = clipboard.get();
                        if !text.is_empty() {
                            self.terminal().inject(text)?;
                        }
                    } else {
                        let data_js_value = clipboard::read_text().await;
                        if let Some(text) = data_js_value.as_string() {
                            self.terminal().inject(text)?;
                        }
                    }

                    if let Some(handler) = self.event_handler() {
                        handler(Event::Copy);
                    }
                }
                Ctl::Close => {
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn exit(&self) {
        self.terminate.store(true, Ordering::SeqCst);
        self.sink
            .sender
            .try_send(Ctl::Close)
            .expect("Unable to send exit Ctl");
    }

    async fn sink(&self, e: SinkEvent) -> Result<()> {
        let key = match e.key.as_str() {
            "Backspace" => Key::Backspace,
            "ArrowUp" => Key::ArrowUp,
            "ArrowDown" => Key::ArrowDown,
            "ArrowLeft" => Key::ArrowLeft,
            "ArrowRight" => Key::ArrowRight,
            "Escape" => Key::Esc,
            "Delete" => Key::Delete,
            "Tab" => {
                // TODO implement completion handler
                return Ok(());
            }
            "Enter" => Key::Enter,
            _ => {
                let printable = !e.meta_key; // ! (e.ctrl_key || e.alt_key || e.meta_key);
                if !printable {
                    return Ok(());
                }
                //log_trace!("e:{:?}", e);
                if let Some(c) = e.key.chars().next() {
                    if e.ctrl_key {
                        Key::Ctrl(c)
                    } else if e.alt_key {
                        Key::Alt(c)
                    } else {
                        Key::Char(c)
                    }
                } else {
                    return Ok(());
                }
            }
        };

        self.terminal().ingest(key, e.term_key).await?;

        Ok(())
    }

    pub fn write<S>(&self, s: S)
    where
        S: Into<String>,
    {
        self.xterm
            .lock()
            .unwrap()
            .as_ref()
            .expect("Xterm is not initialized")
            .write(s.into());
    }

    pub fn measure(&self) -> Result<()> {
        let xterm = self.xterm.lock().unwrap();
        let xterm = xterm.as_ref().unwrap();
        let core = try_get_js_value_prop(xterm, "_core").expect("Unable to get xterm core");
        let char_size_service = try_get_js_value_prop(&core, "_charSizeService")
            .expect("Unable to get xterm charSizeService");
        let has_valid_size = try_get_js_value_prop(&char_size_service, "hasValidSize")
            .expect("Unable to get xterm charSizeService::hasValidSize");
        if has_valid_size.is_falsy() {
            apply_with_args0(&char_size_service, "measure")?;
        }

        Ok(())
    }

    pub fn resize(&self) -> Result<()> {
        //self.measure()?;
        if let Some(xterm) = self.xterm.lock().unwrap().as_ref() {
            let el = xterm.get_element();
            let height = el.client_height();
            if height < 1 {
                return Ok(());
            }
        } else {
            return Ok(());
        }

        let fit = self.fit.lock().unwrap();
        let fit = fit.as_ref().unwrap();
        // TODO review if this is correct
        //fit.propose_dimensions();
        // TODO review if this is correct
        fit.fit();

        Ok(())
    }

    pub fn get_font_size(&self) -> Result<Option<f64>> {
        let font_size = self.get_option("fontSize")?;
        Ok(font_size.as_f64())
    }

    pub fn set_font_size(&self, font_size: f64) -> Result<()> {
        self.set_option("fontSize", JsValue::from_f64(font_size))
    }

    pub fn cols(&self) -> Option<usize> {
        self.xterm().as_ref().map(|xterm| xterm.cols() as usize)
    }

    pub fn rows(&self) -> Option<usize> {
        self.xterm().as_ref().map(|xterm| xterm.rows() as usize)
    }

    fn adjust_font_size(&self, delta: f64) -> Result<Option<f64>> {
        let font_size = self.get_option("fontSize")?;
        let mut font_size = font_size.as_f64().ok_or("Unable to get font size")?;
        font_size += delta;
        if font_size < 4.0 {
            font_size = 4.0;
        }

        self.set_option("fontSize", JsValue::from_f64(font_size))?;
        self.resize()?;
        Ok(Some(font_size))
    }

    pub fn increase_font_size(&self) -> Result<Option<f64>> {
        self.adjust_font_size(1.0)
    }

    pub fn decrease_font_size(&self) -> Result<Option<f64>> {
        self.adjust_font_size(-1.0)
    }

    pub fn clipboard_copy(&self) -> Result<()> {
        let text = self.xterm().as_ref().unwrap().get_selection();
        self.sink
            .sender
            .try_send(Ctl::Copy(Some(text)))
            .expect("Unable to send copy Ctl");
        // log_info!("clipboard_copy inside xterm, sending notification");
        if let Some(handler) = self.event_handler() {
            log_info!("clipboard_copy inside xterm, sending notification - DONE");
            handler(Event::Copy);
        }

        Ok(())
    }

    pub fn clipboard_paste(&self) -> Result<()> {
        self.sink
            .sender
            .try_send(Ctl::Paste(None))
            .expect("Unable to send paste Ctl");
        if let Some(handler) = self.event_handler() {
            handler(Event::Paste);
        }

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

static mut XTERM_LOADED: bool = false;

pub async fn load_scripts() -> Result<()> {
    if unsafe { XTERM_LOADED } {
        return Ok(());
    }

    let xterm_js = include_bytes!("../../extern/resources/xterm.js");
    inject_blob(Content::Script(None, xterm_js)).await?;
    let xterm_addon_fit_js = include_bytes!("../../extern/resources/xterm-addon-fit.js");
    inject_blob(Content::Script(None, xterm_addon_fit_js)).await?;
    let xterm_addon_web_links_js =
        include_bytes!("../../extern/resources/xterm-addon-web-links.js");
    inject_blob(Content::Script(None, xterm_addon_web_links_js)).await?;
    let xterm_css = include_bytes!("../../extern/resources/xterm.css");
    inject_blob(Content::Style(None, xterm_css)).await?;

    unsafe { XTERM_LOADED = true };

    Ok(())
}
