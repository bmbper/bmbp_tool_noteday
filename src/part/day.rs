use chrono::Datelike;
use eframe::epaint::Stroke;
use egui::{ Color32, Context};
use egui_extras::StripBuilder;

use crate::data::{DayNote, NoteItem};
use crate::data::Quadrant::{ImportantLazy, ImportantOnce, NormalLazy, NormalOnce};
use crate::orm::Orm;
use crate::util::{get_now, guid_str, to_date};

use super::ItemView;

pub struct DayView {
    day: String,
    data: Option<DayNote>,
    current_data: Option<NoteItem>,
    show_dialog: bool,
    current_item_title: String,
}

impl DayView {

    pub(crate) fn with_day(day: String) -> Self {
        let day_note = Orm::read(day.clone());
        DayView {
            day,
            data: day_note,
            current_data: None,
            show_dialog: false,
            current_item_title: "".to_string(),
        }
    }
    pub fn reload_data(&mut self) {
        let day_note = Orm::read(self.day.clone());
        self.data = day_note;
    }
    pub fn add_item(&mut self, title: String) {
        let data = NoteItem {
            id: guid_str(),
            title: title,
            content: "".to_string(),
            quadrant: NormalOnce,
            desc: "".to_string(),
            status: false,
            start_date: get_now(),
            end_date: get_now(),
            record_day: self.day.clone(),
        };
        self.write_note_item(data);
    }
    pub fn write_note_item(&mut self, item: NoteItem) {
        if let Some(day_note) = self.data.as_mut() {
            day_note.note.insert(item.id.clone(), item);
        } else {
            let mut day_note = DayNote::with_day(self.day.clone());
            day_note.note.insert(item.id.clone(), item);
            self.data = Some(day_note);
        }
        Orm::write(self.day.clone(), self.data.as_ref().unwrap());
        self.reload_data();
    }
}

impl DayView {
    pub fn is_current_month(&self, current_month: u32) -> bool {
        to_date(self.day.clone()).month() == current_month
    }
}

impl DayView {
    pub fn show(&mut self, ctx: &Context, ui: &mut egui::Ui, current_month: u32) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                StripBuilder::new(ui).size(egui_extras::Size::exact(40.0)).size(egui_extras::Size::remainder()).size(egui_extras::Size::exact(12.0)).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.label(self.day.clone());
                    });
                    strip.cell(|ui| {
                        if self.is_current_month(current_month) {
                            ui.text_edit_singleline(&mut self.current_item_title);
                        }
                        ui.input(|key| {
                            if key.key_pressed(egui::Key::Enter) {
                                if self.current_item_title.is_empty() {
                                    return;
                                }
                                let title = self.current_item_title.clone();
                                self.current_item_title = "".to_string();
                                self.add_item(title);
                            }
                        })
                    });
                    strip.cell(|ui| {
                        if self.is_current_month(current_month) {
                            if ui.add(egui::Button::new("+").fill(egui::Color32::TRANSPARENT)
                                .stroke(Stroke::new(0.0, Color32::TRANSPARENT))).clicked() {
                                self.show_dialog = true;
                            }
                            if self.show_dialog {
                                self.show_add_dialog(ctx, ui);
                            }
                        }
                    });
                });
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .id_source(self.day.clone())
                .show(ui, |ui| {
                    if self.data.is_none() {
                        return;
                    }
                    ui.vertical(|ui| {
                        let mut day_note:Vec<(String, NoteItem)> = self.data.as_ref().unwrap().note.clone().into_iter().collect();
                        day_note.sort_by(|a,b|a.1.title.cmp(&b.1.title));
                        for (id, note) in day_note {
                            ui.horizontal(|ui| {
                                ItemView::with_note(self, note).show(ui);
                            });
                        }
                    });
                });
        });
    }

    pub fn show_add_dialog(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        self.show_dialog = true;
        println!("===>{}", self.show_dialog);
        let screen_rect = ctx.available_rect();
        let window_size = egui::vec2(700.0, 800.0);
        let window_pos = egui::pos2(
            (screen_rect.width() - window_size.x) / 2.0,
            (screen_rect.height() - window_size.y) / 2.0,
        );
        egui::Window::new("增加日志")
            .resizable(false)
            .collapsible(false)
            .fixed_size(window_size)
            .default_pos(window_pos)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        egui::Grid::new(self.day.clone())
                            .num_columns(2)
                            .spacing([10.0, 10.0])
                            .striped(true)
                            .show(ui, |ui| {
                                if (self.current_data.is_none()) {
                                    self.current_data = Some(NoteItem::with_day(self.day.clone()));
                                }
                                ui.label("记录");
                                ui.text_edit_singleline(&mut self.current_data.as_mut().unwrap().title);
                                ui.end_row();
                                ui.label("明细");
                                ui.text_edit_multiline(&mut self.current_data.as_mut().unwrap().desc);
                                ui.end_row();
                                ui.label("开始时间");
                                ui.add(egui_extras::DatePickerButton::new(&mut self.current_data.as_mut().unwrap().start_date).id_source("start"));
                                ui.end_row();
                                ui.label("结束时间");
                                ui.add(egui_extras::DatePickerButton::new(&mut self.current_data.as_mut().unwrap().end_date).id_source("end"));
                                ui.end_row();
                                ui.label("象限");
                                ui.horizontal(|ui| {
                                    ui.radio_value(&mut self.current_data.as_mut().unwrap().quadrant, ImportantOnce, "重要紧急");
                                    ui.radio_value(&mut self.current_data.as_mut().unwrap().quadrant, ImportantLazy, "紧急不重要");
                                    ui.radio_value(&mut self.current_data.as_mut().unwrap().quadrant, NormalOnce, "重要不紧急");
                                    ui.radio_value(&mut self.current_data.as_mut().unwrap().quadrant, NormalLazy, "不重要不紧急");
                                });
                                ui.end_row();
                            });
                    });
                    ui.horizontal(|ui| {
                        if ui.button("确定").clicked() {
                            self.show_dialog = false;
                            let data = self.current_data.take().unwrap();
                            self.write_note_item(data);
                        }
                        if ui.button("取消").clicked() {
                            self.show_dialog = false;
                        }
                    });
                });
            });
    }
}
