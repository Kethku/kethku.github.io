use std::{collections::HashMap, path::PathBuf, time::Instant};

use parking_lot::RwLock;
use rocket::{
    get, launch, routes,
    serde::{json::Json, Serialize},
    State,
};

struct Room {
    visitors: RwLock<HashMap<usize, String>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RoomSnapshot {
    visitors: Vec<String>,
}

struct Visitor {
    description: String,
    last_seen: Instant,
    current_room: RwLock<PathBuf>,
}

struct World {
    rooms: RwLock<HashMap<PathBuf, Room>>,
    visitors: RwLock<HashMap<usize, Visitor>>,
}

#[get("/room/<visitor_id>/<path..>")]
fn room(mut visitor_id: usize, path: PathBuf, world: &State<World>) -> Json<RoomSnapshot> {
    let mut rooms = world.rooms.write();
    let mut visitors = world.visitors.write();

    // If the visitor_id is 0, create a new visitor id
    if visitor_id == 0 {
        visitor_id = visitors.len() + 1;
    }

    // Get or create the visitor
    let visitor = visitors.entry(visitor_id).or_insert_with(|| Visitor {
        description: "Maple Twig".to_string(),
        last_seen: Instant::now(),
        current_room: RwLock::new(PathBuf::from("/")),
    });

    // Remove the visitor from the previous room
    let previous_room_path = visitor.current_room.read().clone();
    if let Some(previous_room) = rooms.get_mut(&previous_room_path) {
        previous_room.visitors.write().remove(&visitor_id);
    }

    // Get or create the room at path
    let room = rooms.entry(path.clone()).or_insert_with(|| Room {
        visitors: RwLock::new(HashMap::new()),
    });

    // Add the visitor to the current room
    visitor.current_room = RwLock::new(path.clone());
    room.visitors
        .write()
        .insert(visitor_id, visitor.description.clone());

    // Update the visitor's last seen time
    visitor.last_seen = Instant::now();

    // Return the room snapshot
    let snapshot = RoomSnapshot {
        visitors: room
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
    };
    Json(snapshot)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(World {
            rooms: RwLock::new(HashMap::new()),
            visitors: RwLock::new(HashMap::new()),
        })
        .mount("/", routes![room])
}
