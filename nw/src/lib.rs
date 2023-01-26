//! Utilities and builders for developing Node Webkit applications
//! on top of [nw-sys](nw-sys) framework. These modules simplify building
//! various application components.
//!
//! # Synopsis
//!
//! ```rust
//!
//! // create Application instance
//! let app = Application::new()?;
//!  
//! // store MediaStream
//! app.set_media_stream(None)?;
//!  
//! // get MediaStream as `Option<MediaStream>`
//! let _media_stream = app.get_media_stream()?;
//!  
//! // stop saved MediaStream
//! app.stop_media_stream(None, None)?;
//!  
//! // create window
//! let options = nw_sys::window::Options::new()
//!     .title("My App")
//!     .width(200)
//!     .height(200)
//!     .left(0)
//!     .top(100);
//!
//! app.create_window_with_callback(
//!     "/root/index.html",
//!     &options,
//!     move |_win|->std::result::Result<(), JsValue>{
//!         //log_info!("window created");
//!         Ok(())
//!     }
//! )?;
//!  
//! // create context menu
//! let item_1 = MenuItemBuilder::new()
//!     .label("Sub Menu 1")
//!     .callback(move |_|->std::result::Result<(), JsValue>{
//!         window().alert_with_message("Context menu 1 clicked")?;
//!         Ok(())
//!     }).build()?;
//!      
//! let item_2 = MenuItemBuilder::new()
//!     .label("Sub Menu 2")
//!     .callback(move |_|->std::result::Result<(), JsValue>{
//!         window().alert_with_message("Context menu 2 clicked")?;
//!         Ok(())
//!     }).build()?;
//!      
//! app.create_context_menu(vec![item_1, item_2])?;
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
//!
//!
//!
//! ```

pub mod application;
pub mod media;
pub mod menu;
pub mod prelude;
pub mod shortcut;
pub mod tray;

pub use workflow_wasm::prelude::*;
