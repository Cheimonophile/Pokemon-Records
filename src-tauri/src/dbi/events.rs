use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::schema;

use crate::dbi::structs::*;

use super::structs::location::Location;
use super::structs::playthrough::Playthrough;

pub fn create_playthrough(
    conn: &mut SqliteConnection,
    id_no: &str,
    name: &str,
    version: &str,
    adventure_started: &str,
) -> Playthrough {
    let new_playthrough = playthrough::InsertPlaythrough {
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

pub fn create_location(conn: &mut SqliteConnection, name: &str, region: &str) -> Location {
    let new_location = location::InsertLocation { name, region };
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

pub fn create_species(
    conn: &mut SqliteConnection,
    dex_no: &i32,
    name: &str,
    form: Option<&str>,
    generation: &i32,
    type1: &str,
    type2: Option<&str>,
) -> species::Species {
    let new_species = species::InsertSpecies {
        dex_no,
        form,
        name,
        generation,
        type1,
        type2,
    };
    diesel::insert_into(schema::Species::table)
        .values(&new_species)
        .execute(conn)
        .expect("Error saving new species");
    let species = schema::Species::table
        .filter(schema::Species::name.eq(name))
        .first::<crate::dbi::structs::species::Species>(conn)
        .expect("Error loading species");
    println!("Ceated species {}", species);
    species
}


pub fn catch_pokemon(
    conn: &mut SqliteConnection,
    playthrough: &playthrough::Playthrough,
    slot: &i32,
    species: &species::Species,
    nickname: Option<&str>,
    caught_date: &str,
    caught_location: &location::Location,
    caught_level: &i32,
    gender: &str,
    ball: &str
) {
    let new_event = event::InsertEvent {
        playthrough_id_no: &playthrough.id_no,
        location_name: &caught_location.name,
        location_region: &caught_location.region,
    };
    diesel::insert_into(schema::Event::table)
        .values(&new_event)
        .execute(conn)
        .expect("Error saving new event");
    let event = schema::Event::table
        .filter(schema::Event::playthrough_id_no.eq(&playthrough.id_no))
        .first::<event::Event>(conn)
        .expect("Error loading event");
    let new_catch_event = catch_event::InsertCatchEvent {
        no: &event.no,
        catch_type: &"Gift",
    };
    diesel::insert_into(schema::Catch_Event::table)
        .values(&new_catch_event)
        .execute(conn)
        .expect("Error saving new catch event");
    let new_team_member =  team_member::InsertTeamMember {
        playthrough_id_no: &playthrough.id_no,
        slot,
        nickname,
        caught_date,
        caught_location_name: &caught_location.name,
        caught_location_region: &caught_location.region,
        caught_species_dex_no: &species.dex_no,
        caught_species_form: species.form.as_deref(),
        caught_level,
        gender,
        ball
    };
    diesel::insert_into(schema::Team_Member::table)
        .values(&new_team_member)
        .execute(conn)
        .expect("Error saving new team member");
}

pub fn create_trainer_class(
    conn: &mut SqliteConnection,
    name: &str,
) -> trainer_class::TrainerClass {
    let new_trainer_class = trainer_class::InsertTrainerClass { name };
    diesel::insert_into(schema::Trainer_Class::table)
        .values(&new_trainer_class)
        .execute(conn)
        .expect("Error saving new trainer class");
    let trainer_class = schema::Trainer_Class::table
        .filter(schema::Trainer_Class::name.eq(name))
        .first::<crate::dbi::structs::trainer_class::TrainerClass>(conn)
        .expect("Error loading trainer class");
    println!("Ceated trainer class {}", trainer_class);
    trainer_class
}
