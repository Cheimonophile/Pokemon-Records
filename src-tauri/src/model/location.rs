use serde;
use sqlx::{self, Row};
use tauri;

use crate::{error::PkmnResult, sqlx_ext::FromRowWithPrefix, state};

use super::{
    playthrough::Playthrough,
    region::{self, Region},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub region: Region,
}