use egui::Ui;
pub trait NotePartView {
    fn show(&mut self, ui: &mut Ui);
    fn title(&mut self) -> String;
}
