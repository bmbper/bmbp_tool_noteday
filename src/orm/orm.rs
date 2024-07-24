use redb::{Database, TableDefinition};
use serde::Deserialize;
use std::sync::{OnceLock, RwLock};

const NOTE_TABLE: TableDefinition<String, String> = TableDefinition::new("bmbp_day_note");

fn orm() -> &'static RwLock<Database> {
    static ORM_LOCK: OnceLock<RwLock<Database>> = OnceLock::new();
    ORM_LOCK.get_or_init(|| {
        let db = Database::create("./note.eif").unwrap();
        let tx = db.begin_write().unwrap();
        tx.open_table(NOTE_TABLE).unwrap();
        tx.commit().unwrap();
        RwLock::new(db)
    })
}

pub struct Orm {}

impl Orm {
    pub fn read<'a, T>(day: String) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let data = Orm::read_data(day);
        if data.is_empty() {
            return None;
        }
        let data_rs: Result<T, serde_json::Error> = serde_json::from_str(&data);
        match data_rs {
            Ok(t) => {
                return Some(t);
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
    pub fn read_data(day: String) -> String {
        return match orm().read().unwrap().begin_read() {
            Ok(tx) => match tx.open_table(NOTE_TABLE) {
                Ok(table) => match table.get(day) {
                    Ok(record) => match record {
                        Some(v) => v.value().to_string(),
                        None => "".to_string(),
                    },
                    Err(e) => {
                        println!("{}", e);
                        "".to_string()
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    "".to_string()
                }
            },
            Err(e) => {
                println!("{}", e);
                "".to_string()
            }
        };
    }
    pub fn write<T: serde::Serialize>(day: String, content: T) {
        let data = serde_json::to_string(&content).unwrap();
        Orm::write_data(day, data);
    }
    pub fn write_data(day: String, content: String) {
        match orm().write().unwrap().begin_write() {
            Ok(tx) => {
                match tx.open_table(NOTE_TABLE) {
                    Ok(mut table) => {
                        match table.insert(day, content) {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{}", err);
                            }
                        };
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                tx.commit().unwrap();
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
