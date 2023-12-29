
pub use sqlx_ext_derive::FromFlatSqlx;

use crate::error::PkmnResult;

pub trait FromFlatSqlx: Sized {
  type Raw: Into<Self>;
}


pub trait FromRowWithPrefix: Sized {
  fn from_row_with_prefix(prefix: &str, row: &sqlx::sqlite::SqliteRow) -> sqlx::Result<Self>;
}


pub trait RawSqlx<T>: Sized {
  fn to_model(self) -> PkmnResult<T>;
}