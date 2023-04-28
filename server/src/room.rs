use std::collections::HashMap;

use parking_lot::RwLock;
use rocket::serde::Serialize;

pub struct Room {
    pub visitors: RwLock<HashMap<usize, String>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RoomSnapshot {
    pub visitors: Vec<String>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            visitors: RwLock::new(HashMap::new()),
        }
    }

    pub fn snapshot(&self, visitor_id: usize) -> RoomSnapshot {
        RoomSnapshot {
            visitors: self
                .visitors
                .read()
                .iter()
                .filter_map(|(key, value)| {
                    if key != &visitor_id {
                        Some(value.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}
