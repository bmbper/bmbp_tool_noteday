use std::ops::{Add, Sub};
use std::str::FromStr;

use chrono::{Datelike, Duration, Local, NaiveDate};

pub fn get_current_y_m_d() -> (i32, u32, u32) {
    let now = Local::now();
    (now.year(), now.month(), now.day())
}

#[allow(dead_code)]
pub fn get_now() -> NaiveDate {
    let now = Local::now();
    now.date_naive()
}

#[allow(dead_code)]
pub fn get_date_time() -> String {
    let now = Local::now();
    let date_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    date_time
}

#[allow(dead_code)]
pub fn get_date() -> String {
    let now = Local::now();
    let date_time = now.format("%Y-%m-%d").to_string();
    date_time
}

pub fn to_date(day: String) -> NaiveDate {
    if let Ok(date) = NaiveDate::from_str(day.as_str()) {
        return date;
    }
    return Local::now().date_naive();
}

#[allow(dead_code)]
pub fn get_current_calendar_days() -> Vec<String> {
    let now = chrono::Local::now();
    get_calendar_week_days(now.year(), now.month())
}

pub fn get_calendar_days(year: i32, month: u32) -> Vec<String> {
    let current_year = year;
    let current_month = month;
    let mut first_day_of_month =
        chrono::NaiveDate::from_ymd_opt(current_year, current_month, 1).unwrap();
    let first_day_of_next_month = if current_month == 12 {
        chrono::NaiveDate::from_ymd_opt(current_year + 1, 1, 1).unwrap()
    } else {
        first_day_of_month
            .clone()
            .with_month(current_month + 1)
            .unwrap()
    };
    // 生成从第一天到下个月第一天前一天的日期列表
    let mut calendar_days = vec![];
    while first_day_of_month < first_day_of_next_month {
        calendar_days.push(first_day_of_month.format("%Y-%m-%d").to_string());
        first_day_of_month = first_day_of_month.add(Duration::days(1));
    }
    calendar_days
}

#[allow(dead_code)]
pub fn get_current_calendar_week_days() -> Vec<String> {
    let now = chrono::Local::now();
    get_calendar_week_days(now.year(), now.month())
}

pub fn get_calendar_week_days(year: i32, month: u32) -> Vec<String> {
    let mut calendar_days = get_calendar_days(year, month);

    let week_day_total = 35u32;
    let first_day = to_date(calendar_days[0].clone());
    let end_day = to_date(calendar_days[calendar_days.len() - 1].clone());
    let first_day_week_day = first_day.clone().weekday().num_days_from_monday();
    // 以上个月的日期补全本周的天数
    for i in 0..first_day_week_day {
        if calendar_days.len() as u32 == week_day_total {
            break;
        }
        let pre_day = first_day.clone().sub(Duration::days((i + 1) as i64));
        calendar_days.insert(0, pre_day.format("%Y-%m-%d").to_string());
    }
    let cal_day_count = calendar_days.len() as u32;
    for i in 0..(week_day_total - cal_day_count) {
        let next_day = end_day.clone().add(Duration::days((i + 1) as i64));
        calendar_days.push(next_day.format("%Y-%m-%d").to_string());
    }
    calendar_days
}

pub fn get_current_week_days() -> Vec<String> {
    let tody = Local::now().date_naive();
    let first_day_week_day = tody.clone().weekday().num_days_from_monday();
    // 补全前几天
    let mut week_day = vec![tody.clone().format("%Y-%m-%d").to_string()];
    for index in 0..first_day_week_day {
        let pre_day = tody.clone().sub(Duration::days((index + 1) as i64));
        week_day.insert(0, pre_day.format("%Y-%m-%d").to_string());
    }
    for index in 0..(7 - first_day_week_day - 1) {
        let next_day = tody.clone().add(Duration::days((index + 1) as i64));
        week_day.push(next_day.format("%Y-%m-%d").to_string());
    }
    week_day
}
