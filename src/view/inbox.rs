#[derive(Debug)]
pub struct InboxView {}

impl InboxView {
    pub fn new() -> Self {
        InboxView {}
    }
    pub fn show(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui) {}
}
