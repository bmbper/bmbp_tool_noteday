use egui::Context;

#[derive(Debug)]
pub struct QuadrantView {}

impl QuadrantView {
    pub fn new() -> Self {
        QuadrantView {}
    }
    pub fn show(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.label("轻重缓急邮箱");
    }
}
