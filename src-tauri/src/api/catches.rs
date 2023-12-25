use diesel::prelude::*;

use serde;

use crate::{
    dbi::structs::{
        catch_event::{CatchEvent, InsertCatchEvent},
        event::{Event, InsertEvent},
        species::Species,
        team_member::{InsertTeamMember, TeamMember},
        team_member_change::InsertTeamMemberChange,
    },
    error::PkmnResult,
    schema::{self},
    state,
};

#[tauri::command]
pub fn create_catch(
    state: tauri::State<state::GameState>,
    playthrough_id_no: &str,
    location_id: i32,
    catch_type_id: i32,
    slot: i32,
    species_id: i32,
    nickname: Option<&str>,
    date: &str,
    level: i32,
    ball_id: i32,
    gender: &str,
) -> PkmnResult<()> {
    state.transact(|connection| {
        let new_team_member = InsertTeamMember {
            playthrough_id_no,
            slot,
            nickname,
            caught_date: date,
            caught_location_id: location_id,
            caught_species_id: species_id,
            caught_level: level,
            ball_id,
            gender,
        };
        diesel::insert_into(schema::team_member::table)
            .values(&new_team_member)
            .execute(connection)?;
        let team_member = schema::team_member::table
            .order(schema::team_member::id.desc())
            .first::<TeamMember>(connection)?;
        let new_event = InsertEvent {
            playthrough_id_no: playthrough_id_no,
            location_id,
            date: &date,
        };
        diesel::insert_into(schema::event::table)
            .values(&new_event)
            .execute(connection)?;
        let event = schema::event::table
            .filter(schema::event::playthrough_id_no.eq(&playthrough_id_no))
            .order(schema::event::no.desc())
            .first::<Event>(connection)
            .expect("Error loading event");
        let new_catch_event = InsertCatchEvent {
            no: event.no,
            catch_type_id,
            team_member_id: team_member.id,
        };
        diesel::insert_into(schema::catch_event::table)
            .values(&new_catch_event)
            .execute(connection)?;
        let new_team_member_change = InsertTeamMemberChange {
            team_member_id: team_member.id,
            event_no: event.no,
            level: Some(level),
            species_id: Some(species_id),
        };
        diesel::insert_into(schema::team_member_change::table)
            .values(&new_team_member_change)
            .execute(connection)?;
        QueryResult::<()>::Ok(())
    })?;
    Ok(())
}

#[derive(serde::Serialize)]
pub struct ReadCatchesResult {
    #[serde(flatten)]
    catch: CatchEvent,
    event: Event,
    species: Species,
}

#[tauri::command]
pub fn read_catches(state: tauri::State<state::GameState>) -> PkmnResult<Vec<ReadCatchesResult>> {
    let raw_catches = state.transact(|connection: &mut SqliteConnection| {
        let raw_catches = schema::catch_event::table
            .inner_join(schema::event::table)
            .inner_join(schema::team_member::table.inner_join(schema::species::table))
            .order(schema::catch_event::no.desc())
            .select((
                CatchEvent::as_select(),
                Event::as_select(),
                Species::as_select(),
            ))
            .load::<(CatchEvent, Event, Species)>(connection)?;
        QueryResult::<Vec<(CatchEvent, Event, Species)>>::Ok(raw_catches)
    })?;
    let catches = raw_catches
        .into_iter()
        .map(|(catch, event, species)| ReadCatchesResult {
            catch,
            event,
            species,
        })
        .collect();
    Ok(catches)
}

#[tauri::command]
pub fn delete_catch(state: tauri::State<state::GameState>, no: i32) -> PkmnResult<()> {
    state.transact(|connection| {
        diesel::delete(schema::catch_event::table.filter(schema::catch_event::no.eq(no)))
            .execute(connection)?;
        diesel::delete(schema::event::table.filter(schema::event::no.eq(no)))
            .execute(connection)?;
        QueryResult::<()>::Ok(())
    })?;
    Ok(())
}
