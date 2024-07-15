use crate::util::{get_calendar_week_days, get_current_y_m_d, to_date};
use eframe::emath::{Align};
use eframe::epaint::{Color32, Stroke};
use egui::{Context, Ui};
use egui::Key::S;
use egui::UiKind::ScrollArea;
use egui_extras::{StripBuilder};
use crate::part::DayView;

pub struct CalendarView {
    current_year: i32,
    current_month: u32,
    current_days: Vec<String>,
    item_view: Vec<DayView>,
}

impl CalendarView {
    pub fn new() -> Self {
        let (current_year, current_month, _) = get_current_y_m_d();
        let mut cal = CalendarView {
            current_days: vec![],
            current_year,
            current_month,
            item_view: vec![],
        };
        cal.init_data();
        cal
    }
    pub fn init_data(&mut self) {
        self.item_view = vec![];
        let cal_days =
            get_calendar_week_days(self.current_year.clone(), self.current_month.clone());
        self.current_days = cal_days;
        for day in &self.current_days {
            self.item_view.push(DayView::with_day(day.clone()));
        }
    }
    pub fn on_next_month(&mut self) {
        if self.current_month == 12 {
            self.current_month = 1;
            self.current_year = self.current_year + 1;
        } else {
            self.current_month = self.current_month + 1;
        }
        self.init_data();
    }
    pub fn on_pre_month(&mut self) {
        if self.current_month == 1 {
            self.current_month = 12;
            self.current_year = self.current_year - 1;
        } else {
            self.current_month = self.current_month - 1;
        }
        self.init_data();
    }
}

impl CalendarView {
    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        let cell_width = 175.0;
        let cell_height = 160.0;
        StripBuilder::new(ui).size(egui_extras::Size::exact(24.0)).size(egui_extras::Size::exact(2.0)).size(egui_extras::Size::remainder()).vertical(|mut strip| {
            strip.cell(|ui| {
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    if ui.button("上").clicked() {
                        self.on_pre_month();
                    }
                    if ui.button("下").clicked() {
                        self.on_next_month();
                    }
                    ui.separator();
                    let _ = ui.button("月");
                    let _ = ui.button("周");
                    let _ = ui.button("年");
                });
            });
            strip.cell(|ui| {
                ui.separator();
            });
            strip.cell(|ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    StripBuilder::new(ui).size(egui_extras::Size::exact(24.0)).sizes(egui_extras::Size::remainder(), 5).vertical(|mut strip| {
                        let week_days = ["一", "二", "三", "四", "五", "六", "日"];
                        // 工作项
                        for row_index in 0..6 {
                            strip.strip(|sub_strip| {
                                sub_strip.sizes(egui_extras::Size::remainder(), 7).horizontal(|mut strip| {
                                    for col_index in 0..7 {
                                        if row_index == 0 {
                                            strip.cell(|ui| {
                                                ui.set_width(cell_width);
                                                ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                                                    ui.label(format!("星期{}", week_days[col_index]));
                                                });
                                            });
                                        } else {
                                            strip.cell(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_width(cell_width);
                                                    let item_view = &mut self.item_view[(row_index - 1) * 7 + col_index];
                                                    let mut fill_color = Color32::WHITE;
                                                    if !item_view.is_current_month(self.current_month.clone()) {
                                                        fill_color = Color32::LIGHT_GRAY;
                                                    }
                                                    egui::Frame::default().inner_margin(4.0).fill(fill_color).stroke(Stroke::new(1.0, Color32::LIGHT_GRAY)).show(ui, |ui| {
                                                        ui.set_min_height(cell_height);
                                                        item_view.show(ctx, ui, self.current_month.clone());
                                                    });
                                                });
                                            });
                                        }
                                    }
                                });
                            });
                        }
                    });
                });
            });
        });
    }
}
