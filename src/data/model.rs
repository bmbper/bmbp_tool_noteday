use crate::util::guid_str;
use egui::{Rounding, Ui};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ItemRecord {
    pub day: String,
    pub id: String,
    pub content: String,
    pub status: bool,
}

impl ItemRecord {
    pub fn with_day(day: String) -> Self {
        ItemRecord {
            id: guid_str(),
            content: "".to_string(),
            status: false,
            day,
        }
    }
}

impl ItemRecord {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if !self.status {
                let checked_img = egui::include_image!("../../image/checkbox_uncheck.png");
                if ui.image(checked_img).clicked() {
                    self.status = true;
                }
            } else {
                let checked_img = egui::include_image!("../../image/checkbox_checked.png");
                if ui.image(checked_img).clicked() {
                    self.status = false;
                }
            }
            ui.set_width(120.0);
            ui.label(self.content.clone());
            ui.label("...".to_string());
        });
    }
}
