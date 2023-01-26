//!
//! Node Webkit application helper provided by the [`Application`] struct.
//!
//!
//!

use crate::media::MediaStreamTrackKind;
use nw_sys::{prelude::*, result::Result, utils};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MediaStream, MediaStreamTrack, MouseEvent};
use workflow_wasm::prelude::*;

static mut APP: Option<Arc<Application>> = None;

/// get saved [Application](Application) instance.
pub fn app() -> Option<Arc<Application>> {
    unsafe { APP.clone() }
}

/// Application helper. This struct contains a map of callbacks that
/// can be used to retain different application callbacks as well as
/// media stream helper functions for controlling media playback.
///
/// For usage example please refer to [Examples](crate)
#[derive(Clone)]
pub struct Application {
    /// a storage for [MediaStream](web_sys::MediaStream)
    pub media_stream: Arc<Mutex<Option<MediaStream>>>,

    /// holds references to [Callback](workflow_wasm::callback::Callback)
    pub callbacks: CallbackMap,
}

impl Application {
    /// Create [Application](Application) object.
    /// if instance is allready created then it will return saved application.
    pub fn new() -> Result<Arc<Self>> {
        if let Some(app) = app() {
            return Ok(app);
        }
        let app = Arc::new(Self {
            callbacks: CallbackMap::new(),
            media_stream: Arc::new(Mutex::new(None)),
        });

        unsafe {
            APP = Some(app.clone());
        };

        Ok(app)
    }

    /// Store or Clear saved [MediaStream](web_sys::MediaStream)
    pub fn set_media_stream(&self, media_stream: Option<MediaStream>) -> Result<()> {
        *self.media_stream.lock()? = media_stream;
        Ok(())
    }

    /// Get saved [MediaStream](web_sys::MediaStream)
    pub fn get_media_stream(&self) -> Result<Option<MediaStream>> {
        let media_stream = self.media_stream.lock()?.clone();
        Ok(media_stream)
    }

    /// Stop [MediaStream](web_sys::MediaStream) tracks ([MediaStreamTrack](web_sys::MediaStreamTrack))
    /// of given kind or [All](MediaStreamTrackKind::All)
    /// you can provide any [MediaStream](web_sys::MediaStream) or it will get internal saved stream.
    pub fn stop_media_stream(
        &self,
        track_kind: Option<MediaStreamTrackKind>,
        mut stream: Option<MediaStream>,
    ) -> Result<()> {
        if stream.is_none() {
            stream = self.get_media_stream()?;
        }
        if let Some(media_stream) = stream {
            let tracks = media_stream.get_tracks();
            let kind = track_kind.unwrap_or(MediaStreamTrackKind::All);
            let mut all = false;
            let mut video = false;
            let mut audio = false;
            match kind {
                MediaStreamTrackKind::All => {
                    all = true;
                }
                MediaStreamTrackKind::Video => {
                    video = true;
                }
                MediaStreamTrackKind::Audio => {
                    audio = true;
                }
            }

            for index in 0..tracks.length() {
                if let Ok(track) = tracks.get(index).dyn_into::<MediaStreamTrack>() {
                    let k = track.kind();
                    if all || (k.eq("video") && video) || (k.eq("audio") && audio) {
                        track.stop();
                    }
                }
            }
        }
        Ok(())
    }

    /// Create window with given [Options](nw_sys::window::Options)
    /// and callback closure
    pub fn create_window_with_callback<F>(
        &self,
        url: &str,
        option: &nw_sys::window::Options,
        callback: F,
    ) -> Result<()>
    where
        F: FnMut(nw_sys::Window) -> std::result::Result<(), JsValue> + 'static,
    {
        let callback = Callback::new(callback);

        nw_sys::window::open_with_options_and_callback(url, option, callback.as_ref());

        self.callbacks.retain(callback)?;
        Ok(())
    }

    /// Create window with given [Options](nw_sys::window::Options)
    pub fn create_window(url: &str, option: &nw_sys::window::Options) -> Result<()> {
        nw_sys::window::open_with_options(url, option);

        Ok(())
    }

    /// Create context menu
    pub fn create_context_menu(&self, menus: Vec<nw_sys::MenuItem>) -> Result<()> {
        let popup_menu = nw_sys::Menu::new();
        for menu_item in menus {
            popup_menu.append(&menu_item);
        }

        self.on_context_menu(move |ev: MouseEvent| -> std::result::Result<(), JsValue> {
            ev.prevent_default();
            popup_menu.popup(ev.x(), ev.y());
            Ok(())
        })?;

        Ok(())
    }

    /// A utility for adding callback for `contextmenu` event
    pub fn on_context_menu<F>(&self, callback: F) -> Result<()>
    where
        F: Sized + FnMut(MouseEvent) -> std::result::Result<(), JsValue> + 'static,
    {
        let win = nw_sys::window::get();
        let dom_win = win.window();
        let body = utils::body(Some(dom_win));

        let callback = callback!(callback);
        body.add_event_listener_with_callback("contextmenu", callback.as_ref())?;
        self.callbacks.retain(callback)?;

        Ok(())
    }

    /// Choose desktop media
    ///
    /// Screen sharing by selection; Currently only working in Windows and OSX
    /// and some linux distribution.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screenchoosedesktopmedia-sources-callback)
    ///
    pub fn choose_desktop_media<F>(
        &self,
        sources: nw_sys::screen::MediaSources,
        mut callback: F,
    ) -> Result<()>
    where
        F: 'static + FnMut(Option<String>) -> Result<()>,
    {
        let callback_ = Callback::new(move |value: JsValue| -> std::result::Result<(), JsValue> {
            let mut stream_id = None;
            if value.is_string() {
                if let Some(id) = value.as_string() {
                    if id.len() > 0 {
                        stream_id = Some(id);
                    }
                }
            }

            callback(stream_id)?;

            Ok(())
        });

        nw_sys::screen::choose_desktop_media(sources, callback_.as_ref())?;

        self.callbacks.retain(callback_)?;

        Ok(())
    }
}
