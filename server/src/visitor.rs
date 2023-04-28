use std::{path::PathBuf, time::Instant};

use parking_lot::RwLock;

pub struct Visitor {
    pub id: usize,
    pub description: String,
    pub last_seen: RwLock<Instant>,
    pub current_room: RwLock<PathBuf>,
}

impl Visitor {
    pub fn new(id: usize) -> Self {
        Visitor {
            id,
            description: "Maple Twig".to_string(),
            last_seen: RwLock::new(Instant::now()),
            current_room: RwLock::new(PathBuf::from("/")),
        }
    }
}
