use egui::{FontData, FontDefinitions, FontFamily};

pub trait RegisterStaticFont {
    fn add_static(&mut self, family: FontFamily, name: &str, bytes: &'static [u8]);
}

impl RegisterStaticFont for FontDefinitions {
    fn add_static(&mut self, family: FontFamily, name: &str, bytes: &'static [u8]) {
        self.font_data
            .insert(name.to_owned(), FontData::from_static(bytes));

        self.families
            .entry(family)
            .or_default()
            .push(name.to_owned());
    }
}
