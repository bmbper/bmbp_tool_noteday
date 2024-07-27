use crate::part::NotePartView;
use egui::Ui;
pub struct Md5View {}
impl Md5View {
    pub fn new() -> Self {
        Md5View {}
    }
}
impl NotePartView for Md5View {
    fn title(&mut self) -> String {
        "MD5".to_string()
    }
    fn show(&mut self, ui: &mut Ui) {}
}
