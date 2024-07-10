use crate::util::{get_calendar_week_days, get_current_y_m_d};
use eframe::emath::{Align, Vec2};
use egui::panel::TopBottomSide;
use egui::{Context, Ui};
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
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |_| {
            egui::TopBottomPanel::new(TopBottomSide::Top, "tbar_top").show(ctx, |ui| {
                ui.set_height(32.0);
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
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("0001")
                        .num_columns(7)
                        .spacing(Vec2::new(2.0, 2.0))
                        .show(ui, |ui| {
                            ui.label("星期一");
                            ui.label("星期二");
                            ui.label("星期三");
                            ui.label("星期四");
                            ui.label("星期五");
                            ui.label("星期六");
                            ui.label("星期七");
                            ui.end_row();
                            let length = self.current_days.len();
                            for index in 0..length {
                                self.item_view[index].show(ctx, ui, self.current_month.clone());
                                if index != 0 && (index + 1) % 7 == 0 {
                                    ui.end_row();
                                }
                            }
                            ui.end_row();
                        });
                });
            });
        });
    }
}
