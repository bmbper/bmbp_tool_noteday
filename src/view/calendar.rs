use crate::part::DayView;
use crate::util::{get_calendar_week_days, get_current_y_m_d};
use eframe::emath::Align;
use eframe::epaint::{Color32, Stroke};
use egui::{vec2, Context, Ui};

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
        let width = ui.available_width();
        let height = ui.available_height();
        let bar_height = 24.0;
        let body_height = height - bar_height - 6.0;
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.set_height(bar_height);
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
            ui.separator();
            ui.horizontal(|ui| {
                let week_days = ["一", "二", "三", "四", "五", "六", "日"];
                let week_height = 24.0;
                let column_width = (width / 7.0) - 10.0;
                let column_height = ((body_height - week_height) / 5.0) - 11.0;
                egui::Grid::new("calc_id")
                    .spacing(vec2(2.0, 2.0))
                    .striped(true)
                    .show(ui, |ui| {
                        for row in 0..6 {
                            for col in 0..7 {
                                if row == 0 {
                                    egui::Frame::none()
                                        .stroke(Stroke::new(0.0, Color32::GRAY))
                                        .show(ui, |ui| {
                                            ui.set_width(column_width);
                                            ui.set_height(week_height);
                                            ui.with_layout(
                                                egui::Layout::top_down(Align::Center),
                                                |ui| {
                                                    ui.label(format!("星期{}", week_days[col]));
                                                },
                                            );
                                        });
                                } else {
                                    egui::Frame::none()
                                        .stroke(Stroke::new(1.0, Color32::GRAY))
                                        .inner_margin(4.0)
                                        .show(ui, |ui| {
                                            ui.set_width(column_width);
                                            ui.set_height(column_height);
                                            let item_view =
                                                &mut self.item_view[(row - 1) * 7 + col];
                                            item_view.show(
                                                ctx,
                                                ui,
                                                column_width,
                                                self.current_month.clone(),
                                            );
                                        });
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
