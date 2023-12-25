







use diesel::prelude::*;

use crate::schema;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::catch_type)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CatchType {
    pub id: i32,
    pub name: String,
    pub detail: Option<String>,
}
