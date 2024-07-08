use eframe::epaint::text::TextWrapMode;
use crate::{data::NoteItem};
use egui::{Color32, Stroke, Ui};
use serde::Serialize;
use crate::data::DayNote;
use crate::orm::Orm;

#[derive(Debug, Serialize)]
pub struct DayItemRecord<'a> {
    pub day: String,
    pub data: &'a mut NoteItem,
}


impl<'a> DayItemRecord<'a> {
    pub fn with_note(note: &'a mut NoteItem) -> Self {
        DayItemRecord {
            day: note.record_day.clone(),
            data: note,
        }
    }
    pub fn save_data(&self) {
        let day = self.day.clone();
        let data = self.data.clone();
        let mut day_note = Orm::read::<DayNote>(day).unwrap();
        day_note.note.insert(data.id.clone(), data);
        Orm::write::<DayNote>(self.day.clone(), day_note);
    }
}

impl<'a> DayItemRecord<'a> {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.set_width(225.0);
        ui.set_max_width(225.0);
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
                    self.save_data();
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
            let label = egui::Label::new(self.data.title.clone()).wrap_mode(TextWrapMode::Truncate);
            ui.add(label);
        });
    }
}
