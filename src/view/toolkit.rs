use egui::Context;

#[derive(Debug)]
pub struct ToolkitView {}

impl ToolkitView {
    pub fn new() -> Self {
        ToolkitView {}
    }
    pub fn show(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.label("工具箱");
    }
}
