use std::collections::HashMap;

use std::hash::Hash;

use crate::error::PkmnResult;



/**
 * A trait for types in the database that can be read
 */
pub trait Read: Sized {
    type Raw: Sized;
    type Key: Eq + Hash + Sized;
    fn id(&self) -> Self::Key;
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>>;
    fn get_map(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<HashMap<Self::Key, Self>> {
        let map = Self::read(transaction)?
            .into_iter()
            .map(|item| (item.id(), item))
            .collect::<HashMap<Self::Key, Self>>();
        Ok(map)
    }
}


/**
 * A trait for types in the database that can be creates
 */
pub trait Create: Read {
    type Create: Sized;
    fn create(transaction: &mut sqlx::SqliteConnection, item: &Self::Create) -> PkmnResult<Self::Key>;
}


/**
 * A trait for types in the database that can be updated
 */
pub trait Delete: Read {
    fn delete(transaction: &mut sqlx::SqliteConnection, id: &Self::Key) -> PkmnResult<()>;
}