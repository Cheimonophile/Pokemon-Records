use tauri::async_runtime::block_on;

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::Read,
    state,
};

use super::type_::Type;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub id: i64,
    pub name: String,
    pub form: Option<String>,
    pub dex_no: i64,
    pub generation: i64,
    pub type1: Type,
    pub type2: Option<Type>,
}

pub struct RawSpecies {
    pub id: i64,
    pub name: String,
    pub form: Option<String>,
    pub dex_no: i64,
    pub generation: i64,
    pub type1_id: i64,
    pub type2_id: Option<i64>,
}

impl Read for Species {
    type Raw = RawSpecies;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let results = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT species.*
                    FROM species
                    ORDER BY dex_no, form
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        let types = Type::get_map(transaction)?;

        let species = results
            .into_iter()
            .map(|raw_species| {
                let type1 = types
                    .get(&raw_species.type1_id)
                    .ok_or_else(|| {
                        StringError::new(&format!(
                            "Type with id {} not found",
                            raw_species.type1_id
                        ))
                    })?
                    .clone();
                let type2 = if let Some(type2_id) = raw_species.type2_id {
                    Some(
                        types
                            .get(&type2_id)
                            .ok_or_else(|| {
                                StringError::new(&format!("Type with id {} not found", type2_id))
                            })?
                            .clone(),
                    )
                } else {
                    None
                };
                Ok(Species {
                    id: raw_species.id,
                    name: raw_species.name,
                    form: raw_species.form,
                    dex_no: raw_species.dex_no,
                    generation: raw_species.generation,
                    type1,
                    type2,
                })
            })
            .collect::<PkmnResult<Vec<Species>>>()?;

        Ok(species)
    }
}

#[tauri::command]
pub fn read_species(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<Species>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let species = Species::read(&mut *transaction)?;
    Ok(species)
}
