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

#[tauri::command]
pub fn create_playthrough(
    id_no: &str,
    name: &str,
    version: &str,
    adventure_started: &str,
) -> Option<Playthrough> {
    let playthrough = match Playthrough::create(
        id_no,
        name,
        version,
        adventure_started,
    ) {
        Ok(playthrough) => Some(playthrough),
        Err(error) => {
            eprintln!("Error creating playthrough: {}", error);
            None
        }
    };
    playthrough
}
