







use diesel::prelude::*;

use crate::schema;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::Catch_Type)]
#[diesel(primary_key(name))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CatchType {
    pub name: String
}
