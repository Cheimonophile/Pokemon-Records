use serde;
use sqlx::sqlite::SqliteRow;
use sqlx::{self, ColumnIndex, Row};
use tauri;

use crate::error::StringError;
use crate::{error::PkmnResult, state};

use crate::sqlx_ext;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Region {
    pub id: i64,
    pub name: String,
}

// macro_rules! region_from_raw {
//     ($prefix:ident) => {
//         paste::paste! {
//             Region {
//                 id: self. [< $prefix _ id >],
//                 name: self. [< $prefix _ name  >],
//             }
//         }
//     };
// }

// pub(crate) use region_from_raw;



// pub struct RawRegion {
//     pub id: Option<i64>,
//     pub name: Option<String>,
// }

// impl sqlx_ext::RawSqlx<Region> for RawRegion {
//     fn to_model(self) -> PkmnResult<Region> {
//         Ok(Region {
//             id: self
//                 .id
//                 .ok_or_else(|| StringError::new("Region id is None"))?,
//             name: self
//                 .name
//                 .ok_or_else(|| StringError::new("Region name is None"))?,
//         })
//     }
// }
