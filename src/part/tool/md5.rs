use crate::part::NotePartView;
use egui::{style::Spacing, RichText, Ui};
use egui_extras::StripBuilder;
pub struct Md5View {
    source: String,
    target: String,
}
impl Md5View {
    pub fn new() -> Self {
        Md5View {
            source: "".to_string(),
            target: "".to_string(),
        }
    }
}
impl NotePartView for Md5View {
    fn title(&mut self) -> String {
        "MD5".to_string()
    }
    fn show(&mut self, ui: &mut Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.set_height(24.0);
                ui.label("原字符串:");
            });
            ui.horizontal(|ui| {
                ui.set_height(24.0);
                ui.text_edit_singleline(&mut self.source);
            });
            ui.horizontal(|ui| {
                ui.set_height(24.0);
                ui.label("加密符串:");
            });
            ui.horizontal(|ui| {
                ui.set_height(24.0);
                let label = egui::Label::new(RichText::new(self.target.clone()))
                    .wrap_mode(egui::TextWrapMode::Truncate);
                ui.add(label);
            });
            ui.horizontal(|ui| {
                ui.set_height(36.0);
                if ui.button("小写").clicked() && !self.source.is_empty() {
                    self.target =
                        format!("{:x}", md5::compute(self.source.as_bytes())).to_lowercase();
                };
                if ui.button("大写").clicked() && !self.source.is_empty() {
                    self.target =
                        format!("{:x}", md5::compute(self.source.as_bytes())).to_uppercase();
                };
            });
        });
    }
}
