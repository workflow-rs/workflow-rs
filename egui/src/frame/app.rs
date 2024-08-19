use crate::imports::*;

pub struct Context<'r, T> {
    pub runtime: Runtime,
    pub app: &'r mut T,
    pub ctx: &'r egui::Context,
    pub frame: &'r mut eframe::Frame,
    pub device: &'r mut Device,
}

pub trait App: Sized + 'static {
    fn mobile_text_styles(&self) -> Option<BTreeMap<egui::TextStyle, egui::FontId>> {
        None
    }
    fn default_text_styles(&self) -> Option<BTreeMap<egui::TextStyle, egui::FontId>> {
        None
    }

    fn handle_event(&mut self, _ctx: &egui::Context, _event: RuntimeEvent) {}

    fn handle_keyboard_events(
        &mut self,
        _key: egui::Key,
        _pressed: bool,
        _modifiers: &egui::Modifiers,
        _repeat: bool,
    ) {
    }

    fn device(&mut self) -> Option<&mut Device> {
        None
    }

    fn init(&mut self, _runtime: &Runtime, _cc: &eframe::CreationContext<'_>) {}

    fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self) {}

    /// Time between automatic calls to [`Self::save`]
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}
