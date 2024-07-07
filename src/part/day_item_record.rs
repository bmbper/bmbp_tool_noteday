use crate::{data::ItemRecord, util::guid_str};
use egui::{Color32, Stroke, Ui};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct DayItemRecord {
    pub day: String,
    pub data: ItemRecord,
}

impl DayItemRecord {
    pub fn with_day(day: String) -> Self {
        DayItemRecord {
            day: day.clone(),
            data: ItemRecord {
                id: guid_str(),
                content: "".to_string(),
                status: false,
                day: day.clone(),
            },
        }
    }
}

impl DayItemRecord {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.set_width(225.0);
        ui.set_max_width(225.0);
        ui.columns(3, |ui| {
            ui[0].label("ADDDDDDDDDDDD");
            ui[1].label("BBBBBBBBBBBBBB");
            ui[2].label("A");
        });
        ui.horizontal(|ui| {
            if !self.data.status {
                if ui
                    .add(
                        egui::Button::image(egui::include_image!(
                            "../../image/checkbox_uncheck.png"
                        ))
                        .fill(egui::Color32::TRANSPARENT)
                        .stroke(Stroke::new(0.0, Color32::TRANSPARENT)),
                    )
                    .clicked()
                {
                    self.data.status = true;
                }
            } else {
                if ui
                    .add(
                        egui::Button::image(egui::include_image!(
                            "../../image/checkbox_checked.png"
                        ))
                        .fill(egui::Color32::TRANSPARENT)
                        .stroke(Stroke::new(0.0, Color32::TRANSPARENT)),
                    )
                    .clicked()
                {
                    self.data.status = false;
                }
            }

            let label = egui::Label::new("ksf").wrap_mode(egui::TextWrapMode::Truncate);
            ui.add(label);
            ui.label("...".to_string());
        });
    }
}
