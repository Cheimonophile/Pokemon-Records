use crate::dbi::connection::connect;
use crate::dbi::events::*;

pub fn run() {
    let conn = &mut connect();
    let mut playthrough = create_playthrough(conn, "26852", "Ben", "Black", "2023-06-24");
    let nuvema_town = create_location(conn, "Nuvema Town", "Unova");
}