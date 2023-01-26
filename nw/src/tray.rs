//!
//! Builder for the application system Tray menu.
//!
//! # Synopsis
//! ```rust
//! // create Tray icon menu without submenus
//! TrayMenuBuilder::new()
//!     .icon("resources/icons/tray-icon@2x.png")
//!     .icons_are_templates(false)
//!     .callback(|_|{
//!         window().alert_with_message("Tray menu click")?;
//!         Ok(())
//!     })
//!     .build()?;
//!
//! // create Tray menu icon with submenus
//! let submenu_1 = MenuItemBuilder::new()
//!     .label("Say hi")
//!     .key("6")
//!     .modifiers("ctrl")
//!     .callback(move |_|->std::result::Result<(), JsValue>{
//!         window().alert_with_message("hi")?;
//!         Ok(())
//!     }).build()?;
//!     
//! let exit_menu = MenuItemBuilder::new()
//!     .label("Exit")
//!     .callback(move |_|->std::result::Result<(), JsValue>{
//!         nw_sys::app::close_all_windows();
//!         Ok(())
//!     }).build()?;
//!     
//! let _tray = TrayMenuBuilder::new()
//!     .icon("resources/icons/tray-icon@2x.png")
//!     .icons_are_templates(false)
//!     .submenus(vec![submenu_1, menu_separator(), exit_menu])
//!     .build()?;
//!
//! ```
//!

use crate::application::app;
use nw_sys::{menu_item::MenuItem, tray::Options, Menu, Tray};
use nw_sys::{prelude::*, result::Result};
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use workflow_wasm::prelude::*;

/// Provides a builder pattern for constructing a system tray menu
/// for the application.
///
/// For usage example please refer to [Examples](self)
pub struct TrayMenuBuilder {
    pub options: Options,
    pub menu: Option<Menu>,
    pub tooltip: Option<String>,
    pub callback: Option<Callback<CallbackClosure<MouseEvent>>>,
}

impl TrayMenuBuilder {
    pub fn new() -> Self {
        Self {
            options: Options::new(),
            menu: None,
            tooltip: None,
            callback: None,
        }
    }

    pub fn set(mut self, key: &str, value: JsValue) -> Self {
        self.options = self.options.set(key, value);
        self
    }

    /// Set the title of the tray.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytitle)
    pub fn title(self, title: &str) -> Self {
        self.set("title", JsValue::from(title))
    }

    /// Set the tooltip of the tray. tooltip shows when you hover the Tray with mouse.
    ///
    /// Note: tooltip is showed on all three platforms.
    /// Should be set as Tray property rather from option object constructor.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytooltip)
    pub fn tooltip(mut self, tooltip: &str) -> Self {
        self = self.set("tooltip", JsValue::from(tooltip));
        self.tooltip = Some(tooltip.to_string());

        self
    }

    /// Set the icon of the tray, icon must receive a path to your icon file.
    /// It can be a relative path which points to an icon in your app,
    /// or an absolute path pointing to a file in user’s system.
    ///
    /// Mac OS X caveat: when used in notification context,
    /// png icon is not sized down like in windows notification area,
    /// it is rather displayed in 1:1 ratio.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayicon)
    pub fn icon(self, icon: &str) -> Self {
        self.set("icon", JsValue::from(icon))
    }

    /// (Mac) Set the alternate (active) tray icon.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayalticon-mac)
    pub fn alticon(self, alticon: &str) -> Self {
        self.set("alticon", JsValue::from(alticon))
    }

    /// (Mac) Set whether icon and alticon images are treated as "templates" (true by default).
    /// When the property is set to true the images are treated as “templates”
    /// and the system automatically ensures proper styling according to the various
    /// states of the status item (e.g. dark menu, light menu, etc.).
    /// Template images should consist only of black and clear colours
    /// and can use the alpha channel in the image to adjust the opacity of black content.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayiconsaretemplates-mac)
    pub fn icons_are_templates(self, icons_are_templates: bool) -> Self {
        self.set("iconsAreTemplates", JsValue::from(icons_are_templates))
    }

    /// Set the menu of the tray, menu will be showed when you click on the tray icon.
    ///
    /// On Mac OS X the menu will be showed when you click on the
    /// tray (which is the only action available for tray icons on Mac OS X).
    /// On Windows and Linux, the menu will be showed when you single click on the
    /// tray with right mouse button, clicking with left mouse button sends the click
    /// event and does not show a menu.
    ///
    /// In order to reduce differences from different platforms, setting menu property
    /// is the only way to bind a menu to tray, there’s no way to popup a menu with
    /// left mouse button click on Linux and Windows.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traymenu)
    pub fn menu(mut self, menu: Menu) -> Self {
        self.menu = Some(menu);
        self
    }

    /// The callback function when tray icon is clicked.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#event-click)
    pub fn callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut(MouseEvent) -> std::result::Result<(), JsValue> + 'static,
    {
        self.callback = Some(Callback::new(callback));

        self
    }

    /// A submenu
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traymenu)
    pub fn submenus(self, items: Vec<MenuItem>) -> Self {
        let submenu = nw_sys::Menu::new();
        for menu_item in items {
            submenu.append(&menu_item);
        }

        self.menu(submenu)
    }

    pub fn build_impl(self) -> Result<(Tray, Option<Callback<CallbackClosure<MouseEvent>>>)> {
        let tray = Tray::new(&self.options);

        if let Some(menu) = self.menu {
            tray.set_menu(&menu);
        }
        if let Some(tooltip) = self.tooltip {
            tray.set_tooltip(&tooltip);
        }

        if let Some(callback) = self.callback {
            tray.on("click", callback.as_ref());
            Ok((tray, Some(callback)))
        } else {
            Ok((tray, None))
        }
    }

    pub fn build(self) -> Result<Tray> {
        let (tray, callback) = self.build_impl()?;

        if let Some(callback) = callback {
            let app = match app() {
                Some(app) => app,
                None => return Err("app is not initialized".to_string().into()),
            };
            app.callbacks.retain(callback)?;
        }

        Ok(tray)
    }

    pub fn finalize(self) -> Result<(Tray, Option<Callback<CallbackClosure<MouseEvent>>>)> {
        let (tray, callback) = self.build_impl()?;

        Ok((tray, callback))
    }
}
