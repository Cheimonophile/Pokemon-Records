use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Battle_Event)]
pub struct InsertBattleEvent<'a> {
    pub no: &'a i32,
    pub battle_type: &'a str,
    pub opponent1_name: &'a str,
    pub opponent1_class: &'a str,
    pub opponent2_name: Option<&'a str>,
    pub opponent2_class: Option<&'a str>,
    pub partner_name: Option<&'a str>,
    pub partner_class: Option<&'a str>,
    pub round: &'a i32,
    pub lost: &'a bool,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Battle_Event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BattleEvent {
    pub no: i32,
    pub battle_type: String,
    pub opponent1_name: String,
    pub opponent1_class: String,
    pub opponent2_name: Option<String>,
    pub opponent2_class: Option<String>,
    pub partner_name: Option<String>,
    pub partner_class: Option<String>,
    pub round: i32,
    pub lost: bool,
}