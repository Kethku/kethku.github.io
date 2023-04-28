use std::{collections::HashMap, path::PathBuf, sync::Arc};

use parking_lot::RwLock;

use crate::{room::Room, visitor::Visitor};

pub struct World {
    rooms: RwLock<HashMap<PathBuf, Arc<Room>>>,
    visitors: RwLock<HashMap<usize, Arc<Visitor>>>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: RwLock::new(HashMap::new()),
            visitors: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_visitor(&self, mut visitor_id: usize) -> Arc<Visitor> {
        let visitors = self.visitors.write();

        if visitor_id == 0 {
            visitor_id = visitors.len();
        }

        self.visitors
            .write()
            .entry(visitor_id)
            .or_insert_with(|| Arc::new(Visitor::new(visitor_id)))
            .clone()
    }

    pub fn move_visitor(&self, visitor: &Visitor, destination: &PathBuf) -> Arc<Room> {
        let mut rooms = self.rooms.write();

        // Remove the visitor from the previous room
        let previous_room_path = visitor.current_room.read().clone();
        if let Some(previous_room) = rooms.get_mut(&previous_room_path) {
            previous_room.visitors.write().remove(&visitor.id);
        }

        // Get or create the room at path
        let room = rooms
            .entry(destination.clone())
            .or_insert_with(|| Arc::new(Room::new()));

        // Add the visitor to the room at path
        let current_room = visitor.current_room.write();
        *current_room = destination.clone();
        room.visitors
            .write()
            .insert(visitor.id, visitor.description.clone());

        room.clone()
    }
}
