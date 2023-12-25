use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::battle_event)]
pub struct InsertBattleEvent {
    pub no: i32,
    pub battle_type_id: i32,
    pub opponent1_id: i32,
    pub opponent2_id: Option<i32>,
    pub partner_id: Option<i32>,
    pub round: Option<i32>,
    pub lost: bool,
}

#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::battle_event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BattleEvent {
    pub no: i32,
    pub battle_type_id: i32,
    pub opponent1_id: i32,
    pub opponent2_id: Option<i32>,
    pub partner_id: Option<i32>,
    pub round: Option<i32>,
    pub lost: bool,
}
