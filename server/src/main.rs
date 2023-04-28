mod room;
mod visitor;
mod world;

use std::{path::PathBuf, time::Instant};

use rocket::{get, launch, routes, serde::json::Json, State};
use world::World;

use crate::room::RoomSnapshot;

#[get("/room/<visitor_id>/<path..>")]
fn visit_room(visitor_id: usize, path: PathBuf, world: &State<World>) -> Json<RoomSnapshot> {
    let visitor = world.get_visitor(visitor_id);
    *visitor.last_seen.write() = Instant::now();
    let room = world.move_visitor(&visitor, &path);
    Json(room.snapshot(visitor.id))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(World::new())
        .mount("/", routes![visit_room])
}
