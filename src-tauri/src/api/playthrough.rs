use crate::dbi::structs::playthrough::Playthrough;



#[tauri::command]
pub fn read_playthroughs(playthrough_id_no: Option<&str>) -> Option<Vec<Playthrough>> {
    let results = match Playthrough::read(playthrough_id_no) {
        Ok(playthroughs) => Some(playthroughs),
        Err(error) => {
            eprintln!("Error reading playthroughs: {}", error);
            None
        }
    };
    results
}