use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Team_Member)]
pub struct InsertTeamMember<'a> {
    pub playthrough_id_no:  &'a str,
    pub slot: &'a i32,
    pub nickname: Option<&'a str>,
    pub caught_date: &'a str,
    pub caught_location_name: &'a str,
    pub caught_location_region: &'a str,
    pub caught_species_dex_no: &'a i32,
    pub caught_species_form: Option<&'a str>,
    pub caught_level: &'a i32,
    pub ball: &'a str,
    pub gender: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Team_Member)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeamMember {
    pub playthrough_id_no:  String,
    pub slot: i32,
    pub nickname: Option<String>,
    pub caught_date: String,
    pub caught_location_name: String,
    pub caught_location_region: String,
    pub caught_species_dex_no: i32,
    pub caught_species_form: Option<String>,
    pub caught_level: i32,
    pub ball: String,
    pub gender: String,
}

impl std::fmt::Display for TeamMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = "".to_string();
        if let Some(nickname) = &self.nickname {
            display.push_str(&format!("{} ", nickname));
        }
        display.push_str(&format!("({})", self.slot));
        write!(f, "{}", display)
    }
}
