use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};

use super::version::Version;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Playthrough {
    pub id_no: String,
    pub name: String,
    pub version: Version,
    pub adventure_started: String,
}