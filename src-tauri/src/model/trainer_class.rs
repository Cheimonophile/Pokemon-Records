use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{
    error::PkmnResult,
    pkmndb::{Create, Read},
    state, util,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TrainerClass {
    pub id: i64,
    pub name: String,
}

impl Read for TrainerClass {
    type Raw = TrainerClass;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let trainer_classes = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name
                    FROM trainer_class
                    ORDER BY id
                "#,
            )
            .fetch_all(transaction),
        )?;
        Ok(trainer_classes)
    }
}

pub struct CreateTrainerClass {
    name: String,
}

impl Create for TrainerClass {
    type Create = CreateTrainerClass;
    fn create(
        transaction: &mut sqlx::SqliteConnection,
        create: &Self::Create,
    ) -> PkmnResult<Self::Key> {
        let util::Id { id } = block_on(
            sqlx::query_as!(
                util::Id,
                "INSERT INTO trainer_class (name) VALUES ($1) RETURNING id",
                create.name
            )
            .fetch_one(transaction),
        )?;
        Ok(id)
    }
}

#[tauri::command]
pub fn create_trainer_class(
    state: tauri::State<state::GameState>,
    name: String,
) -> PkmnResult<i64> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let create_trainer_class = CreateTrainerClass { name };
    let id = TrainerClass::create(&mut *transaction, &create_trainer_class)?;
    transaction.commit()?;
    Ok(id)
}

#[tauri::command]
pub fn read_trainer_classes(
    state: tauri::State<state::GameState>,
) -> PkmnResult<Vec<TrainerClass>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let trainer_classes = TrainerClass::read(&mut *transaction)?;
    Ok(trainer_classes)
}
