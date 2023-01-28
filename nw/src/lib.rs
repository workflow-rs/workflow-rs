//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-nw.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-nw)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--nw-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-nw)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-nw.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/Node Webkit -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//!  Utilities and builders for developing Node Webkit applications
//! on top of [nw-sys](nw-sys) APIs. These modules simplify building
//! various application components by encapsulating creation processes
//! and allowing for use of the Rust builder patterns.
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
pub mod error;
pub mod media;
pub mod menu;
pub mod prelude;
pub mod result;
pub mod shortcut;
pub mod tray;
