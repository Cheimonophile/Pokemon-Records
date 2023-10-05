use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Playthrough)]
pub struct InsertPlaythrough<'a> {
    pub id_no: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub adventure_started: &'a str,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::Playthrough)]
#[diesel(primary_key(id_no))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playthrough {
    pub id_no: String,
    pub name: String,
    pub version: String,
    pub adventure_started: String,
}

impl Playthrough {
    pub fn read(id_no: Option<&str>) -> Result<Vec<Playthrough>, Box<dyn std::error::Error>> {
        let mut connection = crate::dbi::connection::connect();
        let results = if let Some(id_no) = id_no {
            schema::Playthrough::table
                .filter(schema::Playthrough::id_no.eq(id_no))
                .select(Playthrough::as_select())
                .load(&mut connection)?
        }
        else {
            schema::Playthrough::table
                .select(Playthrough::as_select())
                .load(&mut connection)?
        };
        Ok(results)
    }

    pub fn create(
        id_no: &str,
        name: &str,
        version: &str,
        adventure_started: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut connection = crate::dbi::connection::connect();
        let new_playthrough = InsertPlaythrough {
            id_no,
            name,
            version,
            adventure_started,
        };
        diesel::insert_into(schema::Playthrough::table)
            .values(&new_playthrough)
            .execute(&mut connection)?;
        let playthrough = schema::Playthrough::table
            .filter(schema::Playthrough::id_no.eq(id_no))
            .first::<Playthrough>(&mut connection)?;
        Ok(playthrough)
    }
}

impl std::fmt::Display for Playthrough {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.version, self.adventure_started)
    }
}
