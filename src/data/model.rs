use crate::util::guid_str;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct ItemRecord {
    pub day: String,
    pub id: String,
    pub content: String,
    pub status: bool,
}

impl ItemRecord {
    pub fn with_day(day: String) -> Self {
        ItemRecord {
            id: guid_str(),
            content: "".to_string(),
            status: false,
            day,
        }
    }
}
