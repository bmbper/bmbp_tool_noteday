use eframe::epaint::text::TextWrapMode;
use crate::{data::NoteItem};
use egui::{Color32, Stroke, Ui};
use crate::data::DayNote;
use crate::orm::Orm;
use crate::part::DayView;

pub struct ItemView<'a> {
    pub day: String,
    pub data: NoteItem,
    parent: &'a mut DayView,
}


impl<'a> ItemView<'a> {
    pub fn with_note(parent: &'a mut DayView, note: NoteItem) -> Self {
        ItemView {
            day: note.record_day.clone(),
            data: note,
            parent,
        }
    }
    pub fn save_data(&mut self, status: bool) {
        let day = self.day.clone();
        let mut data = self.data.clone();
        data.status = status;
        let mut day_note = Orm::read::<DayNote>(day).unwrap();
        day_note.note.insert(data.id.clone(), data);
        Orm::write::<DayNote>(self.day.clone(), day_note);
        self.parent.reload_data();
    }
    pub fn remove_data(&mut self) {
        let day = self.day.clone();
        let data = self.data.clone();
        let mut day_note = Orm::read::<DayNote>(day).unwrap();
        day_note.note.remove(&data.id);
        Orm::write::<DayNote>(self.day.clone(), day_note);
        self.parent.reload_data();
    }
}

impl<'a> ItemView<'a> {
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
                    self.save_data(true);
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
                    self.save_data(false);
                }
            }
            let label = egui::Label::new(self.data.title.clone()).wrap_mode(TextWrapMode::Truncate);
            ui.add(label);
            if ui.add(egui::Button::image(egui::include_image!("../../image/delete.png")).fill(egui::Color32::TRANSPARENT).stroke(Stroke::new(0.0, Color32::TRANSPARENT))).clicked() {
                self.remove_data();
            }
        });
    }
}
