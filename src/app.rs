use crate::da::DayDialog;
use eframe::App;
use egui::{CentralPanel, Context, SidePanel, Vec2};
use egui::panel::Side;
use crate::view::CalendarView;

pub struct NoteDayApp {
    dialog: DayDialog,
    calendar_view: CalendarView,
}

impl NoteDayApp {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        Self::set_app_fonts(ctx);
        NoteDayApp {
            dialog: DayDialog::new(),
            calendar_view: CalendarView::new(),
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
}

impl App for NoteDayApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        SidePanel::new(Side::Left, "navLeft").default_width(40.0).show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                let _ = ui.button("日历");
                let _ = ui.button("象限");
                let _ = ui.button("信箱");
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            self.calendar_view.show(ctx, ui);
        });
    }
}
