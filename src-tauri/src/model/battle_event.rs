use serde;
use sqlx::{self};
use tauri::{self, Event};

use crate::{error::PkmnResult, state};

use super::{
    battle_type::BattleType, location::Location, playthrough::Playthrough, trainer::Trainer,
};


/**
 * Represents a full recursive battle event
 */
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BattleEvent {
    pub no: i64,
    pub battle_type: BattleType,
    pub opponent1: Trainer,
    pub opponent2: Option<Trainer>,
    pub partner: Option<Trainer>,
    pub lost: bool,
    pub round: i64,
}


