use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};

use super::{battle_type::BattleType, location::Location, playthrough::Playthrough};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrainerClass {
    pub id: i64,
    pub name: String,
}
