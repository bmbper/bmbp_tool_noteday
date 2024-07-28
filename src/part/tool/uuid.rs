use crate::part::NotePartView;
use egui::Ui;
use uuid::Uuid;
pub struct UuidView {
    id_count: u32,
    id_vec: Vec<String>,
}
impl UuidView {
    pub fn new() -> Self {
        UuidView {
            id_count: 0u32,
            id_vec: vec![],
        }
    }
    pub fn generate_id(&mut self) {
        self.id_vec = vec![];
        for _ in 0..self.id_count {
            let id = Uuid::new_v4().to_string().replace("-", "");
            self.id_vec.push(id);
        }
    }
}
impl NotePartView for UuidView {
    fn title(&mut self) -> String {
        "UUID".to_string()
    }
    fn show(&mut self, ui: &mut Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.set_height(24.0);
                ui.label("数量:");
                ui.add(egui::DragValue::new(&mut self.id_count).speed(1.0));
                if ui.button("生成").clicked() {
                    self.generate_id();
                }
            });
            ui.separator();
            egui::Frame::none().inner_margin(2.0).show(ui, |ui| {
                ui.set_height(height - 40.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_width(width - 4.0);
                    ui.set_row_height(24.0);
                    for id in self.id_vec.as_slice() {
                        let label = egui::Label::new(id.to_string())
                            .wrap_mode(egui::TextWrapMode::Truncate);
                        ui.add(label);
                    }
                });
            });
        });
    }
}
