use egui::{vec2, Color32, Stroke};

#[derive(Debug)]
pub struct InboxView {}

impl InboxView {
    pub fn new() -> Self {
        InboxView {}
    }
    pub fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        println!("{}-{}", width, height);
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
                            ui.vertical(|ui|{
                                ui.horizontal(|ui|{
                                    ui.set_height(24.0);
                                    ui.set_width(column_width);
                                    egui_extras::StripBuilder::new(ui).size(egui_extras::Size::exact(20.0)).size(egui_extras::Size::remainder()).size(egui_extras::Size::exact(20.0)).horizontal(|mut strip|{
                                        strip.cell(|ui|{
                                            let mut label =egui::Label::new("AFFFFAAAjjiosdjfiojiojsfojsdojsofjsofjsofjsofjioasjfsojfosjfsiofjsiofjsiojsojsod");
                                            label = label.wrap_mode(egui::TextWrapMode::Truncate);
                                            ui.add(label);
                                        });
                                        strip.cell(|ui|{
                                            let mut label =egui::Label::new("AFFFFAAAjjiosdjfiojiojsfojsdojsofjsofjsofjsofjioasjfsojfosjfsiofjsiofjsiojsojsod");
                                            label = label.wrap_mode(egui::TextWrapMode::Truncate);
                                            ui.add(label);
                                        });
                                        strip.cell(|ui|{
                                            let mut label =egui::Label::new("AFFFFAAAjjiosdjfiojiojsfojsdojsofjsofjsofjsofjioasjfsojfosjfsiofjsiofjsiojsojsod");
                                            label = label.wrap_mode(egui::TextWrapMode::Truncate);
                                            ui.add(label);
                                        });
                                    });

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
