use chrono::Datelike;
use egui::{Align, Color32, Context};

use crate::util::to_date;

use super::DayItemRecord;

pub struct DayItem {
    day: String,
    data: Vec<DayItemRecord>,
    show_dialog: bool,
}

impl DayItem {
    pub(crate) fn with_day(day: String) -> Self {
        DayItem {
            day,
            data: vec![],
            show_dialog: false,
        }
    }
}

impl DayItem {
    pub fn is_current_month(&self, current_month: u32) -> bool {
        to_date(self.day.clone()).month() == current_month
    }
}

impl DayItem {
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
                                        for item in self.data.as_mut_slice() {
                                            item.show(ui);
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
        let data = DayItemRecord::with_day(self.day.clone());
        egui::Window::new("确认对话框")
            .id(egui::Id::new(self.day.clone()))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("确认").clicked() {
                        // 这里执行确认操作后关闭对话框
                        self.show_dialog = false;
                    }
                    if ui.button("取消").clicked() {
                        // 直接关闭对话框
                        self.show_dialog = false;
                    }
                });
                ui.add(egui::Label::new(format!("这是一个示例对话框{}", self.day)));
            });
    }
}
