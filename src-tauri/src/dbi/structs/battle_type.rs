



use diesel::prelude::*;

use crate::schema;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::battle_type)]
#[diesel(primary_key(name))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BattleType {
    pub id: i32,
    pub name: String
}

#[derive(serde::Serialize)]
pub struct BattleTypeResult {
    #[serde(flatten)]
    battle_type: BattleType,
}
