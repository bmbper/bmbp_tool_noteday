use uuid::Uuid;

pub fn guid_str() -> String {
    Uuid::new_v4().to_string()
}
