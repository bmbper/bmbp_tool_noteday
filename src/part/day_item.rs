use chrono::Datelike;
use egui::{Align, Color32};

use crate::{data::ItemRecord, util::to_date};

pub struct DayItem {
    day: String,
    data: Vec<ItemRecord>,
}

impl DayItem {
    pub(crate) fn with_day(day: String) -> Self {
        DayItem { day, data: vec![] }
    }
}

impl DayItem {
    pub fn is_current_month(&self, current_month: u32) -> bool {
        to_date(self.day.clone()).month() == current_month
    }
}

impl DayItem {
    pub fn show(&mut self, ui: &mut egui::Ui, current_month: u32) {
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
                                    self.data.push(ItemRecord::with_day(self.day.clone()));
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
                                        let egui_icon = egui::include_image!(
                                            "../../image/checkbox_checked.png"
                                        );
                                        ui.add(egui::Image::new(egui_icon.clone()));
                                    },
                                );
                            });
                    });
                });
        });
    }
}
