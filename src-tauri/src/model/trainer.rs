use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};

use super::{
    battle_type::BattleType, location::Location, playthrough::Playthrough,
    trainer_class::TrainerClass,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Trainer {
    pub id: i64,
    pub name: String,
    pub class: TrainerClass,
}
