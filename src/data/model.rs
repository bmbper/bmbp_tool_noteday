use std::collections::BTreeMap;

use crate::util::guid_str;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteItem {
    // 记录主键
    pub id: String,
    // 事项开始日期
    pub start_date: NaiveDate,
    // 事项结束日期
    pub end_date: NaiveDate,
    // 事项标题
    pub title: String,
    // 事项内容
    pub content: String,
    // 事项程度
    pub quadrant: Quadrant,
    pub desc: String,
    // 事项状态
    pub status: bool,
    // 记录日期
    pub record_day: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[serde(untagged)]
pub enum Quadrant {
    ImportantOnce,
    ImportantLazy,
    #[default]
    NormalOnce,
    NormalLazy,
}

impl NoteItem {
    pub fn with_day(day: String) -> Self {
        NoteItem {
            id: guid_str(),
            start_date: Local::now().date_naive(),
            end_date: Local::now().date_naive(),
            title: "".to_string(),
            desc: "".to_string(),
            content: "".to_string(),
            status: false,
            record_day: day,
            quadrant: Quadrant::ImportantOnce,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayNote {
    day: String,
    pub(crate) note: BTreeMap<String, NoteItem>,
}
#[allow(dead_code)]
impl DayNote {
    pub fn new(day: String) -> Self {
        DayNote {
            day,
            note: BTreeMap::new(),
        }
    }
    pub fn with_day(day: String) -> Self {
        DayNote {
            day,
            note: BTreeMap::new(),
        }
    }
    pub fn add_note(&mut self, note: NoteItem) {
        self.note.insert(note.id.clone(), note);
    }
    pub fn remove_note(&mut self, id: String) {
        self.note.remove(&id);
    }
    pub fn get_note(&self, id: String) -> Option<&NoteItem> {
        self.note.get(&id)
    }
}
