use std::collections::HashMap;
use chrono::{Datelike};
use egui::{Align, Color32, Context};
use crate::data::{DayNote, NoteItem};
use crate::data::Quadrant::{ImportantLazy, ImportantOnce, NormalLazy, NormalOnce};
use crate::orm::Orm;
use crate::util::{to_date};

use super::ItemView;

pub struct DayView {
    day: String,
    data: Option<DayNote>,
    current_data: Option<NoteItem>,
    show_dialog: bool,
}

impl DayView {
    pub(crate) fn with_day(day: String) -> Self {
        let day_note = Orm::read(day.clone());
        DayView {
            day,
            data: day_note,
            current_data: None,
            show_dialog: false,
        }
    }
    pub fn reload_data(&mut self) {
        let day_note = Orm::read(self.day.clone());
        self.data = day_note;
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
            let rect_width = 225.0;
            let rect_height = 180.0;
            let rect_title_height = 30.0;
            let back_ground = if self.is_current_month(current_month) {
                Color32::WHITE
            } else {
                Color32::GRAY
            };
            let border_stroke = if self.is_current_month(current_month) {
                egui::Stroke::new(1.0, egui::Color32::DARK_GRAY)
            } else {
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            };

            egui::Frame::group(ui.style())
                .fill(back_ground)
                .stroke(border_stroke)
                .show(ui, |ui| {
                    ui.set_width(rect_width.clone());
                    ui.set_height(rect_height.clone());
                    ui.with_layout(egui::Layout::top_down_justified(Align::TOP), |ui| {
                        ui.horizontal(|ui| {
                            ui.set_height(rect_title_height);
                            ui.label(self.day.clone());
                            if self.is_current_month(current_month) {
                                if ui.button("+").clicked() {
                                    self.show_dialog = true;
                                }
                                if self.show_dialog {
                                    self.show_add_dialog(ctx, ui);
                                }
                            }
                        });
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .id_source(self.day.clone())
                            .show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::TOP),
                                    |ui| {
                                        if self.data.is_none() {
                                            return;
                                        }
                                        let dayNote = self.data.as_ref().unwrap().note.clone();
                                        for (id, note) in dayNote {
                                            ItemView::with_note(self, note).show(ui);
                                        }
                                    },
                                );
                            });
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

        let mut start_date = chrono::offset::Utc::now().date_naive();
        let mut end_date = chrono::offset::Utc::now().date_naive();

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
                            if let Some(day_note) = self.data.as_mut() {
                                day_note.note.insert(data.id.clone(), data);
                            } else {
                                let mut day_note = DayNote::with_day(self.day.clone());
                                day_note.note.insert(data.id.clone(), data);
                                self.data = Some(day_note);
                            }
                            Orm::write(self.day.clone(), self.data.as_ref().unwrap());
                        }
                        if ui.button("取消").clicked() {
                            self.show_dialog = false;
                        }
                    });
                });
            });
    }
}
