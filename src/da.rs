use egui::{Context, Label};
#[allow(dead_code)]
pub struct DayDialog {
    pub show_dialog: bool,
    pub text: String,
}
#[allow(dead_code)]
impl DayDialog {
    pub fn new() -> Self {
        DayDialog {
            show_dialog: false,
            text: "".to_string(),
        }
    }
    pub fn dialog_logic(&mut self, ctx: &Context) {
        egui::Window::new("确认对话框")
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("确认").clicked() {
                        // 这里执行确认操作后关闭对话框
                        self.show_dialog = false;
                    }
                    if ui.button("取消").clicked() {
                        // 直接关闭对话框
                        self.show_dialog = false;
                    }
                });
                ui.add(Label::new(format!("这是一个示例对话框{}", self.text)));
            });
    }
}
