use diesel::{dsl::max, prelude::*};

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
    location_name: &str,
    location_region: &str,
    catch_type: &str,
    slot: i32,
    species_name: &str,
    nickname: Option<&str>,
    date: &str,
    level: i32,
    ball: &str,
    gender: &str,
) -> PkmnResult<()> {
    state.transact(|connection| {
        let new_team_member = InsertTeamMember {
            playthrough_id_no,
            slot: &slot,
            nickname,
            caught_date: date,
            caught_location_name: &location_name,
            caught_location_region: &location_region,
            caught_species_name: &species_name,
            caught_level: &level,
            ball,
            gender,
        };
        diesel::insert_into(schema::Team_Member::table)
            .values(&new_team_member)
            .execute(connection)?;
        let team_member = schema::Team_Member::table
            .order(schema::Team_Member::id.desc())
            .first::<TeamMember>(connection)?;
        let new_event = InsertEvent {
            playthrough_id_no: playthrough_id_no,
            location_name: &location_name,
            location_region: &location_region,
        };
        diesel::insert_into(schema::Event::table)
            .values(&new_event)
            .execute(connection)?;
        let event = schema::Event::table
            .filter(schema::Event::playthrough_id_no.eq(&playthrough_id_no))
            .order(schema::Event::no.desc())
            .first::<Event>(connection)
            .expect("Error loading event");
        let new_catch_event = InsertCatchEvent {
            no: &event.no,
            catch_type: catch_type,
            team_member_id: &team_member.id,
        };
        diesel::insert_into(schema::Catch_Event::table)
            .values(&new_catch_event)
            .execute(connection)?;
        let new_team_member_change = InsertTeamMemberChange {
            team_member_id: &team_member.id,
            event_no: &event.no,
            level: Some(&level),
            species_name: Some(species_name),
        };
        diesel::insert_into(schema::Team_Member_Change::table)
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
        let raw_catches = schema::Catch_Event::table
            .inner_join(schema::Event::table)
            .inner_join(schema::Team_Member::table.inner_join(schema::Species::table))
            .order(schema::Catch_Event::no.desc())
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
        diesel::delete(schema::Catch_Event::table.filter(schema::Catch_Event::no.eq(no)))
            .execute(connection)?;
        diesel::delete(schema::Event::table.filter(schema::Event::no.eq(no)))
            .execute(connection)?;
        QueryResult::<()>::Ok(())
    })?;
    Ok(())
}
