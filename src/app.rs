use crate::da::DayDialog;
use eframe::App;
use egui::{Button, CentralPanel, Color32, Context, Vec2};
pub struct NoteDayApp {
    dialog: DayDialog,
    is_dialog_open: bool,
}

impl NoteDayApp {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        Self::set_app_fonts(ctx);
        NoteDayApp {
            dialog: DayDialog::new(),
            is_dialog_open: false,
        }
    }
    fn set_app_fonts(ctx: &eframe::CreationContext<'_>) {
        // 加载自定义字体数据
        let mut font_data =
            egui::FontData::from_static(include_bytes!("../fonts/SourceHanSerifCN-Regular.otf")); // 替换为你的字体文件路径
                                                                                                  // 定义字体家族，关联字体数据
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("app_font".to_owned(), font_data);
        // 将自定义字体设为默认的中文字体
        fonts
            .families
            .insert(egui::FontFamily::Monospace, vec!["app_font".to_owned()]);
        fonts
            .families
            .insert(egui::FontFamily::Proportional, vec!["app_font".to_owned()]);
        // 应用字体定义
        ctx.egui_ctx.set_fonts(fonts);
    }
    fn show_popwindow(&mut self, ctx: &Context) {
        println!("ddddd");
        self.dialog.dialog_logic(ctx);
    }
}

impl App for NoteDayApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut app_layout = egui::Layout::top_down(egui::Align::TOP);
        let mut header_layout = egui::Layout::left_to_right(egui::Align::LEFT);
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(app_layout, |ui| {
                ui.with_layout(header_layout, |ui| {
                    ui.set_height(24.0);
                    let _ = ui.button("日历");
                    let _ = ui.button("象限");
                    let _ = ui.button("信箱");
                });
                ui.separator();
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("0001")
                        .num_columns(5)
                        .spacing(Vec2::new(2.0, 2.0))
                        .show(ui, |ui| {
                            for row in 0..5 {
                                for item in 0..5 {
                                    egui::Frame::group(ui.style())
                                        .fill(ui.visuals().widgets.noninteractive.bg_fill) // 设置背景颜色
                                        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK)) // 设置边框颜色和宽度
                                        .show(ui, |ui| {
                                            // 在Frame内部添加Label
                                            ui.add(egui::Label::new(egui::RichText::new(format!(
                                                "这是一个带有边框的{}",
                                                item
                                            ))));

                                            if (ui.button("text").clicked()) {
                                                if (self.dialog.show_dialog) {
                                                    self.dialog.show_dialog = false;
                                                }
                                                self.dialog.text = format!("{}-{}", row, item);
                                                self.dialog.show_dialog = true;
                                            }
                                            if (self.dialog.show_dialog) {
                                                self.show_popwindow(ctx);
                                            }
                                        });
                                }
                                ui.end_row();
                            }
                        });
                });
            });
        });
    }
}
