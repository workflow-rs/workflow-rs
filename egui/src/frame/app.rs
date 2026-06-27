use crate::imports::*;

/// Bundle of references passed around during a frame, giving access to the
/// runtime, the application, the egui context, the eframe frame, and the
/// current device.
pub struct Context<'r, T> {
    /// The async runtime backing the application.
    pub runtime: Runtime,
    /// Mutable reference to the application instance.
    pub app: &'r mut T,
    /// The egui context for the current frame.
    pub ctx: &'r egui::Context,
    /// The eframe frame for the current update.
    pub frame: &'r mut eframe::Frame,
    /// The device describing the current screen geometry and form factor.
    pub device: &'r mut Device,
}

/// Trait implemented by an application driving the egui frame. It defines the
/// lifecycle hooks (init, render, save, exit), input handling, and styling used
/// by the `Core` wrapper.
pub trait App: Sized + 'static {
    /// Returns the text styles to apply when rendering mobile-layout modules,
    /// or `None` to keep egui's current styles.
    fn mobile_text_styles(&self) -> Option<BTreeMap<egui::TextStyle, egui::FontId>> {
        None
    }
    /// Returns the text styles to apply when rendering desktop/default-layout
    /// modules, or `None` to keep egui's current styles.
    fn default_text_styles(&self) -> Option<BTreeMap<egui::TextStyle, egui::FontId>> {
        None
    }

    /// Handles a runtime event dispatched to the application.
    fn handle_event(&mut self, _ctx: &egui::Context, _event: RuntimeEvent) {}

    /// Handles a keyboard event, reporting the key, its pressed state, active
    /// modifiers, and whether the event is a key-repeat.
    fn handle_keyboard_events(
        &mut self,
        _key: egui::Key,
        _pressed: bool,
        _modifiers: &egui::Modifiers,
        _repeat: bool,
    ) {
    }

    /// Returns the device responsible for tracking screen geometry and form
    /// factor, if the application provides one.
    fn device(&mut self) -> Option<&mut Device> {
        None
    }

    /// Called once during startup to initialize the application with the
    /// runtime and the eframe creation context.
    fn init(&mut self, _runtime: &Runtime, _cc: &eframe::CreationContext<'_>) {}

    /// Renders the application's UI for the current frame. This is the primary
    /// per-frame entry point that each application must implement.
    fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);

    /// Called periodically (and on exit) to persist application state into the
    /// provided storage backend.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    /// Called once when the application is shutting down, allowing for cleanup.
    fn on_exit(&mut self) {}

    /// Time between automatic calls to [`Self::save`]
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    /// Returns the background clear color (normalized RGBA) used before each
    /// frame is rendered. Defaults to a translucent dark gray.
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    }

    /// Whether egui's memory (widget state, window positions, etc.) should be
    /// persisted across application restarts. Defaults to `true`.
    fn persist_egui_memory(&self) -> bool {
        true
    }

    /// Hook invoked with the raw input before it is processed each frame,
    /// allowing the application to inspect or mutate incoming events.
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}
