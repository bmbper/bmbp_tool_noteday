use crate::view::{CalendarView, InboxView, QuadrantView, ToolkitView};
use eframe::App;
use egui::panel::Side;
use egui::{CentralPanel, Context, SidePanel};

#[derive(PartialEq)]
pub enum ViewEnum {
    CALENDAR,
    INBOX,
    QUADRANT,
    TOOLKIT,
}

pub struct NoteDayApp {
    view_state: ViewEnum,
    calendar_view: CalendarView,
    inbox_view: InboxView,
    quadrant_view: QuadrantView,
    toolkit_view: ToolkitView,
}

impl NoteDayApp {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        Self::set_app_fonts(ctx);
        NoteDayApp {
            view_state: ViewEnum::CALENDAR,
            calendar_view: CalendarView::new(),
            inbox_view: InboxView::new(),
            quadrant_view: QuadrantView::new(),
            toolkit_view: ToolkitView::new(),
        }
    }
    fn set_app_fonts(ctx: &eframe::CreationContext<'_>) {
        // 加载自定义字体数据
        let font_data =
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
        SidePanel::new(Side::Left, "navLeft")
            .default_width(40.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.selectable_value(&mut self.view_state, ViewEnum::CALENDAR, "日历");
                    ui.selectable_value(&mut self.view_state, ViewEnum::INBOX, "信箱");
                    ui.selectable_value(&mut self.view_state, ViewEnum::QUADRANT, "象限");
                    ui.selectable_value(&mut self.view_state, ViewEnum::TOOLKIT, "工具");
                });
            });
        CentralPanel::default().show(ctx, |ui| match self.view_state {
            ViewEnum::CALENDAR => {
                self.calendar_view.show(ctx, ui);
            }
            ViewEnum::INBOX => {
                self.inbox_view.show(ctx, ui);
            }
            ViewEnum::QUADRANT => {
                self.quadrant_view.show(ctx, ui);
            }
            ViewEnum::TOOLKIT => {
                self.toolkit_view.show(ctx, ui);
            }
        });
    }
}
