//!
//! Media control helpers
//!
//! # Synopsis
//! ```rust
//!
//! // create Application instance
//! let app = Application::new()?;
//!
//! // choose desktop media
//! app.choose_desktop_media(
//!     nw_sys::screen::MediaSources::ScreenAndWindow,
//!     move |stream_id: Option<String>|->nw_sys::result::Result<()>{
//!         if let Some(stream_id) = stream_id{
//!             render_media(stream_id)?;
//!         }
//!         Ok(())
//!     }
//! )?;
//!
//! fn render_media(stream_id:String)->Result<()>{
//!     log_info!("stream_id: {:?}", stream_id);
//!      
//!     let video_element_id = "video_el".to_string();
//!     let video_constraints = VideoConstraints::new()
//!         .source_id(&stream_id)
//!         .max_height(1000);
//!
//!     workflow_nw::media::render_media(
//!         video_element_id,
//!         video_constraints,
//!         None,
//!         move |stream|->nw_sys::result::Result<()>{
//!             workflow_nw::application::app().unwrap().set_media_stream(stream)?;
//!             Ok(())
//!         }
//!     )?;
//!      
//!     Ok(())
//! }
//! ```

use crate::application::app;
use js_sys::Object;
use nw_sys::prelude::OptionsExt;
use nw_sys::result::Result;
use std::sync::Arc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MediaStream;
use workflow_dom::utils::{document, window};
use workflow_log::{log_debug, log_error};
use workflow_wasm::prelude::*;

/// MediaStream track kind
pub enum MediaStreamTrackKind {
    Video,
    Audio,
    All,
}

impl ToString for MediaStreamTrackKind {
    fn to_string(&self) -> String {
        match self {
            Self::Video => "Video".to_string(),
            Self::Audio => "Audio".to_string(),
            Self::All => "All".to_string(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    /// Video Constraints
    ///
    ///
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type VideoConstraints;
}

impl OptionsExt for VideoConstraints {}

impl VideoConstraints {
    /// Source Id
    ///
    ///
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn source_id(self, source_id: &str) -> Self {
        self.set("mandatory.chromeMediaSource", JsValue::from("desktop"))
            .set("mandatory.chromeMediaSourceId", JsValue::from(source_id))
    }

    /// Max Width
    ///
    ///
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn max_width(self, max_width: u32) -> Self {
        self.set("mandatory.maxWidth", JsValue::from(max_width))
    }

    /// Max Height
    ///
    ///
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn max_height(self, max_height: u32) -> Self {
        self.set("mandatory.maxHeight", JsValue::from(max_height))
    }

    /// Device Id
    ///
    /// a device ID or an array of device IDs which are acceptable and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn device_id(self, device_id: &str) -> Self {
        self.set("deviceId", JsValue::from(device_id))
    }

    /// Group Id
    ///
    /// a group ID or an array of group IDs which are acceptable and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn group_id(self, group_id: &str) -> Self {
        self.set("groupId", JsValue::from(group_id))
    }

    /// Aspect ratio of video
    ///
    /// specifying the video aspect ratio or range of aspect ratios
    /// which are acceptable and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn aspect_ratio(self, aspect_ratio: f32) -> Self {
        self.set("aspectRatio", JsValue::from(aspect_ratio))
    }

    /// Facing mode
    ///
    /// Object specifying a facing or an array of facings which are acceptable
    /// and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn facing_mode(self, facing_mode: &str) -> Self {
        self.set("facingMode", JsValue::from(facing_mode))
    }

    /// Frame rate
    ///
    /// frame rate or range of frame rates which are acceptable and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn frame_rate(self, frame_rate: f32) -> Self {
        self.set("frameRate", JsValue::from(frame_rate))
    }

    /// Width of video
    ///
    /// video width or range of widths which are acceptable and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn width(self, width: u16) -> Self {
        self.set("width", JsValue::from(width))
    }

    ///Height of video
    ///
    /// video height or range of heights which are acceptable and/or required.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn height(self, height: u16) -> Self {
        self.set("height", JsValue::from(height))
    }
}

/// Get user media
///
/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)
///
pub fn get_user_media(
    video_constraints: VideoConstraints,
    audio_constraints: Option<JsValue>,
    callback: Arc<dyn Fn(Option<MediaStream>)>,
) -> Result<()> {
    let app = match app() {
        Some(app) => app,
        None => return Err("app is not initialized".to_string().into()),
    };

    let navigator = window().navigator();
    let media_devices = navigator.media_devices()?;

    log_debug!("navigator: {:?}", navigator);
    log_debug!("media_devices: {:?}", media_devices);
    log_debug!("video_constraints: {:?}", video_constraints);

    let audio_constraints = audio_constraints.unwrap_or(JsValue::from(false));

    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints
        .audio(&audio_constraints)
        .video(&JsValue::from(&video_constraints));

    log_debug!("constraints: {:?}", constraints);

    let promise = media_devices.get_user_media_with_constraints(&constraints)?;

    let mut callback_ = Callback::default();
    let app_clone = app.clone();
    let callback_id = callback_.get_id();
    callback_.set_closure(move |value: JsValue| {
        let _ = app_clone.callbacks.remove(&callback_id);
        if let Ok(media_stream) = value.dyn_into::<MediaStream>() {
            callback(Some(media_stream));
        } else {
            callback(None);
        }
    });

    let binding = match callback_.closure() {
        Ok(b) => b,
        Err(err) => {
            return Err(format!(
                "media::get_user_media(), callback_.closure() failed, error: {:?}",
                err
            )
            .into());
        }
    };

    let _ = promise.then(binding.as_ref());

    app.callbacks.retain(callback_)?;
    Ok(())
}

/// render media to a video element
pub fn render_media<F>(
    video_element_id: String,
    video_constraints: VideoConstraints,
    audio_constraints: Option<JsValue>,
    callback: F,
) -> Result<()>
where
    F: 'static + Fn(Option<MediaStream>) -> Result<()>,
{
    get_user_media(
        video_constraints,
        audio_constraints,
        Arc::new(move |value| {
            let media_stream = if let Some(media_stream) = value {
                let el = document().get_element_by_id(&video_element_id).unwrap();
                match el.dyn_into::<web_sys::HtmlVideoElement>() {
                    Ok(el) => {
                        el.set_src_object(Some(&media_stream));
                    }
                    Err(err) => {
                        log_error!(
                            "Unable to cast element to HtmlVideoElement: element = {:?}",
                            err
                        );
                    }
                }

                Some(media_stream)
            } else {
                None
            };

            callback(media_stream)
                .map_err(|err| {
                    log_error!("render_media callback error: {:?}", err);
                })
                .ok();
        }),
    )?;
    Ok(())
}
