use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::{Create, Read},
    state, util,
};

use super::trainer_class::TrainerClass;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Trainer {
    pub id: i64,
    pub name: String,
    pub class: TrainerClass,
}

pub struct RawTrainer {
    pub id: i64,
    pub name: String,
    pub class_id: i64,
}

impl Read for Trainer {
    type Raw = RawTrainer;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let trainers = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT trainer.*
                    FROM trainer
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        let trainer_classes = TrainerClass::get_map(transaction)?;

        let trainers = trainers
            .into_iter()
            .map(|raw_trainer| {
                Ok(Trainer {
                    id: raw_trainer.id,
                    name: raw_trainer.name,
                    class: trainer_classes
                        .get(&raw_trainer.class_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No trainer class found with id {}",
                                raw_trainer.class_id
                            ))
                        })?
                        .clone(),
                })
            })
            .collect::<PkmnResult<Vec<_>>>()?;

        Ok(trainers)
    }
}

pub struct CreateTrainer {
    name: String,
    class_id: i64,
}

impl Create for Trainer {
    type Create = CreateTrainer;
    fn create(
        transaction: &mut sqlx::SqliteConnection,
        create: &Self::Create,
    ) -> PkmnResult<Self::Key> {
        let util::Id { id } = block_on(
            sqlx::query_as!(
                util::Id,
                "INSERT INTO trainer (name, class_id) VALUES ($1, $2) RETURNING id",
                create.name,
                create.class_id
            )
            .fetch_one(transaction),
        )?;
        Ok(id)
    }
}

#[tauri::command]
pub fn create_trainer(
    state: tauri::State<state::GameState>,
    name: String,
    class_id: i64,
) -> PkmnResult<i64> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;

    let util::Id { id } = block_on(
        sqlx::query_as!(
            util::Id,
            "INSERT INTO trainer (name, class_id) VALUES ($1, $2) RETURNING id",
            name,
            class_id
        )
        .fetch_one(&mut *transaction),
    )?;

    transaction.commit()?;

    Ok(id)
}

#[tauri::command]
pub fn read_trainers(
    state: tauri::State<state::GameState>, 
    id: Option<i64>,
    class_id: Option<i64>
) -> PkmnResult<Vec<Trainer>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let trainers = Trainer::read(&mut *transaction)?
        .into_iter()
        .filter(|trainer| {
            let mut keep = true;
            keep &= if let Some(id) = id {
                trainer.id == id
            } else {
                true
            };
            keep &= if let Some(class_id) = class_id {
                trainer.class.id == class_id
            } else {
                true
            };
            keep
        })
        .collect::<Vec<_>>();
    Ok(trainers)
}
