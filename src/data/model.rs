use egui::Ui;
use serde::Serialize;
use crate::util::guid_str;

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
                ui.checkbox(&mut self.status, "".to_string());
            } else {
                if ui.label("v".to_string()).clicked() {
                    self.status = false;
                }
            }
            ui.label(self.content.clone());
            ui.label("...".to_string());
        });
    }
}
