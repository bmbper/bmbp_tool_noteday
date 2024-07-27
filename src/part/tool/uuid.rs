use crate::part::NotePartView;
use egui::Ui;
pub struct UuidView {}
impl UuidView {
    pub fn new() -> Self {
        UuidView {}
    }
}
impl NotePartView for UuidView {
    fn title(&mut self) -> String {
        "UUID".to_string()
    }
    fn show(&mut self, ui: &mut Ui) {}
}
