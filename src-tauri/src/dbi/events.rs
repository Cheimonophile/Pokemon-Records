use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::schema;

use crate::dbi::structs::playthrough::InsertPlaythrough;

use super::structs::playthrough::Playthrough;

pub fn create_playthrough(
    conn: &mut SqliteConnection,
    id_no: &str,
    name: &str,
    version: &str,
    adventure_started: &str,
) -> Playthrough {
    let new_playthrough = InsertPlaythrough {
        id_no,
        name,
        version,
        adventure_started,
    };
    diesel::insert_into(schema::Playthrough::table)
        .values(&new_playthrough)
        .execute(conn)
        .expect("Error saving new playthrough");
    let playthrough = schema::Playthrough::table
        .filter(schema::Playthrough::id_no.eq(id_no))
        .first::<Playthrough>(conn)
        .expect("Error loading playthrough");
    println!("Ceated playthrough {}", playthrough);
    playthrough
}

pub fn create_location(
    conn: &mut SqliteConnection,
    name: &str,
    region: &str,
) -> crate::dbi::structs::location::Location {
    let new_location = crate::dbi::structs::location::InsertLocation { name, region };
    diesel::insert_into(schema::Location::table)
        .values(&new_location)
        .execute(conn)
        .expect("Error saving new location");
    let location = schema::Location::table
        .filter(schema::Location::name.eq(name))
        .first::<crate::dbi::structs::location::Location>(conn)
        .expect("Error loading location");
    println!("Ceated location {}", location);
    location
}
