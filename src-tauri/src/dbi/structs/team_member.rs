use diesel::{prelude::*, sqlite};

use crate::{schema, dbi::structs::species};

use super::team_member_change;

#[derive(Insertable)]
#[diesel(table_name = schema::Team_Member)]
pub struct InsertTeamMember<'a> {
    pub playthrough_id_no:  &'a str,
    pub slot: &'a i32,
    pub nickname: Option<&'a str>,
    pub caught_date: &'a str,
    pub caught_location_name: &'a str,
    pub caught_location_region: &'a str,
    pub caught_species_name: &'a str,
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
    pub caught_species_name: String,
    pub caught_level: i32,
    pub ball: String,
    pub gender: String,
}



impl TeamMember {
    pub fn format(&self, conn: &mut sqlite::SqliteConnection) -> String {
        if let Some(nickname) = &self.nickname {
            nickname.into()
        } else {
            // let team_member_change = schema::Team_Member_Change::table
            //     .filter(
            //         schema::Team_Member_Change::team_member_playthrough_id_no.eq(&self.playthrough_id_no)
            //         .and(schema::Team_Member_Change::team_member_slot.eq(&self.slot))
            //         .and(schema::Team_Member_Change::species_dex_no.is_not_null()))
            //     .order(schema::Team_Member_Change::id.desc())
            //     .first::<team_member_change::TeamMemberChange>(conn)
            //     .expect("Error loading team member change");
            // let species = schema::Species::table
            //     .filter(
            //         schema::Species::dex_no.is_not_null()
            //         .and(schema::Species::dex_no.assume_not_null().eq(&team_member_change.species_dex_no)))
            //     .first::<species::Species>(conn)
            //     .expect("Error loading species");
            format!("{}", self.caught_species_name)
        }
    }
}
