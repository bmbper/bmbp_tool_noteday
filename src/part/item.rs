use crate::data::DayNote;
use crate::data::NoteItem;
use crate::orm::Orm;
use crate::part::DayView;
use crate::util::get_now;
use eframe::epaint::text::TextWrapMode;
use egui::{Color32, Stroke, Ui};
use egui_extras::StripBuilder;

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
        ui.vertical(|ui| {
            ui.set_height(24.0);
            StripBuilder::new(ui)
                .size(egui_extras::Size::exact(24.0))
                .size(egui_extras::Size::remainder())
                .size(egui_extras::Size::exact(24.0))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
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
                                self.data.end_date = get_now();
                                self.save_data(false);
                            }
                        }
                    });
                    strip.cell(|ui| {
                        if self.data.is_edit {
                            let input_title = ui.text_edit_singleline(&mut self.data.title);
                            if input_title.changed() {
                                self.save_data(false);
                            }
                            if input_title.lost_focus() {
                                self.data.is_edit = false;
                                self.save_data(false);
                            }
                            ui.input(|key| {
                                if key.key_pressed(egui::Key::Enter) {
                                    self.data.is_edit = false;
                                    self.save_data(false);
                                }
                            });
                        } else {
                            let label = egui::Label::new(self.data.title.clone())
                                .wrap_mode(TextWrapMode::Truncate);
                            if ui.add(label).clicked() {
                                self.data.is_edit = true;
                                self.save_data(self.data.status);
                            }
                        }
                    });
                    strip.cell(|ui| {
                        if ui
                            .add(
                                egui::Button::image(egui::include_image!("../../image/delete.png"))
                                    .fill(egui::Color32::TRANSPARENT)
                                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT)),
                            )
                            .clicked()
                        {
                            self.remove_data();
                        }
                    });
                });
        });
    }
}
