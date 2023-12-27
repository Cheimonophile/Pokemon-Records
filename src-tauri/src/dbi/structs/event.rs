use diesel::prelude::*;

use serde;

use crate::schema;

use super::{
    battle_event::BattleEventResult, location::LocationResult, playthrough::PlaythroughResult,
};

#[derive(Insertable)]
#[diesel(table_name = schema::event)]
pub struct InsertEvent<'a> {
    pub playthrough_id_no: &'a str,
    pub location_id: i32,
    pub date: &'a str,
}

#[derive(serde::Serialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::event)]
#[diesel(primary_key(no))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub no: i32,
    pub playthrough_id_no: String,
    pub location_id: i32,
    pub date: String,
}

#[derive(serde::Serialize)]
pub struct EventResult {
    #[serde(flatten)]
    event: Event,
    playthrough: PlaythroughResult,
    location: LocationResult,
    battle: Option<BattleEventResult>,
    // catch: Option<CatchEventResult>,
    // item: Option<ItemEventResult>,
}

// #[tauri::command]
// pub fn read_events(
//     state: tauri::State<state::GameState>,
//     playthrough_id_no: Option<String>,
// ) -> PkmnResult<Vec<EventResult>> {
//     let (events, trainers) = state.transact(|connection: &mut SqliteConnection| {
//         // events
//         let mut query = schema::event::table
//             .left_join(schema::playthrough::table.left_join(schema::version::table))
//             .left_join(schema::location::table.left_join(schema::region::table))
//             .left_join(schema::battle_event::table)
//             .left_join(schema::catch_event::table)
//             .left_join(schema::item_event::table.left_join(schema::item::table))
//             .into_boxed();
//         if let Some(playthrough_id_no) = playthrough_id_no {
//             query = query.filter(schema::event::playthrough_id_no.eq(playthrough_id_no));
//         }
//         let events: Vec<(Event, Option<BattleEvent>)> = query
//             .order(schema::event::no.desc())
//             .select((
//                 schema::event::all_columns,
//                 schema::playthrough::all_columns,
//                 schema::version::all_columns,
//                 schema::battle_event::all_columns,
//             ))
//             .load(connection)?;

//         // trainers
//         let trainers = schema::trainer::table
//             .inner_join(schema::trainer_class::table)
//             .select((
//                 schema::trainer::all_columns,
//                 schema::trainer_class::all_columns,
//             ))
//             .load::<(Trainer, TrainerClass)>(connection)?;

//         QueryResult::<_>::Ok((events, trainers))
//     })?;
//     let trainer_result_map = trainers
//         .into_iter()
//         .map(|(trainer, trainer_class)| {
//             (
//                 trainer.id,
//                 TrainerResult {
//                     trainer,
//                     class: TrainerClassResult { trainer_class },
//                 },
//             )
//         })
//         .collect::<std::collections::HashMap<_, _>>();
//     let battles = events
//         .into_iter()
//         .map(|(event, battle)| EventResult {
//             event,
//             playthrough: 1,
//             location: 1,
//             battle,
//         })
//         .collect();
//     Ok(battles)
// }
