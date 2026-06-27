use {
    egui::{FontData, FontDefinitions, FontFamily},
    std::sync::Arc,
};

/// Extension trait for registering statically embedded font data with egui.
pub trait RegisterStaticFont {
    /// Register `bytes` as a font under `name` and append it to the given
    /// [`FontFamily`]'s fallback list.
    fn add_static(&mut self, family: FontFamily, name: &str, bytes: &'static [u8]);
}

impl RegisterStaticFont for FontDefinitions {
    fn add_static(&mut self, family: FontFamily, name: &str, bytes: &'static [u8]) {
        self.font_data
            .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));

        self.families
            .entry(family)
            .or_default()
            .push(name.to_owned());
    }
}
