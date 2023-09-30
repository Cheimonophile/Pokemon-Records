use crate::dbi::connection::connect;
use crate::dbi::events::*;

pub fn run() {

    // connection
    let conn = &mut connect();

    // events
    let mut playthrough = create_playthrough(conn, "26852", "Ben", "Black", "2023-06-24");
    let nuvema_town = create_location(conn, "Nuvema Town", "Unova");
    let species_lillipup = create_species(conn,
        &506,
        "Lillipup",
        None,
        &5,
        "Normal",
        None
    );
    let lillipup = catch_pokemon(conn,
        &playthrough,
        &1,
        &species_lillipup,
        None,
        "Gift",
        "2023-06-24",
        &nuvema_town,
        &5,
        "M",
        "Poké Ball"
    );
    let pkmn_trainer = create_trainer_class(conn,
        "PKMN Trainer"
    );
    let bianca = create_trainer(conn,
        Some("Bianca"),
        &pkmn_trainer
    );
}