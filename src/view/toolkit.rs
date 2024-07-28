use crate::part::Md5View;
use crate::part::NotePartView;
use crate::part::UuidView;
use egui::Context;
use egui::{vec2, Color32, Stroke};
pub struct ToolkitView {
    tool: Vec<Box<dyn NotePartView>>,
}

impl ToolkitView {
    pub fn new() -> Self {
        let mut tool = vec![];
        let md5_view: Box<dyn NotePartView> = Box::new(Md5View::new());
        tool.push(md5_view);
        let uuid_view: Box<dyn NotePartView> = Box::new(UuidView::new());
        tool.push(uuid_view);
        ToolkitView { tool }
    }
    pub fn show(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        let column_width = width / 7.0;
        let column_height = height / 5.0;
        egui::Grid::new("parent grid")
            .spacing(vec2(0.0, 0.0))
            .striped(true)
            .show(ui, |ui| {
                for index in 0..35 {
                    egui::Frame::none()
                        .stroke(Stroke::new(1.0, Color32::GRAY))
                        .show(ui, |ui| {
                            ui.set_width(column_width);
                            ui.set_height(column_height);
                            ui.vertical(|ui| {
                                egui_extras::StripBuilder::new(ui)
                                    .size(egui_extras::Size::exact(20.0))
                                    .size(egui_extras::Size::remainder())
                                    .vertical(|mut strip| {
                                        if self.tool.len() > index {
                                            let view = self.tool[index].as_mut();
                                            strip.cell(|ui| {
                                                ui.with_layout(
                                                    egui::Layout::top_down(egui::Align::Center),
                                                    |ui| {
                                                        ui.label(view.title());
                                                    },
                                                );
                                            });
                                            strip.cell(|ui| {
                                                view.show(ui);
                                            });
                                        } else {
                                            strip.cell(|ui| {});
                                            strip.cell(|ui| {});
                                        }
                                    });
                            });
                        });
                    if (index + 1) % 7 == 0 {
                        ui.end_row();
                    }
                }
            });
    }
}
