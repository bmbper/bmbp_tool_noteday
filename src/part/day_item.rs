use egui::{Align, vec2, Vec2b};

use crate::data::ItemRecord;

pub struct DayItem {
    day: String,
    data: Vec<ItemRecord>,
    t1: String,
}

impl DayItem {
    pub(crate) fn with_day(day: String) -> Self {
        DayItem {
            day,
            data: vec![],
            t1: "".to_string(),
        }
    }
}

impl DayItem {}

impl DayItem {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let rect_width = 225.0;
            let rect_height = 180.0;
            let rect_title_height = 30.0;
            egui::Frame::group(ui.style()).fill(ui.visuals().widgets.noninteractive.bg_fill).stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY)).show(ui, |ui| {
                ui.set_width(rect_width.clone());
                ui.set_height(rect_height.clone());
                ui.with_layout(egui::Layout::top_down_justified(Align::TOP), |ui| {
                    ui.horizontal(|ui| {
                        ui.set_height(rect_title_height);
                        ui.label(self.day.clone());
                        if ui.button("+").clicked() {
                            self.data.push(ItemRecord::with_day(self.day.clone()));
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().id_source(self.day.clone()).show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down_justified(Align::TOP), |ui| {
                            for item in self.data.as_mut_slice(){
                                item.show(ui);
                            }
                        });
                    });
                });
            });
        });
    }
}