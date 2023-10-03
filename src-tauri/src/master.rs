use crate::dbi::connection::connect;
use crate::dbi::events::*;
use crate::dbi::structs::*;
use crate::schema::Team_Member_Change::level;

pub fn run() {
    // connection
    let conn = &mut connect();

    let playthrough = create_playthrough(conn, "17053", "Ben", "Diamond", "2023-02-23");

    let lake_verity_sinnoh = create_location(conn, "Lake Verity", "Sinnoh");

    let species_torterra = create_species(conn, "Torterra", &389, &5, "Grass", Some("Ground"));

    let team_member_torterra = catch_pokemon(
        conn,
        &playthrough,
        &1,
        &species_torterra,
        None,
        "Gift",
        "2023-02-23",
        &lake_verity_sinnoh,
        &5,
        "M",
        "Poké Ball",
    );

    let route_204_sinnoh = create_location(conn, "Route 204", "Sinnoh");

    let species_bibarel = create_species(conn, "Bibarel", &400, &5, "Normal", Some("Water"));

    let team_member_bibarel = catch_pokemon(
        conn,
        &playthrough,
        &2,
        &species_bibarel,
        None,
        "Grass",
        "2023-02-25",
        &route_204_sinnoh,
        &4,
        "M",
        "Poké Ball",
    );

    let eterna_forest_sinnoh = create_location(conn, "Eterna Forest", "Sinnoh");

    let species_honchkrow = create_species(conn, "Honchkrow", &430, &5, "Dark", Some("Flying"));

    let team_member_honchkrow = catch_pokemon(
        conn,
        &playthrough,
        &3,
        &species_honchkrow,
        None,
        "Grass",
        "2023-03-02",
        &eterna_forest_sinnoh,
        &11,
        "F",
        "Dusk Ball",
    );

    let mining_museum_sinnoh = create_location(conn, "Mining Museum", "Sinnoh");

    let species_rampardos = create_species(conn, "Rampardos", &409, &5, "Rock", None);

    let team_member_rampardos = catch_pokemon(
        conn,
        &playthrough,
        &4,
        &species_rampardos,
        None,
        "Fossil",
        "2023-03-11",
        &mining_museum_sinnoh,
        &20,
        "M",
        "Poké Ball",
    );

    let great_marsh_sinnoh = create_location(conn, "Great Marsh", "Sinnoh");

    let species_toxicroak = create_species(conn, "Toxicroak", &454, &5, "Poison", Some("Fighting"));

    let team_member_toxicroak = catch_pokemon(
        conn,
        &playthrough,
        &5,
        &species_toxicroak,
        None,
        "Grass",
        "2023-03-16",
        &great_marsh_sinnoh,
        &23,
        "N",
        "Safari Ball",
    );

    let mt_coronet_sinnoh = create_location(conn, "Mt. Coronet", "Sinnoh");

    let species_bronzong = create_species(conn, "Bronzong", &437, &5, "Steel", Some("Psychic"));

    let team_member_bronzong = catch_pokemon(
        conn,
        &playthrough,
        &6,
        &species_bronzong,
        None,
        "Grass",
        "2023-04-11",
        &mt_coronet_sinnoh,
        &36,
        "N",
        "Ultra Ball",
    );

    let pokémon_league_sinnoh = create_location(conn, "Pokémon League", "Sinnoh");

    let elite_four = create_trainer_class(conn, "Elite Four");

    let elite_four_e4 = create_trainer(conn, "E4", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &51);

    level_up(conn, &battle, &team_member_bibarel, &46);

    level_up(conn, &battle, &team_member_honchkrow, &47);

    level_up(conn, &battle, &team_member_rampardos, &46);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &51);

    level_up(conn, &battle, &team_member_bibarel, &46);

    level_up(conn, &battle, &team_member_honchkrow, &47);

    level_up(conn, &battle, &team_member_rampardos, &47);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &52);

    level_up(conn, &battle, &team_member_bibarel, &47);

    level_up(conn, &battle, &team_member_honchkrow, &48);

    level_up(conn, &battle, &team_member_rampardos, &47);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &52);

    level_up(conn, &battle, &team_member_bibarel, &48);

    level_up(conn, &battle, &team_member_honchkrow, &48);

    level_up(conn, &battle, &team_member_rampardos, &48);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &52);

    level_up(conn, &battle, &team_member_bibarel, &49);

    level_up(conn, &battle, &team_member_honchkrow, &48);

    level_up(conn, &battle, &team_member_rampardos, &48);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &52);

    level_up(conn, &battle, &team_member_bibarel, &49);

    level_up(conn, &battle, &team_member_honchkrow, &48);

    level_up(conn, &battle, &team_member_rampardos, &49);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &53);

    level_up(conn, &battle, &team_member_bibarel, &49);

    level_up(conn, &battle, &team_member_honchkrow, &49);

    level_up(conn, &battle, &team_member_rampardos, &50);

    level_up(conn, &battle, &team_member_toxicroak, &48);

    level_up(conn, &battle, &team_member_bronzong, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &53);

    level_up(conn, &battle, &team_member_bibarel, &49);

    level_up(conn, &battle, &team_member_honchkrow, &49);

    level_up(conn, &battle, &team_member_rampardos, &50);

    level_up(conn, &battle, &team_member_toxicroak, &49);

    level_up(conn, &battle, &team_member_bronzong, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &53);

    level_up(conn, &battle, &team_member_bibarel, &49);

    level_up(conn, &battle, &team_member_honchkrow, &49);

    level_up(conn, &battle, &team_member_rampardos, &50);

    level_up(conn, &battle, &team_member_toxicroak, &50);

    level_up(conn, &battle, &team_member_bronzong, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &54);

    level_up(conn, &battle, &team_member_bibarel, &49);

    level_up(conn, &battle, &team_member_honchkrow, &50);

    level_up(conn, &battle, &team_member_rampardos, &51);

    level_up(conn, &battle, &team_member_toxicroak, &50);

    level_up(conn, &battle, &team_member_bronzong, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &55);

    level_up(conn, &battle, &team_member_bibarel, &51);

    level_up(conn, &battle, &team_member_honchkrow, &50);

    level_up(conn, &battle, &team_member_rampardos, &51);

    level_up(conn, &battle, &team_member_toxicroak, &50);

    level_up(conn, &battle, &team_member_bronzong, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &56);

    level_up(conn, &battle, &team_member_bibarel, &51);

    level_up(conn, &battle, &team_member_honchkrow, &51);

    level_up(conn, &battle, &team_member_rampardos, &52);

    level_up(conn, &battle, &team_member_toxicroak, &50);

    level_up(conn, &battle, &team_member_bronzong, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &57);

    level_up(conn, &battle, &team_member_bibarel, &51);

    level_up(conn, &battle, &team_member_honchkrow, &51);

    level_up(conn, &battle, &team_member_rampardos, &52);

    level_up(conn, &battle, &team_member_toxicroak, &52);

    level_up(conn, &battle, &team_member_bronzong, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &58);

    level_up(conn, &battle, &team_member_bibarel, &51);

    level_up(conn, &battle, &team_member_honchkrow, &52);

    level_up(conn, &battle, &team_member_rampardos, &53);

    level_up(conn, &battle, &team_member_toxicroak, &52);

    level_up(conn, &battle, &team_member_bronzong, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &59);

    level_up(conn, &battle, &team_member_bibarel, &51);

    level_up(conn, &battle, &team_member_honchkrow, &52);

    level_up(conn, &battle, &team_member_rampardos, &54);

    level_up(conn, &battle, &team_member_toxicroak, &52);

    level_up(conn, &battle, &team_member_bronzong, &53);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &60);

    level_up(conn, &battle, &team_member_bibarel, &53);

    level_up(conn, &battle, &team_member_honchkrow, &52);

    level_up(conn, &battle, &team_member_rampardos, &55);

    level_up(conn, &battle, &team_member_toxicroak, &52);

    level_up(conn, &battle, &team_member_bronzong, &53);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &61);

    level_up(conn, &battle, &team_member_bibarel, &54);

    level_up(conn, &battle, &team_member_honchkrow, &53);

    level_up(conn, &battle, &team_member_rampardos, &55);

    level_up(conn, &battle, &team_member_toxicroak, &52);

    level_up(conn, &battle, &team_member_bronzong, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &62);

    level_up(conn, &battle, &team_member_bibarel, &54);

    level_up(conn, &battle, &team_member_honchkrow, &53);

    level_up(conn, &battle, &team_member_rampardos, &55);

    level_up(conn, &battle, &team_member_toxicroak, &54);

    level_up(conn, &battle, &team_member_bronzong, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &63);

    level_up(conn, &battle, &team_member_bibarel, &54);

    level_up(conn, &battle, &team_member_honchkrow, &54);

    level_up(conn, &battle, &team_member_rampardos, &56);

    level_up(conn, &battle, &team_member_toxicroak, &54);

    level_up(conn, &battle, &team_member_bronzong, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &64);

    level_up(conn, &battle, &team_member_bibarel, &54);

    level_up(conn, &battle, &team_member_honchkrow, &54);

    level_up(conn, &battle, &team_member_rampardos, &57);

    level_up(conn, &battle, &team_member_toxicroak, &55);

    level_up(conn, &battle, &team_member_bronzong, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &65);

    level_up(conn, &battle, &team_member_bibarel, &54);

    level_up(conn, &battle, &team_member_honchkrow, &55);

    level_up(conn, &battle, &team_member_rampardos, &58);

    level_up(conn, &battle, &team_member_toxicroak, &55);

    level_up(conn, &battle, &team_member_bronzong, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &66);

    level_up(conn, &battle, &team_member_bibarel, &56);

    level_up(conn, &battle, &team_member_honchkrow, &55);

    level_up(conn, &battle, &team_member_rampardos, &58);

    level_up(conn, &battle, &team_member_toxicroak, &55);

    level_up(conn, &battle, &team_member_bronzong, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &67);

    level_up(conn, &battle, &team_member_bibarel, &56);

    level_up(conn, &battle, &team_member_honchkrow, &55);

    level_up(conn, &battle, &team_member_rampardos, &59);

    level_up(conn, &battle, &team_member_toxicroak, &55);

    level_up(conn, &battle, &team_member_bronzong, &56);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &68);

    level_up(conn, &battle, &team_member_bibarel, &56);

    level_up(conn, &battle, &team_member_honchkrow, &56);

    level_up(conn, &battle, &team_member_rampardos, &59);

    level_up(conn, &battle, &team_member_toxicroak, &56);

    level_up(conn, &battle, &team_member_bronzong, &56);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &69);

    level_up(conn, &battle, &team_member_bibarel, &56);

    level_up(conn, &battle, &team_member_honchkrow, &58);

    level_up(conn, &battle, &team_member_rampardos, &59);

    level_up(conn, &battle, &team_member_toxicroak, &56);

    level_up(conn, &battle, &team_member_bronzong, &56);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &69);

    level_up(conn, &battle, &team_member_bibarel, &58);

    level_up(conn, &battle, &team_member_honchkrow, &58);

    level_up(conn, &battle, &team_member_rampardos, &60);

    level_up(conn, &battle, &team_member_toxicroak, &56);

    level_up(conn, &battle, &team_member_bronzong, &56);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &70);

    level_up(conn, &battle, &team_member_bibarel, &58);

    level_up(conn, &battle, &team_member_honchkrow, &58);

    level_up(conn, &battle, &team_member_rampardos, &60);

    level_up(conn, &battle, &team_member_toxicroak, &57);

    level_up(conn, &battle, &team_member_bronzong, &57);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &70);

    level_up(conn, &battle, &team_member_bibarel, &58);

    level_up(conn, &battle, &team_member_honchkrow, &58);

    level_up(conn, &battle, &team_member_rampardos, &60);

    level_up(conn, &battle, &team_member_toxicroak, &59);

    level_up(conn, &battle, &team_member_bronzong, &57);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &71);

    level_up(conn, &battle, &team_member_bibarel, &58);

    level_up(conn, &battle, &team_member_honchkrow, &59);

    level_up(conn, &battle, &team_member_rampardos, &61);

    level_up(conn, &battle, &team_member_toxicroak, &59);

    level_up(conn, &battle, &team_member_bronzong, &59);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &71);

    level_up(conn, &battle, &team_member_bibarel, &60);

    level_up(conn, &battle, &team_member_honchkrow, &59);

    level_up(conn, &battle, &team_member_rampardos, &61);

    level_up(conn, &battle, &team_member_toxicroak, &59);

    level_up(conn, &battle, &team_member_bronzong, &59);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &72);

    level_up(conn, &battle, &team_member_bibarel, &60);

    level_up(conn, &battle, &team_member_honchkrow, &61);

    level_up(conn, &battle, &team_member_rampardos, &62);

    level_up(conn, &battle, &team_member_toxicroak, &60);

    level_up(conn, &battle, &team_member_bronzong, &59);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &73);

    level_up(conn, &battle, &team_member_bibarel, &61);

    level_up(conn, &battle, &team_member_honchkrow, &61);

    level_up(conn, &battle, &team_member_rampardos, &63);

    level_up(conn, &battle, &team_member_toxicroak, &60);

    level_up(conn, &battle, &team_member_bronzong, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &73);

    level_up(conn, &battle, &team_member_bibarel, &61);

    level_up(conn, &battle, &team_member_honchkrow, &62);

    level_up(conn, &battle, &team_member_rampardos, &64);

    level_up(conn, &battle, &team_member_toxicroak, &62);

    level_up(conn, &battle, &team_member_bronzong, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &74);

    level_up(conn, &battle, &team_member_bibarel, &63);

    level_up(conn, &battle, &team_member_honchkrow, &62);

    level_up(conn, &battle, &team_member_rampardos, &64);

    level_up(conn, &battle, &team_member_toxicroak, &62);

    level_up(conn, &battle, &team_member_bronzong, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &75);

    level_up(conn, &battle, &team_member_bibarel, &63);

    level_up(conn, &battle, &team_member_honchkrow, &62);

    level_up(conn, &battle, &team_member_rampardos, &65);

    level_up(conn, &battle, &team_member_toxicroak, &62);

    level_up(conn, &battle, &team_member_bronzong, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &75);

    level_up(conn, &battle, &team_member_bibarel, &63);

    level_up(conn, &battle, &team_member_honchkrow, &64);

    level_up(conn, &battle, &team_member_rampardos, &66);

    level_up(conn, &battle, &team_member_toxicroak, &63);

    level_up(conn, &battle, &team_member_bronzong, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &76);

    level_up(conn, &battle, &team_member_bibarel, &65);

    level_up(conn, &battle, &team_member_honchkrow, &64);

    level_up(conn, &battle, &team_member_rampardos, &66);

    level_up(conn, &battle, &team_member_toxicroak, &63);

    level_up(conn, &battle, &team_member_bronzong, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_sinnoh,
        &elite_four_e4,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &76);

    level_up(conn, &battle, &team_member_bibarel, &65);

    level_up(conn, &battle, &team_member_honchkrow, &64);

    level_up(conn, &battle, &team_member_rampardos, &67);

    level_up(conn, &battle, &team_member_toxicroak, &65);

    level_up(conn, &battle, &team_member_bronzong, &64);

    let victory_road_sinnoh = create_location(conn, "Victory Road", "Sinnoh");

    let ace_trainer = create_trainer_class(conn, "Ace Trainer");

    let ace_trainer_micah = create_trainer(conn, "Micah", &ace_trainer);

    let ace_trainer_brandi = create_trainer(conn, "Brandi", &ace_trainer);

    let marley = create_trainer_class(conn, "Marley");

    let marley = create_trainer(conn, "", &marley);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &ace_trainer_micah,
        Some(&ace_trainer_brandi),
        Some(&marley),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_torterra, &77);

    level_up(conn, &battle, &team_member_bronzong, &65);

    let psychic = create_trainer_class(conn, "Psychic");

    let psychic_desiree = create_trainer(conn, "Desiree", &psychic);

    let psychic_landon = create_trainer(conn, "Landon", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &psychic_desiree,
        Some(&psychic_landon),
        Some(&marley),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_honchkrow, &65);

    let black_belt = create_trainer_class(conn, "Black Belt");

    let black_belt_eddie = create_trainer(conn, "Eddie", &black_belt);

    let veteran = create_trainer_class(conn, "Veteran");

    let veteran_terrell = create_trainer(conn, "Terrell", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &black_belt_eddie,
        Some(&veteran_terrell),
        Some(&marley),
        "Double",
        &0,
        &false,
    );

    let bird_keeper = create_trainer_class(conn, "Bird Keeper");

    let bird_keeper_autumn = create_trainer(conn, "Autumn", &bird_keeper);

    let dragon_tamer = create_trainer_class(conn, "Dragon Tamer");

    let dragon_tamer_joe = create_trainer(conn, "Joe", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &bird_keeper_autumn,
        Some(&dragon_tamer_joe),
        Some(&marley),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_honchkrow, &66);

    let psychic_deardra = create_trainer(conn, "Deardra", &psychic);

    let psychic_kendra = create_trainer(conn, "Kendra", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &psychic_deardra,
        Some(&psychic_kendra),
        Some(&marley),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_toxicroak, &66);

    let black_belt_willie = create_trainer(conn, "Willie", &black_belt);

    let veteran_brenden = create_trainer(conn, "Brenden", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &black_belt_willie,
        Some(&veteran_brenden),
        Some(&marley),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bibarel, &66);

    let ace_trainer_arthur = create_trainer(conn, "Arthur", &ace_trainer);

    let ace_trainer_clarice = create_trainer(conn, "Clarice", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_sinnoh,
        &ace_trainer_arthur,
        Some(&ace_trainer_clarice),
        None,
        "Double",
        &0,
        &false,
    );

    let route_224_sinnoh = create_location(conn, "Route 224", "Sinnoh");

    let ace_trainer_ruben = create_trainer(conn, "Ruben", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &ace_trainer_ruben,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_brittney = create_trainer(conn, "Brittney", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &psychic_brittney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bronzong, &66);

    let psychic_maxwell = create_trainer(conn, "Maxwell", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &psychic_maxwell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let veteran_armando = create_trainer(conn, "Armando", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &veteran_armando,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let black_belt_carl = create_trainer(conn, "Carl", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &black_belt_carl,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ruin_maniac = create_trainer_class(conn, "Ruin Maniac");

    let ruin_maniac_larry = create_trainer(conn, "Larry", &ruin_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &ruin_maniac_larry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bronzong, &67);

    let ace_trainer_jamie = create_trainer(conn, "Jamie", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &ace_trainer_jamie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dragon_tamer_hayden = create_trainer(conn, "Hayden", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_224_sinnoh,
        &dragon_tamer_hayden,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_honchkrow, &67);

    let route_225_sinnoh = create_location(conn, "Route 225", "Sinnoh");

    let pkmn_ranger = create_trainer_class(conn, "PKMN Ranger");

    let pkmn_ranger_ashlee = create_trainer(conn, "Ashlee", &pkmn_ranger);

    let bird_keeper_audrey = create_trainer(conn, "Audrey", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &pkmn_ranger_ashlee,
        Some(&bird_keeper_audrey),
        None,
        "Double",
        &0,
        &false,
    );

    let psychic_daisy = create_trainer(conn, "Daisy", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &psychic_daisy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_dwayne = create_trainer(conn, "Dwayne", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &pkmn_ranger_dwayne,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_rodolfo = create_trainer(conn, "Rodolfo", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &ace_trainer_rodolfo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_quinn = create_trainer(conn, "Quinn", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &ace_trainer_quinn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_toxicroak, &67);

    let ace_trainer_deanna = create_trainer(conn, "Deanna", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &ace_trainer_deanna,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dragon_tamer_geoffrey = create_trainer(conn, "Geoffrey", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_225_sinnoh,
        &dragon_tamer_geoffrey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_226_sinnoh = create_location(conn, "Route 226", "Sinnoh");

    let ace_trainer_graham = create_trainer(conn, "Graham", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_226_sinnoh,
        &ace_trainer_graham,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_geneva = create_trainer(conn, "Geneva", &bird_keeper);

    let dragon_tamer_stanley = create_trainer(conn, "Stanley", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_226_sinnoh,
        &bird_keeper_geneva,
        Some(&dragon_tamer_stanley),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bibarel, &67);

    let swimmer = create_trainer_class(conn, "Swimmer");

    let swimmer_lydia = create_trainer(conn, "Lydia", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_226_sinnoh,
        &swimmer_lydia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_wade = create_trainer(conn, "Wade", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_226_sinnoh,
        &swimmer_wade,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_228_sinnoh = create_location(conn, "Route 228", "Sinnoh");

    let ace_trainer_jose = create_trainer(conn, "Jose", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &ace_trainer_jose,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bibarel, &68);

    let black_belt_davon = create_trainer(conn, "Davon", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &black_belt_davon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_ariana = create_trainer(conn, "Ariana", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &ace_trainer_ariana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_rampardos, &68);

    let dragon_tamer_keegan = create_trainer(conn, "Keegan", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &dragon_tamer_keegan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_kyler = create_trainer(conn, "Kyler", &pkmn_ranger);

    let pkmn_ranger_krista = create_trainer(conn, "Krista", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &pkmn_ranger_kyler,
        Some(&pkmn_ranger_krista),
        None,
        "Double",
        &0,
        &false,
    );

    let ace_trainer_meagan = create_trainer(conn, "Meagan", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &ace_trainer_meagan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bronzong, &68);

    let psychic_corbin = create_trainer(conn, "Corbin", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_228_sinnoh,
        &psychic_corbin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_229_sinnoh = create_location(conn, "Route 229", "Sinnoh");

    let ace_trainer_felix = create_trainer(conn, "Felix", &ace_trainer);

    let ace_trainer_dana = create_trainer(conn, "Dana", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_229_sinnoh,
        &ace_trainer_felix,
        Some(&ace_trainer_dana),
        None,
        "Double",
        &0,
        &false,
    );

    let pkmn_ranger_deshawn = create_trainer(conn, "Deshawn", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_229_sinnoh,
        &pkmn_ranger_deshawn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_sandra = create_trainer(conn, "Sandra", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_229_sinnoh,
        &pkmn_ranger_sandra,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_honchkrow, &68);

    let route_230_sinnoh = create_location(conn, "Route 230", "Sinnoh");

    let swimmer_kurt = create_trainer(conn, "Kurt", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_230_sinnoh,
        &swimmer_kurt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_toxicroak, &68);

    let swimmer_joanna = create_trainer(conn, "Joanna", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_230_sinnoh,
        &swimmer_joanna,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_mallory = create_trainer(conn, "Mallory", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_230_sinnoh,
        &swimmer_mallory,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_glenn = create_trainer(conn, "Glenn", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_230_sinnoh,
        &swimmer_glenn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_sophia = create_trainer(conn, "Sophia", &swimmer);

    let swimmer_sam = create_trainer(conn, "Sam", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_230_sinnoh,
        &swimmer_sophia,
        Some(&swimmer_sam),
        None,
        "Double",
        &0,
        &false,
    );

    let route_227_sinnoh = create_location(conn, "Route 227", "Sinnoh");

    let ace_trainer_saul = create_trainer(conn, "Saul", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_227_sinnoh,
        &ace_trainer_saul,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_toxicroak, &69);

    let pkmn_ranger_felicia = create_trainer(conn, "Felicia", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_227_sinnoh,
        &pkmn_ranger_felicia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_mikayla = create_trainer(conn, "Mikayla", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_227_sinnoh,
        &ace_trainer_mikayla,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_griffin = create_trainer(conn, "Griffin", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_227_sinnoh,
        &black_belt_griffin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dragon_tamer_darien = create_trainer(conn, "Darien", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_227_sinnoh,
        &dragon_tamer_darien,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bibarel, &69);

    let stark_mountain_sinnoh = create_location(conn, "Stark Mountain", "Sinnoh");

    let psychic_chelsey = create_trainer(conn, "Chelsey", &psychic);

    let psychic_sterling = create_trainer(conn, "Sterling", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &psychic_chelsey,
        Some(&psychic_sterling),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_rampardos, &69);

    let veteran_harlan = create_trainer(conn, "Harlan", &veteran);

    let dragon_tamer_kenny = create_trainer(conn, "Kenny", &dragon_tamer);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &veteran_harlan,
        Some(&dragon_tamer_kenny),
        None,
        "Double",
        &0,
        &false,
    );

    let ace_trainer_skylar = create_trainer(conn, "Skylar", &ace_trainer);

    let ace_trainer_natasha = create_trainer(conn, "Natasha", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &ace_trainer_skylar,
        Some(&ace_trainer_natasha),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bronzong, &69);

    let dragon_tamer_drake = create_trainer(conn, "Drake", &dragon_tamer);

    let black_belt_jarrett = create_trainer(conn, "Jarrett", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &dragon_tamer_drake,
        Some(&black_belt_jarrett),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bronzong, &70);

    let bird_keeper_krystal = create_trainer(conn, "Krystal", &bird_keeper);

    let black_belt_ray = create_trainer(conn, "Ray", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &bird_keeper_krystal,
        Some(&black_belt_ray),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_honchkrow, &69);

    let ace_trainer_stefan = create_trainer(conn, "Stefan", &ace_trainer);

    let ace_trainer_jasmin = create_trainer(conn, "Jasmin", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &ace_trainer_stefan,
        Some(&ace_trainer_jasmin),
        None,
        "Double",
        &0,
        &false,
    );

    let ace_trainer_keenan = create_trainer(conn, "Keenan", &ace_trainer);

    let ace_trainer_kassandra = create_trainer(conn, "Kassandra", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &stark_mountain_sinnoh,
        &ace_trainer_keenan,
        Some(&ace_trainer_kassandra),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_honchkrow, &70);

    let playthrough = create_playthrough(conn, "11139", "Ben", "Heartgold", "2023-04-19");

    let cherrygrove_city_johto = create_location(conn, "Cherrygrove City", "Johto");

    let species_raichu = create_species(conn, "Raichu", &26, &5, "Electric", None);

    let team_member_raichu = catch_pokemon(
        conn,
        &playthrough,
        &1,
        &species_raichu,
        None,
        "Gift",
        "2023-04-20",
        &cherrygrove_city_johto,
        &5,
        "M",
        "Poké Ball",
    );

    let route_29_johto = create_location(conn, "Route 29", "Johto");

    let species_noctowl = create_species(conn, "Noctowl", &164, &5, "Normal", Some("Flying"));

    let team_member_noctowl = catch_pokemon(
        conn,
        &playthrough,
        &2,
        &species_noctowl,
        None,
        "Grass",
        "2023-04-20",
        &route_29_johto,
        &3,
        "F",
        "Poké Ball",
    );

    let route_35_johto = create_location(conn, "Route 35", "Johto");

    let species_yanmega = create_species(conn, "Yanmega", &469, &5, "Bug", Some("Flying"));

    let team_member_yanmega = catch_pokemon(
        conn,
        &playthrough,
        &3,
        &species_yanmega,
        None,
        "Grass",
        "2023-04-22",
        &route_35_johto,
        &12,
        "M",
        "Net Ball",
    );

    let burned_tower_johto = create_location(conn, "Burned Tower", "Johto");

    let species_magmortar = create_species(conn, "Magmortar", &467, &5, "Fire", None);

    let team_member_magmortar = catch_pokemon(
        conn,
        &playthrough,
        &4,
        &species_magmortar,
        None,
        "Grass",
        "2023-04-23",
        &burned_tower_johto,
        &16,
        "M",
        "Poké Ball",
    );

    let whirl_islands_johto = create_location(conn, "Whirl Islands", "Johto");

    let species_kingdra = create_species(conn, "Kingdra", &230, &5, "Water", Some("Dragon"));

    let team_member_kingdra = catch_pokemon(
        conn,
        &playthrough,
        &5,
        &species_kingdra,
        None,
        "Grass",
        "2023-04-24",
        &whirl_islands_johto,
        &20,
        "F",
        "Lure Ball",
    );

    let safari_zone_johto = create_location(conn, "Safari Zone", "Johto");

    let species_tyranitar = create_species(conn, "Tyranitar", &248, &5, "Rock", Some("Dark"));

    let team_member_tyranitar = catch_pokemon(
        conn,
        &playthrough,
        &6,
        &species_tyranitar,
        None,
        "Grass",
        "2023-04-24",
        &safari_zone_johto,
        &17,
        "N",
        "Safari Ball",
    );

    let rival = create_trainer_class(conn, "Rival");

    let rival_theo = create_trainer(conn, "Theo", &rival);

    let battle = create_battle(
        conn,
        &playthrough,
        &cherrygrove_city_johto,
        &rival_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &6);

    level_up(conn, &battle, &team_member_noctowl, &3);

    let route_30_johto = create_location(conn, "Route 30", "Johto");

    let youngster = create_trainer_class(conn, "Youngster");

    let youngster_joey = create_trainer(conn, "Joey", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_30_johto,
        &youngster_joey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let youngster_mikey = create_trainer(conn, "Mikey", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_30_johto,
        &youngster_mikey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &4);

    let bug_catcher = create_trainer_class(conn, "Bug Catcher");

    let bug_catcher_don = create_trainer(conn, "Don", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_30_johto,
        &bug_catcher_don,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &5);

    let route_31_johto = create_location(conn, "Route 31", "Johto");

    let bug_catcher_wade = create_trainer(conn, "Wade", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_31_johto,
        &bug_catcher_wade,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let sprout_tower_johto = create_location(conn, "Sprout Tower", "Johto");

    let sage = create_trainer_class(conn, "Sage");

    let sage_nico = create_trainer(conn, "Nico", &sage);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &sage_nico,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &7);

    level_up(conn, &battle, &team_member_noctowl, &6);

    let sage_chow = create_trainer(conn, "Chow", &sage);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &sage_chow,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &7);

    let sage_edmond = create_trainer(conn, "Edmond", &sage);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &sage_edmond,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &8);

    let sage_jin = create_trainer(conn, "Jin", &sage);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &sage_jin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let sage_neal = create_trainer(conn, "Neal", &sage);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &sage_neal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &8);

    let sage_troy = create_trainer(conn, "Troy", &sage);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &sage_troy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &9);

    let elder = create_trainer_class(conn, "Elder");

    let elder_li = create_trainer(conn, "Li", &elder);

    let battle = create_battle(
        conn,
        &playthrough,
        &sprout_tower_johto,
        &elder_li,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &9);

    let violet_city_johto = create_location(conn, "Violet City", "Johto");

    let bird_keeper_abe = create_trainer(conn, "Abe", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &violet_city_johto,
        &bird_keeper_abe,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &10);

    let bird_keeper_rod = create_trainer(conn, "Rod", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &violet_city_johto,
        &bird_keeper_rod,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader = create_trainer_class(conn, "Leader");

    let leader_faulkner = create_trainer(conn, "Faulkner", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &violet_city_johto,
        &leader_faulkner,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &10);

    let battle = create_battle(
        conn,
        &playthrough,
        &violet_city_johto,
        &leader_faulkner,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &violet_city_johto,
        &leader_faulkner,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &violet_city_johto,
        &leader_faulkner,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &11);

    level_up(conn, &battle, &team_member_noctowl, &11);

    let route_32_johto = create_location(conn, "Route 32", "Johto");

    let youngster_albert = create_trainer(conn, "Albert", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &youngster_albert,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker = create_trainer_class(conn, "Picnicker");

    let picnicker_liz = create_trainer(conn, "Liz", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &picnicker_liz,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman = create_trainer_class(conn, "Fisherman");

    let fisherman_henry = create_trainer(conn, "Henry", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &fisherman_henry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &12);

    let fisherman_justin = create_trainer(conn, "Justin", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &fisherman_justin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_ralph = create_trainer(conn, "Ralph", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &fisherman_ralph,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let youngster_gordon = create_trainer(conn, "Gordon", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &youngster_gordon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper = create_trainer_class(conn, "Camper");

    let camper_roland = create_trainer(conn, "Roland", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_32_johto,
        &camper_roland,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &12);

    let route_33_johto = create_location(conn, "Route 33", "Johto");

    let bird_keeper_peter = create_trainer(conn, "Peter", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_33_johto,
        &bird_keeper_peter,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let union_cave_johto = create_location(conn, "Union Cave", "Johto");

    let firebreather = create_trainer_class(conn, "Firebreather");

    let firebreather_bill = create_trainer(conn, "Bill", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &union_cave_johto,
        &firebreather_bill,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &13);

    let firebreather_ray = create_trainer(conn, "Ray", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &union_cave_johto,
        &firebreather_ray,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &13);

    let hiker = create_trainer_class(conn, "Hiker");

    let hiker_russel = create_trainer(conn, "Russel", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &union_cave_johto,
        &hiker_russel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let poké_maniac = create_trainer_class(conn, "Poké Maniac");

    let poké_maniac_larry = create_trainer(conn, "Larry", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &union_cave_johto,
        &poké_maniac_larry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let slowpoke_well_johto = create_location(conn, "Slowpoke Well", "Johto");

    let team_rocket_grunt = create_trainer_class(conn, "Team Rocket Grunt");

    let team_rocket_grunt_nan = create_trainer(conn, "nan", &team_rocket_grunt);

    let battle = create_battle(
        conn,
        &playthrough,
        &slowpoke_well_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &slowpoke_well_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &14);

    level_up(conn, &battle, &team_member_noctowl, &14);

    let battle = create_battle(
        conn,
        &playthrough,
        &slowpoke_well_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let executive = create_trainer_class(conn, "Executive");

    let executive_proton = create_trainer(conn, "Proton", &executive);

    let battle = create_battle(
        conn,
        &playthrough,
        &slowpoke_well_johto,
        &executive_proton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let azalea_town_johto = create_location(conn, "Azalea Town", "Johto");

    let bug_catcher_al = create_trainer(conn, "Al", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &bug_catcher_al,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_benny = create_trainer(conn, "Benny", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &bug_catcher_benny,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &15);

    level_up(conn, &battle, &team_member_noctowl, &15);

    let twins = create_trainer_class(conn, "Twins");

    let twins_amy_and_mimi = create_trainer(conn, "Amy & Mimi", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &twins_amy_and_mimi,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let bug_catcher_josh = create_trainer(conn, "Josh", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &bug_catcher_josh,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_bugsy = create_trainer(conn, "Bugsy", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_daniel = create_trainer(conn, "Daniel", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &union_cave_johto,
        &hiker_daniel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_anthony = create_trainer(conn, "Anthony", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_33_johto,
        &hiker_anthony,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &16);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &16);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &17);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &18);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &leader_bugsy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &17);

    let battle = create_battle(
        conn,
        &playthrough,
        &azalea_town_johto,
        &rival_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &19);

    let route_34_johto = create_location(conn, "Route 34", "Johto");

    let youngster_samuel = create_trainer(conn, "Samuel", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_34_johto,
        &youngster_samuel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan = create_trainer_class(conn, "Pokéfan");

    let pokéfan_brandon = create_trainer(conn, "Brandon", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_34_johto,
        &pokéfan_brandon,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &20);

    let youngster_ian = create_trainer(conn, "Ian", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_34_johto,
        &youngster_ian,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_gina = create_trainer(conn, "Gina", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_34_johto,
        &picnicker_gina,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &18);

    let policeman = create_trainer_class(conn, "Policeman");

    let policeman_keith = create_trainer(conn, "Keith", &policeman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_34_johto,
        &policeman_keith,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper_todd = create_trainer(conn, "Todd", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_34_johto,
        &camper_todd,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_kim = create_trainer(conn, "Kim", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &picnicker_kim,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper_elliot = create_trainer(conn, "Elliot", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &camper_elliot,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_brooke = create_trainer(conn, "Brooke", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &picnicker_brooke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &19);

    let camper_ivan = create_trainer(conn, "Ivan", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &camper_ivan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &21);

    let juggler = create_trainer_class(conn, "Juggler");

    let juggler_irwin = create_trainer(conn, "Irwin", &juggler);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &juggler_irwin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let firebreather_walt = create_trainer(conn, "Walt", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &firebreather_walt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &22);

    let policeman_dirk = create_trainer(conn, "Dirk", &policeman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &policeman_dirk,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_arnie = create_trainer(conn, "Arnie", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &bug_catcher_arnie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_bryan = create_trainer(conn, "Bryan", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_35_johto,
        &bird_keeper_bryan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &20);

    let national_park_johto = create_location(conn, "National Park", "Johto");

    let school_kid = create_trainer_class(conn, "School Kid");

    let school_kid_jack = create_trainer(conn, "Jack", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &national_park_johto,
        &school_kid_jack,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_beverly = create_trainer(conn, "Beverly", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &national_park_johto,
        &pokéfan_beverly,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lass = create_trainer_class(conn, "Lass");

    let lass_krise = create_trainer(conn, "Krise", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &national_park_johto,
        &lass_krise,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &23);

    let pokéfan_william = create_trainer(conn, "William", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &national_park_johto,
        &pokéfan_william,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_36_johto = create_location(conn, "Route 36", "Johto");

    let psychic_mark = create_trainer(conn, "Mark", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_36_johto,
        &psychic_mark,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &21);

    let school_kid_alan = create_trainer(conn, "Alan", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_36_johto,
        &school_kid_alan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &12);

    let goldenrod_city_johto = create_location(conn, "Goldenrod City", "Johto");

    let beauty = create_trainer_class(conn, "Beauty");

    let beauty_victoria = create_trainer(conn, "Victoria", &beauty);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &beauty_victoria,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let beauty_samantha = create_trainer(conn, "Samantha", &beauty);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &beauty_samantha,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lass_carrie = create_trainer(conn, "Carrie", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &lass_carrie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &13);

    let lass_cathy = create_trainer(conn, "Cathy", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &lass_cathy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_whitney = create_trainer(conn, "Whitney", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &14);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &15);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &24);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_city_johto,
        &leader_whitney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &16);

    let route_37_johto = create_location(conn, "Route 37", "Johto");

    let twins_tori_and_til = create_trainer(conn, "Tori & Til", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_37_johto,
        &twins_tori_and_til,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &22);

    let beauty_callie = create_trainer(conn, "Callie", &beauty);

    let beauty_kassandra = create_trainer(conn, "Kassandra", &beauty);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_37_johto,
        &beauty_callie,
        Some(&beauty_kassandra),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &17);

    let psychic_greg = create_trainer(conn, "Greg", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_37_johto,
        &psychic_greg,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &burned_tower_johto,
        &rival_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &18);

    let firebreather_ned = create_trainer(conn, "Ned", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &burned_tower_johto,
        &firebreather_ned,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &19);

    let firebreather_richard = create_trainer(conn, "Richard", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &burned_tower_johto,
        &firebreather_richard,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &25);

    let ecruteak_city_johto = create_location(conn, "Ecruteak City", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_38_johto = create_location(conn, "Route 38", "Johto");

    let bird_keeper_toby = create_trainer(conn, "Toby", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_38_johto,
        &bird_keeper_toby,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &23);

    let sailor = create_trainer_class(conn, "Sailor");

    let sailor_harry = create_trainer(conn, "Harry", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_38_johto,
        &sailor_harry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &20);

    let lass_dana = create_trainer(conn, "Dana", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_38_johto,
        &lass_dana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let beauty_valerie = create_trainer(conn, "Valerie", &beauty);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_38_johto,
        &beauty_valerie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_chad = create_trainer(conn, "Chad", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_38_johto,
        &school_kid_chad,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_39_johto = create_location(conn, "Route 39", "Johto");

    let psychic_nelson = create_trainer(conn, "Nelson", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_39_johto,
        &psychic_nelson,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &21);

    let sailor_eugene = create_trainer(conn, "Eugene", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_39_johto,
        &sailor_eugene,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &24);

    let pokéfan_ruth = create_trainer(conn, "Ruth", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_39_johto,
        &pokéfan_ruth,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_derek = create_trainer(conn, "Derek", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_39_johto,
        &pokéfan_derek,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let olivine_city_johto = create_location(conn, "Olivine City", "Johto");

    let beauty_charlotte = create_trainer(conn, "Charlotte", &beauty);

    let battle = create_battle(
        conn,
        &playthrough,
        &olivine_city_johto,
        &beauty_charlotte,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &22);

    let shining_lighthouse_johto = create_location(conn, "Shining Lighthouse", "Johto");

    let gentleman = create_trainer_class(conn, "Gentleman");

    let gentleman_alfred = create_trainer(conn, "Alfred", &gentleman);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &gentleman_alfred,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let sailor_huey = create_trainer(conn, "Huey", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &sailor_huey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_theo = create_trainer(conn, "Theo", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &bird_keeper_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &25);

    let gentleman_preston = create_trainer(conn, "Preston", &gentleman);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &gentleman_preston,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &23);

    let lass_connie = create_trainer(conn, "Connie", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &lass_connie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let sailor_kent = create_trainer(conn, "Kent", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &sailor_kent,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_denis = create_trainer(conn, "Denis", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &bird_keeper_denis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let sailor_terrell = create_trainer(conn, "Terrell", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &sailor_terrell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &26);

    level_up(conn, &battle, &team_member_yanmega, &24);

    let sailor_roberto = create_trainer(conn, "Roberto", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &shining_lighthouse_johto,
        &sailor_roberto,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let mt_mortar_johto = create_location(conn, "Mt. Mortar", "Johto");

    let poké_maniac_harrison = create_trainer(conn, "Harrison", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_mortar_johto,
        &poké_maniac_harrison,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &26);

    let route_42_johto = create_location(conn, "Route 42", "Johto");

    let fisherman_tully = create_trainer(conn, "Tully", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_42_johto,
        &fisherman_tully,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let poké_maniac_shane = create_trainer(conn, "Shane", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_42_johto,
        &poké_maniac_shane,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &25);

    let hiker_benjamin = create_trainer(conn, "Benjamin", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_42_johto,
        &hiker_benjamin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_43_johto = create_location(conn, "Route 43", "Johto");

    let camper_spencer = create_trainer(conn, "Spencer", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_43_johto,
        &camper_spencer,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_tiffany = create_trainer(conn, "Tiffany", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_43_johto,
        &picnicker_tiffany,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let poké_maniac_brent = create_trainer(conn, "Brent", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_43_johto,
        &poké_maniac_brent,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &27);

    level_up(conn, &battle, &team_member_yanmega, &26);

    level_up(conn, &battle, &team_member_magmortar, &16);

    let poké_maniac_beckett = create_trainer(conn, "Beckett", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_43_johto,
        &poké_maniac_beckett,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let poké_maniac_ron = create_trainer(conn, "Ron", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_43_johto,
        &poké_maniac_ron,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &17);

    let fisherman_marvin = create_trainer(conn, "Marvin", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_43_johto,
        &fisherman_marvin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let medium = create_trainer_class(conn, "Medium");

    let medium_georgina = create_trainer(conn, "Georgina", &medium);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &medium_georgina,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &18);

    let medium_grace = create_trainer(conn, "Grace", &medium);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &medium_grace,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &19);

    let medium_edith = create_trainer(conn, "Edith", &medium);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &medium_edith,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &28);

    let medium_martha = create_trainer(conn, "Martha", &medium);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &medium_martha,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &20);

    let leader_morty = create_trainer(conn, "Morty", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &leader_morty,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &21);

    let route_40_johto = create_location(conn, "Route 40", "Johto");

    let swimmer_simon = create_trainer(conn, "Simon", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_40_johto,
        &swimmer_simon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_elaine = create_trainer(conn, "Elaine", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_40_johto,
        &swimmer_elaine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_randall = create_trainer(conn, "Randall", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_40_johto,
        &swimmer_randall,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let swimmer_paula = create_trainer(conn, "Paula", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_40_johto,
        &swimmer_paula,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &27);

    level_up(conn, &battle, &team_member_magmortar, &22);

    let route_41_johto = create_location(conn, "Route 41", "Johto");

    let swimmer_george = create_trainer(conn, "George", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_george,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &23);

    let swimmer_charlie = create_trainer(conn, "Charlie", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_charlie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &27);

    let swimmer_kaylee = create_trainer(conn, "Kaylee", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_kaylee,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &24);

    let swimmer_matthew = create_trainer(conn, "Matthew", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_matthew,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_ronald = create_trainer(conn, "Ronald", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_ronald,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &28);

    let swimmer_denise = create_trainer(conn, "Denise", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_denise,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &25);

    let swimmer_berke = create_trainer(conn, "Berke", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_berke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_kara = create_trainer(conn, "Kara", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_kara,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_wendy = create_trainer(conn, "Wendy", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_wendy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_susie = create_trainer(conn, "Susie", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_41_johto,
        &swimmer_susie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let cianwood_city_johto = create_location(conn, "Cianwood City", "Johto");

    let mystery_man = create_trainer_class(conn, "Mystery Man");

    let mystery_man_eusine = create_trainer(conn, "Eusine", &mystery_man);

    let battle = create_battle(
        conn,
        &playthrough,
        &cianwood_city_johto,
        &mystery_man_eusine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &26);

    let black_belt_yoshi = create_trainer(conn, "Yoshi", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &cianwood_city_johto,
        &black_belt_yoshi,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &29);

    let black_belt_nob = create_trainer(conn, "Nob", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &cianwood_city_johto,
        &black_belt_nob,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_lao = create_trainer(conn, "Lao", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &cianwood_city_johto,
        &black_belt_lao,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_lung = create_trainer(conn, "Lung", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &cianwood_city_johto,
        &black_belt_lung,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &27);

    let leader_chuck = create_trainer(conn, "Chuck", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &cianwood_city_johto,
        &leader_chuck,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_47_johto = create_location(conn, "Route 47", "Johto");

    let hiker_devin = create_trainer(conn, "Devin", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_47_johto,
        &hiker_devin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &29);

    let double = create_trainer_class(conn, "Double");

    let double_team_thom_and_kae = create_trainer(conn, "Team Thom & Kae", &double);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_47_johto,
        &double_team_thom_and_kae,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &30);

    level_up(conn, &battle, &team_member_yanmega, &28);

    let camper_grant = create_trainer(conn, "Grant", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_47_johto,
        &camper_grant,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let young = create_trainer_class(conn, "Young");

    let young_couple_duff_and_eda = create_trainer(conn, "Couple Duff & Eda", &young);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_47_johto,
        &young_couple_duff_and_eda,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &28);

    let rocket_hideout_johto = create_location(conn, "Rocket Hideout", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &29);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist = create_trainer_class(conn, "Scientist");

    let scientist_gregg = create_trainer(conn, "Gregg", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &scientist_gregg,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &30);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &29);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &30);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_ross = create_trainer(conn, "Ross", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &scientist_ross,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &30);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_mitch = create_trainer(conn, "Mitch", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &scientist_mitch,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let executive_petrel = create_trainer(conn, "Petrel", &executive);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &executive_petrel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &31);

    let executive_ariana = create_trainer(conn, "Ariana", &executive);

    let team_rocket = create_trainer_class(conn, "Team Rocket");

    let team_rocket_grunt = create_trainer(conn, "Grunt", &team_rocket);

    let battle = create_battle(
        conn,
        &playthrough,
        &rocket_hideout_johto,
        &executive_ariana,
        Some(&team_rocket_grunt),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &31);

    let mahogany_town_johto = create_location(conn, "Mahogany Town", "Johto");

    let skier = create_trainer_class(conn, "Skier");

    let skier_diana = create_trainer(conn, "Diana", &skier);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &skier_diana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let boarder = create_trainer_class(conn, "Boarder");

    let boarder_patton = create_trainer(conn, "Patton", &boarder);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &boarder_patton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let boarder_deandre = create_trainer(conn, "Deandre", &boarder);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &boarder_deandre,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let boarder_gerardo = create_trainer(conn, "Gerardo", &boarder);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &boarder_gerardo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &32);

    let skier_jill = create_trainer(conn, "Jill", &skier);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &skier_jill,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_pryce = create_trainer(conn, "Pryce", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &leader_pryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &33);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &leader_pryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &31);

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &leader_pryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &leader_pryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &mahogany_town_johto,
        &leader_pryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &34);

    level_up(conn, &battle, &team_member_kingdra, &20);

    let leader_jasmine = create_trainer(conn, "Jasmine", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &olivine_city_johto,
        &leader_jasmine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &32);

    level_up(conn, &battle, &team_member_kingdra, &21);

    let battle = create_battle(
        conn,
        &playthrough,
        &olivine_city_johto,
        &leader_jasmine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &22);

    let radio_tower_johto = create_location(conn, "Radio Tower", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &23);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &31);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &24);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &32);

    let scientist_garett = create_trainer(conn, "Garett", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &scientist_garett,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &25);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_trenton = create_trainer(conn, "Trenton", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &scientist_trenton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &26);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &executive_petrel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &33);

    level_up(conn, &battle, &team_member_kingdra, &27);

    let goldenrod_underground_johto = create_location(conn, "Goldenrod Underground", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &rival_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &28);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let burglar = create_trainer_class(conn, "Burglar");

    let burglar_duncan = create_trainer(conn, "Duncan", &burglar);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &burglar_duncan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &32);

    level_up(conn, &battle, &team_member_kingdra, &29);

    let burglar_orson = create_trainer(conn, "Orson", &burglar);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &burglar_orson,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &30);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &34);

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &goldenrod_underground_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &33);

    level_up(conn, &battle, &team_member_kingdra, &31);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &executive_proton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &executive_ariana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let executive_archer = create_trainer(conn, "Archer", &executive);

    let battle = create_battle(
        conn,
        &playthrough,
        &radio_tower_johto,
        &executive_archer,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &33);

    level_up(conn, &battle, &team_member_kingdra, &32);

    let route_44_johto = create_location(conn, "Route 44", "Johto");

    let psychic_phil = create_trainer(conn, "Phil", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &psychic_phil,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_vance = create_trainer(conn, "Vance", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &bird_keeper_vance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_allen = create_trainer(conn, "Allen", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &ace_trainer_allen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &33);

    let fisherman_edgar = create_trainer(conn, "Edgar", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &fisherman_edgar,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &34);

    let ace_trainer_cybil = create_trainer(conn, "Cybil", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &ace_trainer_cybil,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let poké_maniac_zach = create_trainer(conn, "Zach", &poké_maniac);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &poké_maniac_zach,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_wilton = create_trainer(conn, "Wilton", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_44_johto,
        &fisherman_wilton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let blackthorn_city_johto = create_location(conn, "Blackthorn City", "Johto");

    let ace_trainer_paulo = create_trainer(conn, "Paulo", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &ace_trainer_paulo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &34);

    let ace_trainer_lola = create_trainer(conn, "Lola", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &ace_trainer_lola,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_cody = create_trainer(conn, "Cody", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &ace_trainer_cody,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &34);

    let ace_trainer_fran = create_trainer(conn, "Fran", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &ace_trainer_fran,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &35);

    let ace_trainer_mike = create_trainer(conn, "Mike", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &ace_trainer_mike,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_claire = create_trainer(conn, "Claire", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &35);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &35);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &36);

    level_up(conn, &battle, &team_member_tyranitar, &17);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &35);

    level_up(conn, &battle, &team_member_tyranitar, &18);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &19);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &20);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &21);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &36);

    level_up(conn, &battle, &team_member_tyranitar, &22);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &35);

    level_up(conn, &battle, &team_member_tyranitar, &23);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &24);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &36);

    level_up(conn, &battle, &team_member_tyranitar, &25);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_45_johto = create_location(conn, "Route 45", "Johto");

    let hiker_parry = create_trainer(conn, "Parry", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &hiker_parry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_46_johto = create_location(conn, "Route 46", "Johto");

    let camper_ted = create_trainer(conn, "Ted", &camper);

    let picnicker_erin = create_trainer(conn, "Erin", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_46_johto,
        &camper_ted,
        Some(&picnicker_erin),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &26);

    let hiker_erik = create_trainer(conn, "Erik", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &hiker_erik,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_kelly = create_trainer(conn, "Kelly", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &ace_trainer_kelly,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_kenji = create_trainer(conn, "Kenji", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &black_belt_kenji,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &36);

    let hiker_timothy = create_trainer(conn, "Timothy", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &hiker_timothy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &27);

    let hiker_bailey = create_trainer(conn, "Bailey", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_46_johto,
        &hiker_bailey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_ryan = create_trainer(conn, "Ryan", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &ace_trainer_ryan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_michael = create_trainer(conn, "Michael", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_45_johto,
        &hiker_michael,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &28);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &36);

    level_up(conn, &battle, &team_member_tyranitar, &29);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &37);

    level_up(conn, &battle, &team_member_tyranitar, &30);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &31);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &38);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &37);

    level_up(conn, &battle, &team_member_tyranitar, &32);

    let battle = create_battle(
        conn,
        &playthrough,
        &blackthorn_city_johto,
        &leader_claire,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &33);

    let dragons_den_johto = create_location(conn, "Dragon's Den", "Johto");

    let ace_trainer_kobe = create_trainer(conn, "Kobe", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &dragons_den_johto,
        &ace_trainer_kobe,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_piper = create_trainer(conn, "Piper", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &dragons_den_johto,
        &ace_trainer_piper,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let twins_clea_and_gil = create_trainer(conn, "Clea & Gil", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &dragons_den_johto,
        &twins_clea_and_gil,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &37);

    let kimono_girl = create_trainer_class(conn, "Kimono Girl");

    let kimono_girl_zuki = create_trainer(conn, "Zuki", &kimono_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let kimono_girl_naoko = create_trainer(conn, "Naoko", &kimono_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let kimono_girl_miki = create_trainer(conn, "Miki", &kimono_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &37);

    level_up(conn, &battle, &team_member_tyranitar, &34);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let kimono_girl_sayo = create_trainer(conn, "Sayo", &kimono_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &39);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &35);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &38);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &36);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &37);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let kimono_girl_kuni = create_trainer(conn, "Kuni", &kimono_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &37);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &38);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &39);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &39);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &40);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &38);

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_zuki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_miki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_sayo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &ecruteak_city_johto,
        &kimono_girl_kuni,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &38);

    let bell_tower_johto = create_location(conn, "Bell Tower", "Johto");

    let wild = create_trainer_class(conn, "Wild");

    let wild_ho_oh = create_trainer(conn, "Ho-Oh", &wild);

    let battle = create_battle(
        conn,
        &playthrough,
        &bell_tower_johto,
        &wild_ho_oh,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &39);

    let super_nerd = create_trainer_class(conn, "Super Nerd");

    let super_nerd_markus = create_trainer(conn, "Markus", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_mortar_johto,
        &super_nerd_markus,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let super_nerd_hugh = create_trainer(conn, "Hugh", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_mortar_johto,
        &super_nerd_hugh,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &38);

    let black_belt_kiyo = create_trainer(conn, "Kiyo", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_mortar_johto,
        &black_belt_kiyo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_27_johto = create_location(conn, "Route 27", "Johto");

    let ace_trainer_megan = create_trainer(conn, "Megan", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_27_johto,
        &ace_trainer_megan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &40);

    let ace_trainer_blake = create_trainer(conn, "Blake", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_27_johto,
        &ace_trainer_blake,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_26_johto = create_location(conn, "Route 26", "Johto");

    let fisherman_scott = create_trainer(conn, "Scott", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_26_johto,
        &fisherman_scott,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_eli = create_trainer(conn, "Eli", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_27_johto,
        &psychic_eli,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_reena = create_trainer(conn, "Reena", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_27_johto,
        &ace_trainer_reena,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &39);

    level_up(conn, &battle, &team_member_kingdra, &39);

    let ace_trainer_brian = create_trainer(conn, "Brian", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_27_johto,
        &ace_trainer_brian,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_jose = create_trainer(conn, "Jose", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_27_johto,
        &bird_keeper_jose,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_vernon = create_trainer(conn, "Vernon", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_26_johto,
        &psychic_vernon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_joyce = create_trainer(conn, "Joyce", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_26_johto,
        &ace_trainer_joyce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &41);

    let ace_trainer_gaven = create_trainer(conn, "Gaven", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_26_johto,
        &ace_trainer_gaven,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_jake = create_trainer(conn, "Jake", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_26_johto,
        &ace_trainer_jake,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &40);

    level_up(conn, &battle, &team_member_noctowl, &40);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_26_johto,
        &ace_trainer_jamie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &40);

    let victory_road_johto = create_location(conn, "Victory Road", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_johto,
        &rival_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &40);

    let pokémon_league_johto = create_location(conn, "Pokémon League", "Johto");

    let elite_four_will = create_trainer(conn, "Will", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &41);

    let elite_four_koga = create_trainer(conn, "Koga", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &43);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &41);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &41);

    level_up(conn, &battle, &team_member_noctowl, &41);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &42);

    level_up(conn, &battle, &team_member_yanmega, &41);

    level_up(conn, &battle, &team_member_magmortar, &42);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &42);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &42);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &42);

    level_up(conn, &battle, &team_member_magmortar, &43);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &43);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let elite_four_bruno = create_trainer(conn, "Bruno", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &43);

    level_up(conn, &battle, &team_member_noctowl, &43);

    level_up(conn, &battle, &team_member_tyranitar, &44);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &43);

    level_up(conn, &battle, &team_member_kingdra, &44);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &44);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &44);

    level_up(conn, &battle, &team_member_magmortar, &44);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &44);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &45);

    level_up(conn, &battle, &team_member_tyranitar, &45);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &45);

    level_up(conn, &battle, &team_member_magmortar, &45);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &45);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &45);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &46);

    level_up(conn, &battle, &team_member_tyranitar, &46);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &46);

    level_up(conn, &battle, &team_member_magmortar, &46);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &46);

    level_up(conn, &battle, &team_member_magmortar, &47);

    level_up(conn, &battle, &team_member_kingdra, &46);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &47);

    level_up(conn, &battle, &team_member_yanmega, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &48);

    level_up(conn, &battle, &team_member_tyranitar, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &48);

    level_up(conn, &battle, &team_member_yanmega, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &48);

    let elite_four_karen = create_trainer(conn, "Karen", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &49);

    level_up(conn, &battle, &team_member_tyranitar, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &49);

    level_up(conn, &battle, &team_member_tyranitar, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &50);

    let champion = create_trainer_class(conn, "Champion");

    let champion_lance = create_trainer(conn, "Lance", &champion);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &51);

    level_up(conn, &battle, &team_member_yanmega, &51);

    level_up(conn, &battle, &team_member_kingdra, &51);

    let ss_aqua_johto = create_location(conn, "S.S. Aqua", "Johto");

    let hiker_noland = create_trainer(conn, "Noland", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &hiker_noland,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let pokéfan_colin = create_trainer(conn, "Colin", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &pokéfan_colin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let twins_meg_and_peg = create_trainer(conn, "Meg & Peg", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &twins_meg_and_peg,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let firebreather_lyle = create_trainer(conn, "Lyle", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &firebreather_lyle,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &50);

    let sailor_jeff = create_trainer(conn, "Jeff", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &sailor_jeff,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let juggler_fritz = create_trainer(conn, "Fritz", &juggler);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &juggler_fritz,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &51);

    let sailor_stanley = create_trainer(conn, "Stanley", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &sailor_stanley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &51);

    let picnicker_debra = create_trainer(conn, "Debra", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &ss_aqua_johto,
        &picnicker_debra,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_6_johto = create_location(conn, "Route 6", "Johto");

    let picnicker_selina = create_trainer(conn, "Selina", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_johto,
        &picnicker_selina,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let twins_day_and_dani = create_trainer(conn, "Day & Dani", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_johto,
        &twins_day_and_dani,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let camper_virgil = create_trainer(conn, "Virgil", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_johto,
        &camper_virgil,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_8_johto = create_location(conn, "Route 8", "Johto");

    let biker = create_trainer_class(conn, "Biker");

    let biker_dwayne = create_trainer(conn, "Dwayne", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &biker_dwayne,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_harris = create_trainer(conn, "Harris", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &biker_harris,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &52);

    let biker_zeke = create_trainer(conn, "Zeke", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &biker_zeke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let super_nerd_sam = create_trainer(conn, "Sam", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &super_nerd_sam,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let young_couple_moe_and_lulu = create_trainer(conn, "Couple Moe & Lulu", &young);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &young_couple_moe_and_lulu,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let super_nerd_tyrone = create_trainer(conn, "Tyrone", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &super_nerd_tyrone,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let gentleman_milton = create_trainer(conn, "Milton", &gentleman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_johto,
        &gentleman_milton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &52);

    let route_12_johto = create_location(conn, "Route 12", "Johto");

    let fisherman_kyle = create_trainer(conn, "Kyle", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &fisherman_kyle,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_justin = create_trainer(conn, "Justin", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &bird_keeper_justin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &52);

    let fisherman_martin = create_trainer(conn, "Martin", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &fisherman_martin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let young_couple_vic_and_tara = create_trainer(conn, "Couple Vic & Tara", &young);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &young_couple_vic_and_tara,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let fisherman_stephen = create_trainer(conn, "Stephen", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &fisherman_stephen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &52);

    let bird_keeper_gail = create_trainer(conn, "Gail", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &bird_keeper_gail,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_barney = create_trainer(conn, "Barney", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &fisherman_barney,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_kyler = create_trainer(conn, "Kyler", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_johto,
        &fisherman_kyler,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_11_johto = create_location(conn, "Route 11", "Johto");

    let psychic_herman = create_trainer(conn, "Herman", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_johto,
        &psychic_herman,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &52);

    let youngster_jason = create_trainer(conn, "Jason", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_johto,
        &youngster_jason,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let youngster_owen = create_trainer(conn, "Owen", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_johto,
        &youngster_owen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_fidel = create_trainer(conn, "Fidel", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_johto,
        &psychic_fidel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_13_johto = create_location(conn, "Route 13", "Johto");

    let bird_keeper_bret = create_trainer(conn, "Bret", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &bird_keeper_bret,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &52);

    let picnicker_piper = create_trainer(conn, "Piper", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &picnicker_piper,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_perry = create_trainer(conn, "Perry", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &bird_keeper_perry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let young_couple_tim_and_sue = create_trainer(conn, "Couple Tim & Sue", &young);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &young_couple_tim_and_sue,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let pokéfan_joshua = create_trainer(conn, "Joshua", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &pokéfan_joshua,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper_tanner = create_trainer(conn, "Tanner", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &camper_tanner,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_alex = create_trainer(conn, "Alex", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &pokéfan_alex,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_ginger = create_trainer(conn, "Ginger", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &picnicker_ginger,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &53);

    let camper_clark = create_trainer(conn, "Clark", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &camper_clark,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_kenny = create_trainer(conn, "Kenny", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_johto,
        &hiker_kenny,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_17_johto = create_location(conn, "Route 17", "Johto");

    let biker_dale = create_trainer(conn, "Dale", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_dale,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_reese = create_trainer(conn, "Reese", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_reese,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_aiden = create_trainer(conn, "Aiden", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_aiden,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_dan = create_trainer(conn, "Dan", &biker);

    let biker_theron = create_trainer(conn, "Theron", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_dan,
        Some(&biker_theron),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &53);

    let biker_glenn = create_trainer(conn, "Glenn", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_glenn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_teddy = create_trainer(conn, "Teddy", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_teddy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_markey = create_trainer(conn, "Markey", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_markey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_charles = create_trainer(conn, "Charles", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_charles,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &53);

    let biker_joel = create_trainer(conn, "Joel", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_joel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let biker_jacob = create_trainer(conn, "Jacob", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_jacob,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &53);

    let biker_ernest = create_trainer(conn, "Ernest", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_johto,
        &biker_ernest,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_18_johto = create_location(conn, "Route 18", "Johto");

    let bird_keeper_bob = create_trainer(conn, "Bob", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_18_johto,
        &bird_keeper_bob,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_boris = create_trainer(conn, "Boris", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_18_johto,
        &bird_keeper_boris,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_15_johto = create_location(conn, "Route 15", "Johto");

    let school_kid_kipp = create_trainer(conn, "Kipp", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &school_kid_kipp,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &53);

    let school_kid_tommy = create_trainer(conn, "Tommy", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &school_kid_tommy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &53);

    let teacher = create_trainer_class(conn, "Teacher");

    let teacher_hillary = create_trainer(conn, "Hillary", &teacher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &teacher_hillary,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_boone = create_trainer(conn, "Boone", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &pokéfan_boone,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_johnny = create_trainer(conn, "Johnny", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &school_kid_johnny,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &54);

    let school_kid_billy = create_trainer(conn, "Billy", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &school_kid_billy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let teacher_colette = create_trainer(conn, "Colette", &teacher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &teacher_colette,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let twins_kay_and_tia = create_trainer(conn, "Kay & Tia", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &twins_kay_and_tia,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let teacher_eleanor = create_trainer(conn, "Eleanor", &teacher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_johto,
        &teacher_eleanor,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_14_johto = create_location(conn, "Route 14", "Johto");

    let bird_keeper_roy = create_trainer(conn, "Roy", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &bird_keeper_roy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_carter = create_trainer(conn, "Carter", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &pokéfan_carter,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_josh = create_trainer(conn, "Josh", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &bird_keeper_josh,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_trevor = create_trainer(conn, "Trevor", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &pokéfan_trevor,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_connor = create_trainer(conn, "Connor", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &school_kid_connor,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &54);

    let teacher_clarice = create_trainer(conn, "Clarice", &teacher);

    let school_kid_torrin = create_trainer(conn, "Torrin", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &teacher_clarice,
        Some(&school_kid_torrin),
        None,
        "Double",
        &0,
        &false,
    );

    let school_kid_travis = create_trainer(conn, "Travis", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_johto,
        &school_kid_travis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_25_johto = create_location(conn, "Route 25", "Johto");

    let school_kid_dudley = create_trainer(conn, "Dudley", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &school_kid_dudley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lass_ellen = create_trainer(conn, "Ellen", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &lass_ellen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_joe = create_trainer(conn, "Joe", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &school_kid_joe,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper_lloyd = create_trainer(conn, "Lloyd", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &camper_lloyd,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &54);

    let lass_laura = create_trainer(conn, "Laura", &lass);

    let lass_shannon = create_trainer(conn, "Shannon", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &lass_laura,
        Some(&lass_shannon),
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &54);

    let super_nerd_pat = create_trainer(conn, "Pat", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &super_nerd_pat,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_kevin = create_trainer(conn, "Kevin", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_25_johto,
        &ace_trainer_kevin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_9_johto = create_location(conn, "Route 9", "Johto");

    let picnicker_edna = create_trainer(conn, "Edna", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_johto,
        &picnicker_edna,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper_sid = create_trainer(conn, "Sid", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_johto,
        &camper_sid,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &54);

    let camper_dean = create_trainer(conn, "Dean", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_johto,
        &camper_dean,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_clarke = create_trainer(conn, "Clarke", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_johto,
        &hiker_clarke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &54);

    let hiker_eoin = create_trainer(conn, "Eoin", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_johto,
        &hiker_eoin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_heidi = create_trainer(conn, "Heidi", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_johto,
        &picnicker_heidi,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_10_johto = create_location(conn, "Route 10", "Johto");

    let hiker_jim = create_trainer(conn, "Jim", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_johto,
        &hiker_jim,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_robert = create_trainer(conn, "Robert", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_johto,
        &pokéfan_robert,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &55);

    let route_24_johto = create_location(conn, "Route 24", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &route_24_johto,
        &team_rocket_grunt_nan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_1_johto = create_location(conn, "Route 1", "Johto");

    let school_kid_sherman = create_trainer(conn, "Sherman", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_johto,
        &school_kid_sherman,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &55);

    let school_kid_danny = create_trainer(conn, "Danny", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_johto,
        &school_kid_danny,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_french = create_trainer(conn, "French", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_johto,
        &ace_trainer_french,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &55);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_johto,
        &ace_trainer_quinn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &55);

    let route_21_johto = create_location(conn, "Route 21", "Johto");

    let swimmer_nikki = create_trainer(conn, "Nikki", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &swimmer_nikki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_arnold = create_trainer(conn, "Arnold", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &fisherman_arnold,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_kinsley = create_trainer(conn, "Kinsley", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &bird_keeper_kinsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_murphy = create_trainer(conn, "Murphy", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &fisherman_murphy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &55);

    let swimmer_chelan = create_trainer(conn, "Chelan", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &swimmer_chelan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_liam = create_trainer(conn, "Liam", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &fisherman_liam,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_tyson = create_trainer(conn, "Tyson", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &swimmer_tyson,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_gideon = create_trainer(conn, "Gideon", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &fisherman_gideon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_duane = create_trainer(conn, "Duane", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &fisherman_duane,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &55);

    let swimmer_esteban = create_trainer(conn, "Esteban", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &swimmer_esteban,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_easton = create_trainer(conn, "Easton", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &bird_keeper_easton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_kendra = create_trainer(conn, "Kendra", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_21_johto,
        &swimmer_kendra,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &56);

    let route_20_johto = create_location(conn, "Route 20", "Johto");

    let bird_keeper_bert = create_trainer(conn, "Bert", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &bird_keeper_bert,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_cheyenne = create_trainer(conn, "Cheyenne", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &picnicker_cheyenne,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_frankie = create_trainer(conn, "Frankie", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &swimmer_frankie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_mina = create_trainer(conn, "Mina", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &swimmer_mina,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_leona = create_trainer(conn, "Leona", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &swimmer_leona,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_adrian = create_trainer(conn, "Adrian", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &picnicker_adrian,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let camper_pedro = create_trainer(conn, "Pedro", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &camper_pedro,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &56);

    let swimmer_elmo = create_trainer(conn, "Elmo", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &swimmer_elmo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_lori = create_trainer(conn, "Lori", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &swimmer_lori,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &56);

    let swimmer_nicole = create_trainer(conn, "Nicole", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_20_johto,
        &swimmer_nicole,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_19_johto = create_location(conn, "Route 19", "Johto");

    let swimmer_harold = create_trainer(conn, "Harold", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_19_johto,
        &swimmer_harold,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_debbie = create_trainer(conn, "Debbie", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_19_johto,
        &swimmer_debbie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_jerome = create_trainer(conn, "Jerome", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_19_johto,
        &swimmer_jerome,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &56);

    let swimmer_tucker = create_trainer(conn, "Tucker", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_19_johto,
        &swimmer_tucker,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_2_johto = create_location(conn, "Route 2", "Johto");

    let bug_catcher_rob = create_trainer(conn, "Rob", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_johto,
        &bug_catcher_rob,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_doug = create_trainer(conn, "Doug", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_johto,
        &bug_catcher_doug,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &56);

    let viridian_forest_johto = create_location(conn, "Viridian Forest", "Johto");

    let bug_catcher_dane = create_trainer(conn, "Dane", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_forest_johto,
        &bug_catcher_dane,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_dion = create_trainer(conn, "Dion", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_forest_johto,
        &bug_catcher_dion,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_stacey = create_trainer(conn, "Stacey", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_forest_johto,
        &bug_catcher_stacey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &56);

    let bug_catcher_ellis = create_trainer(conn, "Ellis", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_forest_johto,
        &bug_catcher_ellis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_abner = create_trainer(conn, "Abner", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_forest_johto,
        &bug_catcher_abner,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bug_catcher_ed = create_trainer(conn, "Ed", &bug_catcher);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_johto,
        &bug_catcher_ed,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &57);

    let route_3_johto = create_location(conn, "Route 3", "Johto");

    let youngster_regis = create_trainer(conn, "Regis", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &youngster_regis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let double_team_zac_and_jen = create_trainer(conn, "Team Zac & Jen", &double);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &double_team_zac_and_jen,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let youngster_warren = create_trainer(conn, "Warren", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &youngster_warren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let youngster_jimmy = create_trainer(conn, "Jimmy", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &youngster_jimmy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let firebreather_otis = create_trainer(conn, "Otis", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &firebreather_otis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &57);

    let hiker_bruce = create_trainer(conn, "Bruce", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &hiker_bruce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_manford = create_trainer(conn, "Manford", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &black_belt_manford,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_ander = create_trainer(conn, "Ander", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &black_belt_ander,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_dwight = create_trainer(conn, "Dwight", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &hiker_dwight,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &57);

    let firebreather_burt = create_trainer(conn, "Burt", &firebreather);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_johto,
        &firebreather_burt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let mt_moon_johto = create_location(conn, "Mt. Moon", "Johto");

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_moon_johto,
        &rival_theo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &58);

    let route_4_johto = create_location(conn, "Route 4", "Johto");

    let picnicker_hope = create_trainer(conn, "Hope", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_johto,
        &picnicker_hope,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let bird_keeper_hank = create_trainer(conn, "Hank", &bird_keeper);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_johto,
        &bird_keeper_hank,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_sharon = create_trainer(conn, "Sharon", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_johto,
        &picnicker_sharon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &57);

    let vermillion_city_johto = create_location(conn, "Vermillion City", "Johto");

    let guitarist = create_trainer_class(conn, "Guitarist");

    let guitarist_vincent = create_trainer(conn, "Vincent", &guitarist);

    let juggler_horton = create_trainer(conn, "Horton", &juggler);

    let battle = create_battle(
        conn,
        &playthrough,
        &vermillion_city_johto,
        &guitarist_vincent,
        Some(&juggler_horton),
        None,
        "Double",
        &0,
        &false,
    );

    let gentleman_gregory = create_trainer(conn, "Gregory", &gentleman);

    let battle = create_battle(
        conn,
        &playthrough,
        &vermillion_city_johto,
        &gentleman_gregory,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_lt = create_trainer_class(conn, "Leader Lt.");

    let leader_lt_surge = create_trainer(conn, "Surge", &leader_lt);

    let battle = create_battle(
        conn,
        &playthrough,
        &vermillion_city_johto,
        &leader_lt_surge,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &57);

    let pewter_city_johto = create_location(conn, "Pewter City", "Johto");

    let camper_jerry = create_trainer(conn, "Jerry", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &pewter_city_johto,
        &camper_jerry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &57);

    let hiker_edwin = create_trainer(conn, "Edwin", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &pewter_city_johto,
        &hiker_edwin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_brock = create_trainer(conn, "Brock", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &pewter_city_johto,
        &leader_brock,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &58);

    let cerulean_city_johto = create_location(conn, "Cerulean City", "Johto");

    let sailor_parker = create_trainer(conn, "Parker", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &cerulean_city_johto,
        &sailor_parker,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let sailor_eddie = create_trainer(conn, "Eddie", &sailor);

    let battle = create_battle(
        conn,
        &playthrough,
        &cerulean_city_johto,
        &sailor_eddie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_diana = create_trainer(conn, "Diana", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &cerulean_city_johto,
        &swimmer_diana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_joy = create_trainer(conn, "Joy", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &cerulean_city_johto,
        &swimmer_joy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_briana = create_trainer(conn, "Briana", &swimmer);

    let battle = create_battle(
        conn,
        &playthrough,
        &cerulean_city_johto,
        &swimmer_briana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &58);

    let leader_misty = create_trainer(conn, "Misty", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &cerulean_city_johto,
        &leader_misty,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let celadon_city_johto = create_location(conn, "Celadon City", "Johto");

    let twins_jo_and_zoe = create_trainer(conn, "Jo & Zoe", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &celadon_city_johto,
        &twins_jo_and_zoe,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let lass_michelle = create_trainer(conn, "Michelle", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &celadon_city_johto,
        &lass_michelle,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let picnicker_tanya = create_trainer(conn, "Tanya", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &celadon_city_johto,
        &picnicker_tanya,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let beauty_julia = create_trainer(conn, "Julia", &beauty);

    let battle = create_battle(
        conn,
        &playthrough,
        &celadon_city_johto,
        &beauty_julia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &58);

    let leader_erika = create_trainer(conn, "Erika", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &celadon_city_johto,
        &leader_erika,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let saffron_city_johto = create_location(conn, "Saffron City", "Johto");

    let medium_rebecca = create_trainer(conn, "Rebecca", &medium);

    let battle = create_battle(
        conn,
        &playthrough,
        &saffron_city_johto,
        &medium_rebecca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_franklin = create_trainer(conn, "Franklin", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &saffron_city_johto,
        &psychic_franklin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let medium_darcy = create_trainer(conn, "Darcy", &medium);

    let battle = create_battle(
        conn,
        &playthrough,
        &saffron_city_johto,
        &medium_darcy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &58);

    let psychic_jared = create_trainer(conn, "Jared", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &saffron_city_johto,
        &psychic_jared,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_sabrina = create_trainer(conn, "Sabrina", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &saffron_city_johto,
        &leader_sabrina,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fuschia_city_johto = create_location(conn, "Fuschia City", "Johto");

    let picnicker_cindy = create_trainer(conn, "Cindy", &picnicker);

    let battle = create_battle(
        conn,
        &playthrough,
        &fuschia_city_johto,
        &picnicker_cindy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &58);

    let camper_barry = create_trainer(conn, "Barry", &camper);

    let battle = create_battle(
        conn,
        &playthrough,
        &fuschia_city_johto,
        &camper_barry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lass_linda = create_trainer(conn, "Linda", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &fuschia_city_johto,
        &lass_linda,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_janine = create_trainer(conn, "Janine", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &fuschia_city_johto,
        &leader_janine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &59);

    let seafoam_islands_johto = create_location(conn, "Seafoam Islands", "Johto");

    let scientist_lowell = create_trainer(conn, "Lowell", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &scientist_lowell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let super_nerd_cary = create_trainer(conn, "Cary", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &super_nerd_cary,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_linden = create_trainer(conn, "Linden", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &scientist_linden,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let super_nerd_waldo = create_trainer(conn, "Waldo", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &super_nerd_waldo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &59);

    let super_nerd_merle = create_trainer(conn, "Merle", &super_nerd);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &super_nerd_merle,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_daniel = create_trainer(conn, "Daniel", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &scientist_daniel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_blane = create_trainer(conn, "Blane", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &leader_blane,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let viridian_city_johto = create_location(conn, "Viridian City", "Johto");

    let ace_trainer_salma = create_trainer(conn, "Salma", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_city_johto,
        &ace_trainer_salma,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &59);

    let ace_trainer_arabella = create_trainer(conn, "Arabella", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_city_johto,
        &ace_trainer_arabella,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let double_team_elan_and_ida = create_trainer(conn, "Team Elan & Ida", &double);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_city_johto,
        &double_team_elan_and_ida,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &59);

    level_up(conn, &battle, &team_member_yanmega, &59);

    let ace_trainer_bonita = create_trainer(conn, "Bonita", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_city_johto,
        &ace_trainer_bonita,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &60);

    let leader_blue = create_trainer(conn, "Blue", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &viridian_city_johto,
        &leader_blue,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &59);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &60);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &60);

    level_up(conn, &battle, &team_member_magmortar, &60);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &60);

    level_up(conn, &battle, &team_member_noctowl, &60);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &61);

    level_up(conn, &battle, &team_member_magmortar, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &61);

    level_up(conn, &battle, &team_member_tyranitar, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &61);

    level_up(conn, &battle, &team_member_tyranitar, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &62);

    level_up(conn, &battle, &team_member_yanmega, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &63);

    level_up(conn, &battle, &team_member_tyranitar, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &65);

    level_up(conn, &battle, &team_member_tyranitar, &65);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &64);

    level_up(conn, &battle, &team_member_magmortar, &65);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &65);

    level_up(conn, &battle, &team_member_tyranitar, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &65);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &65);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &66);

    level_up(conn, &battle, &team_member_tyranitar, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &67);

    level_up(conn, &battle, &team_member_yanmega, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &66);

    level_up(conn, &battle, &team_member_kingdra, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &70);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &69);

    level_up(conn, &battle, &team_member_yanmega, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &70);

    level_up(conn, &battle, &team_member_kingdra, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_magmortar, &71);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &70);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &70);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &70);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &71);

    level_up(conn, &battle, &team_member_yanmega, &71);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &70);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &72);

    level_up(conn, &battle, &team_member_tyranitar, &71);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &71);

    level_up(conn, &battle, &team_member_magmortar, &72);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &72);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &72);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &71);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_yanmega, &73);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &72);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_will,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_koga,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &72);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_bruno,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &elite_four_karen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_noctowl, &73);

    level_up(conn, &battle, &team_member_yanmega, &74);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_johto,
        &champion_lance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tyranitar, &73);

    let mt_silver_cave_johto = create_location(conn, "Mt. Silver Cave", "Johto");

    let pkmn_trainer = create_trainer_class(conn, "Pkmn Trainer");

    let pkmn_trainer_red = create_trainer(conn, "Red", &pkmn_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_silver_cave_johto,
        &pkmn_trainer_red,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &73);

    level_up(conn, &battle, &team_member_noctowl, &74);

    let battle = create_battle(
        conn,
        &playthrough,
        &mt_silver_cave_johto,
        &pkmn_trainer_red,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let skier_cady = create_trainer(conn, "Cady", &skier);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &skier_cady,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let boarder_shaun = create_trainer(conn, "Shaun", &boarder);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &boarder_shaun,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_kingdra, &73);

    let boarder_bryce = create_trainer(conn, "Bryce", &boarder);

    let battle = create_battle(
        conn,
        &playthrough,
        &seafoam_islands_johto,
        &boarder_bryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_raichu, &74);

    let playthrough = create_playthrough(conn, "26852", "Ben", "Black", "2023-06-24");

    let nuvema_town_unova = create_location(conn, "Nuvema Town", "Unova");

    let species_lillipup = create_species(conn, "Lillipup", &506, &5, "Normal", None);

    let team_member_lillipup = catch_pokemon(
        conn,
        &playthrough,
        &1,
        &species_lillipup,
        None,
        "Gift",
        "2023-06-24",
        &nuvema_town_unova,
        &5,
        "M",
        "Poké Ball",
    );

    let pkmn_trainer = create_trainer_class(conn, "PKMN Trainer");

    let pkmn_trainer_bianca = create_trainer(conn, "Bianca", &pkmn_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &nuvema_town_unova,
        &pkmn_trainer_bianca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_trainer_cheren = create_trainer(conn, "Cheren", &pkmn_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &nuvema_town_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let accumula_town_unova = create_location(conn, "Accumula Town", "Unova");

    let pkmn_trainer_n = create_trainer(conn, "N", &pkmn_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &accumula_town_unova,
        &pkmn_trainer_n,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &7);

    let route_2_unova = create_location(conn, "Route 2", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_unova,
        &youngster_jimmy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &8);

    let lass_anna = create_trainer(conn, "Anna", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_unova,
        &lass_anna,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &9);

    let youngster_roland = create_trainer(conn, "Roland", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_unova,
        &youngster_roland,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &10);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_2_unova,
        &pkmn_trainer_bianca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dreamyard_unova = create_location(conn, "Dreamyard", "Unova");

    let lass_eri = create_trainer(conn, "Eri", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &dreamyard_unova,
        &lass_eri,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &11);

    let battle = create_battle(
        conn,
        &playthrough,
        &dreamyard_unova,
        &youngster_joey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &12);

    let species_pansear = create_species(conn, "Pansear", &513, &5, "Fire", None);

    let team_member_pansear = catch_pokemon(
        conn,
        &playthrough,
        &2,
        &species_pansear,
        None,
        "Gift",
        "2023-06-25",
        &dreamyard_unova,
        &10,
        "M",
        "Poké Ball",
    );

    let striaton_city_unova = create_location(conn, "Striaton City", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &striaton_city_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let waiter = create_trainer_class(conn, "Waiter");

    let waiter_maxwell = create_trainer(conn, "Maxwell", &waiter);

    let battle = create_battle(
        conn,
        &playthrough,
        &striaton_city_unova,
        &waiter_maxwell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &11);

    let waitress = create_trainer_class(conn, "Waitress");

    let waitress_tia = create_trainer(conn, "Tia", &waitress);

    let battle = create_battle(
        conn,
        &playthrough,
        &striaton_city_unova,
        &waitress_tia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_cilan = create_trainer(conn, "Cilan", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &striaton_city_unova,
        &leader_cilan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &13);

    level_up(conn, &battle, &team_member_pansear, &12);

    let team_plasma = create_trainer_class(conn, "Team Plasma");

    let team_plasma_grunt = create_trainer(conn, "Grunt", &team_plasma);

    let battle = create_battle(
        conn,
        &playthrough,
        &dreamyard_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dreamyard_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &14);

    let route_3_unova = create_location(conn, "Route 3", "Unova");

    let nursery_aid = create_trainer_class(conn, "Nursery Aid");

    let nursery_aid_autumn = create_trainer(conn, "Autumn", &nursery_aid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &nursery_aid_autumn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let preschooler = create_trainer_class(conn, "Preschooler");

    let preschooler_doyle = create_trainer(conn, "Doyle", &preschooler);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &preschooler_doyle,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &13);

    let preschooler_wendy = create_trainer(conn, "Wendy", &preschooler);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &preschooler_wendy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let preschooler_tully = create_trainer(conn, "Tully", &preschooler);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &preschooler_tully,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let twins_kumi_and_amy = create_trainer(conn, "Kumi & Amy", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &twins_kumi_and_amy,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &14);

    let pkmn_breeder = create_trainer_class(conn, "PKMN Breeder");

    let pkmn_breeder_adelaide = create_trainer(conn, "Adelaide", &pkmn_breeder);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &pkmn_breeder_adelaide,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &15);

    let wellspring_cave_unova = create_location(conn, "Wellspring Cave", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &wellspring_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &wellspring_cave_unova,
        &team_plasma_grunt,
        Some(&team_plasma_grunt),
        Some(&pkmn_trainer_cheren),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &15);

    let school_kid_al = create_trainer(conn, "Al", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &school_kid_al,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_marsha = create_trainer(conn, "Marsha", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &school_kid_marsha,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_gina = create_trainer(conn, "Gina", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &school_kid_gina,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &16);

    let school_kid_edgar = create_trainer(conn, "Edgar", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_3_unova,
        &school_kid_edgar,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pinwheel_forest_unova = create_location(conn, "Pinwheel Forest", "Unova");

    let nurse = create_trainer_class(conn, "Nurse");

    let nurse_shery = create_trainer(conn, "Shery", &nurse);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &nurse_shery,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_lillipup, &16);

    let species_herdier = create_species(conn, "Herdier", &507, &5, "Normal", None);

    evolve(conn, &battle, &team_member_lillipup, &species_herdier);

    let team_member_herdier = team_member_lillipup;

    let nacrene_city_unova = create_location(conn, "Nacrene City", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &nacrene_city_unova,
        &pkmn_trainer_n,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let preschooler_juliet = create_trainer(conn, "Juliet", &preschooler);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &preschooler_juliet,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &17);

    let wild_tympole = create_trainer(conn, "Tympole", &wild);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &wild_tympole,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let species_tympole = create_species(conn, "Tympole", &535, &5, "Water", None);

    let team_member_tympole = catch_pokemon(
        conn,
        &playthrough,
        &3,
        &species_tympole,
        None,
        "Grass",
        "2023-06-26",
        &pinwheel_forest_unova,
        &12,
        "M",
        "Net Ball",
    );

    let preschooler_homer = create_trainer(conn, "Homer", &preschooler);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &preschooler_homer,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let youngster_keita = create_trainer(conn, "Keita", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &youngster_keita,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &18);

    let youngster_zachary = create_trainer(conn, "Zachary", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &youngster_zachary,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle_girl = create_trainer_class(conn, "Battle Girl");

    let battle_girl_lee = create_trainer(conn, "Lee", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &battle_girl_lee,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &13);

    let black_belt_kentaro = create_trainer(conn, "Kentaro", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &black_belt_kentaro,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_carter = create_trainer(conn, "Carter", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &nacrene_city_unova,
        &school_kid_carter,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &14);

    let scientist_satomi = create_trainer(conn, "Satomi", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &nacrene_city_unova,
        &scientist_satomi,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &15);

    let school_kid_lydia = create_trainer(conn, "Lydia", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &nacrene_city_unova,
        &school_kid_lydia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &17);

    level_up(conn, &battle, &team_member_herdier, &19);

    let leader_lenora = create_trainer(conn, "Lenora", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &nacrene_city_unova,
        &leader_lenora,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &20);

    let twins_mayo_and_may = create_trainer(conn, "Mayo & May", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &twins_mayo_and_may,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &16);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &17);

    let pkmn_ranger_forrest = create_trainer(conn, "Forrest", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &pkmn_ranger_forrest,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &21);

    let youngster_nicholas = create_trainer(conn, "Nicholas", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &youngster_nicholas,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &18);

    let pkmn_ranger_audra = create_trainer(conn, "Audra", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &pkmn_ranger_audra,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_irene = create_trainer(conn, "Irene", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &pkmn_ranger_irene,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &18);

    let pkmn_ranger_miguel = create_trainer(conn, "Miguel", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &pkmn_ranger_miguel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lass_eva = create_trainer(conn, "Eva", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &lass_eva,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &22);

    let school_kid_sammy = create_trainer(conn, "Sammy", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &school_kid_sammy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_millie = create_trainer(conn, "Millie", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &pinwheel_forest_unova,
        &school_kid_millie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let castelia_city_unova = create_location(conn, "Castelia City", "Unova");

    let clerk_f = create_trainer_class(conn, "Clerk F");

    let clerk_f_ingrid = create_trainer(conn, "Ingrid", &clerk_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &clerk_f_ingrid,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &19);

    let clerk_m = create_trainer_class(conn, "Clerk M");

    let clerk_m_clemens = create_trainer(conn, "Clemens", &clerk_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &clerk_m_clemens,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &19);

    let clerk_f_alberta = create_trainer(conn, "Alberta", &clerk_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &clerk_f_alberta,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_randall = create_trainer(conn, "Randall", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &scientist_randall,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let clerk_m_warren = create_trainer(conn, "Warren", &clerk_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &clerk_m_warren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &20);

    level_up(conn, &battle, &team_member_herdier, &23);

    let clerk_m_ivan = create_trainer(conn, "Ivan", &clerk_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &clerk_m_ivan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_samantha = create_trainer(conn, "Samantha", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &scientist_samantha,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_steve = create_trainer(conn, "Steve", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &scientist_steve,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &20);

    let clerk_m_wade = create_trainer(conn, "Wade", &clerk_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &clerk_m_wade,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let janitor = create_trainer_class(conn, "Janitor");

    let janitor_geoff = create_trainer(conn, "Geoff", &janitor);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &janitor_geoff,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &21);

    level_up(conn, &battle, &team_member_tympole, &21);

    let dancer = create_trainer_class(conn, "Dancer");

    let dancer_mickey = create_trainer(conn, "Mickey", &dancer);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &dancer_mickey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dancer_edmond = create_trainer(conn, "Edmond", &dancer);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &dancer_edmond,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_4_unova = create_location(conn, "Route 4", "Unova");

    let fisherman_hubert = create_trainer(conn, "Hubert", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &fisherman_hubert,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &22);

    let fisherman_andrew = create_trainer(conn, "Andrew", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &fisherman_andrew,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &22);

    level_up(conn, &battle, &team_member_herdier, &24);

    let parasol_lady = create_trainer_class(conn, "Parasol Lady");

    let parasol_lady_april = create_trainer(conn, "April", &parasol_lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &parasol_lady_april,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &23);

    let worker = create_trainer_class(conn, "Worker");

    let worker_gus = create_trainer(conn, "Gus", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &worker_gus,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_shelby = create_trainer(conn, "Shelby", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &worker_shelby,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dancer_raymond = create_trainer(conn, "Raymond", &dancer);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &dancer_raymond,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let harlequin = create_trainer_class(conn, "Harlequin");

    let harlequin_jack = create_trainer(conn, "Jack", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &harlequin_jack,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &23);

    let harlequin_kerry = create_trainer(conn, "Kerry", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &harlequin_kerry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let harlequin_rick = create_trainer(conn, "Rick", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &harlequin_rick,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let harlequin_louis = create_trainer(conn, "Louis", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &harlequin_louis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &24);

    let leader_burgh = create_trainer(conn, "Burgh", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_city_unova,
        &leader_burgh,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &24);

    level_up(conn, &battle, &team_member_pansear, &25);

    let castelia_gate_unova = create_location(conn, "Castelia Gate", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &castelia_gate_unova,
        &pkmn_trainer_bianca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &25);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &26);

    let backpacker = create_trainer_class(conn, "Backpacker");

    let backpacker_keane = create_trainer(conn, "Keane", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &backpacker_keane,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_tympole, &25);

    let species_palpitoad = create_species(conn, "Palpitoad", &536, &5, "Water", Some("Ground"));

    evolve(conn, &battle, &team_member_tympole, &species_palpitoad);

    let team_member_palpitoad = team_member_tympole;

    let backpacker_anna = create_trainer(conn, "Anna", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &backpacker_anna,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_jill = create_trainer(conn, "Jill", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &backpacker_jill,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_waylon = create_trainer(conn, "Waylon", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &backpacker_waylon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_scott = create_trainer(conn, "Scott", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &worker_scott,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_zack = create_trainer(conn, "Zack", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &worker_zack,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let desert_resort_unova = create_location(conn, "Desert Resort", "Unova");

    let doctor = create_trainer_class(conn, "Doctor");

    let doctor_jerry = create_trainer(conn, "Jerry", &doctor);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &doctor_jerry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_kelsey = create_trainer(conn, "Kelsey", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &backpacker_kelsey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &26);

    let backpacker_liz = create_trainer(conn, "Liz", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &backpacker_liz,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_cybil = create_trainer(conn, "Cybil", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &psychic_cybil,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &26);

    let backpacker_nate = create_trainer(conn, "Nate", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &backpacker_nate,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_elaine = create_trainer(conn, "Elaine", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &backpacker_elaine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_gaven = create_trainer(conn, "Gaven", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &psychic_gaven,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &27);

    let psychic_low = create_trainer(conn, "Low", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &psychic_low,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_mylene = create_trainer(conn, "Mylene", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &pkmn_ranger_mylene,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_jaden = create_trainer(conn, "Jaden", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &desert_resort_unova,
        &pkmn_ranger_jaden,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &27);

    let relic_castle_unova = create_location(conn, "Relic Castle", "Unova");

    let psychic_perry = create_trainer(conn, "Perry", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &psychic_perry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_dua = create_trainer(conn, "Dua", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &psychic_dua,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &27);

    let backpacker_jerome = create_trainer(conn, "Jerome", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_4_unova,
        &backpacker_jerome,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let nimbasa_city_unova = create_location(conn, "Nimbasa City", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let linebacker = create_trainer_class(conn, "Linebacker");

    let linebacker_dan = create_trainer(conn, "Dan", &linebacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &linebacker_dan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hoopster = create_trainer_class(conn, "Hoopster");

    let hoopster_bobby = create_trainer(conn, "Bobby", &hoopster);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &hoopster_bobby,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &28);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &pkmn_trainer_n,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &28);

    let route_16_unova = create_location(conn, "Route 16", "Unova");

    let policeman_daniel = create_trainer(conn, "Daniel", &policeman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_16_unova,
        &policeman_daniel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let cyclist = create_trainer_class(conn, "Cyclist");

    let cyclist_krissa = create_trainer(conn, "Krissa", &cyclist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_16_unova,
        &cyclist_krissa,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &28);

    let backpacker_peter = create_trainer(conn, "Peter", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_16_unova,
        &backpacker_peter,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let cyclist_hector = create_trainer(conn, "Hector", &cyclist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_16_unova,
        &cyclist_hector,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_stephen = create_trainer(conn, "Stephen", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_16_unova,
        &backpacker_stephen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &29);

    let route_5_unova = create_location(conn, "Route 5", "Unova");

    let backpacker_lois = create_trainer(conn, "Lois", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &backpacker_lois,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_michael = create_trainer(conn, "Michael", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &backpacker_michael,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let baker = create_trainer_class(conn, "Baker");

    let baker_jenn = create_trainer(conn, "Jenn", &baker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &baker_jenn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let harlequin_paul = create_trainer(conn, "Paul", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &harlequin_paul,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &29);

    let musician = create_trainer_class(conn, "Musician");

    let musician_preston = create_trainer(conn, "Preston", &musician);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &musician_preston,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let dancer_brian = create_trainer(conn, "Brian", &dancer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &dancer_brian,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &29);

    let artist = create_trainer_class(conn, "Artist");

    let artist_horton = create_trainer(conn, "Horton", &artist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &artist_horton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lady = create_trainer_class(conn, "Lady");

    let lady_magnolia = create_trainer(conn, "Magnolia", &lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &lady_magnolia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let rich_boy = create_trainer_class(conn, "Rich Boy");

    let rich_boy_cody = create_trainer(conn, "Cody", &rich_boy);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &rich_boy_cody,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &30);

    let rich_boy_rolan = create_trainer(conn, "Rolan", &rich_boy);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &rich_boy_rolan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lady_colette = create_trainer(conn, "Colette", &lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &lady_colette,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_elesa = create_trainer(conn, "Elesa", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &nimbasa_city_unova,
        &leader_elesa,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &31);

    let species_archen = create_species(conn, "Archen", &566, &5, "Rock", Some("Flying"));

    let team_member_archen = catch_pokemon(
        conn,
        &playthrough,
        &4,
        &species_archen,
        None,
        "Fossil",
        "2023-07-02",
        &nacrene_city_unova,
        &25,
        "M",
        "Poké Ball",
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &30);

    level_up(conn, &battle, &team_member_palpitoad, &32);

    let preschooler_sarah = create_trainer(conn, "Sarah", &preschooler);

    let preschooler_billy = create_trainer(conn, "Billy", &preschooler);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_5_unova,
        &preschooler_sarah,
        Some(&preschooler_billy),
        Some(&pkmn_trainer_cheren),
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &26);

    let cold_storage_unova = create_location(conn, "Cold Storage", "Unova");

    let youngster_kenneth = create_trainer(conn, "Kenneth", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &youngster_kenneth,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &youngster_albert,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_eddie = create_trainer(conn, "Eddie", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &worker_eddie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &27);

    let worker_victor = create_trainer(conn, "Victor", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &worker_victor,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &30);

    let worker_glenn = create_trainer(conn, "Glenn", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &worker_glenn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_filipe = create_trainer(conn, "Filipe", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &worker_filipe,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_patton = create_trainer(conn, "Patton", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &worker_patton,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_ryan = create_trainer(conn, "Ryan", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &worker_ryan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &28);

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &cold_storage_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_6_unova = create_location(conn, "Route 6", "Unova");

    let scientist_william = create_trainer(conn, "William", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &scientist_william,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_shanti = create_trainer(conn, "Shanti", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &pkmn_ranger_shanti,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &31);

    level_up(conn, &battle, &team_member_archen, &29);

    let parasol_lady_nicole = create_trainer(conn, "Nicole", &parasol_lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &parasol_lady_nicole,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_ron = create_trainer(conn, "Ron", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &scientist_ron,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_maria = create_trainer(conn, "Maria", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &scientist_maria,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let parasol_lady_tihana = create_trainer(conn, "Tihana", &parasol_lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &parasol_lady_tihana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_richard = create_trainer(conn, "Richard", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_6_unova,
        &pkmn_ranger_richard,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &31);

    level_up(conn, &battle, &team_member_archen, &30);

    let driftveil_city_unova = create_location(conn, "Driftveil City", "Unova");

    let worker_felix = create_trainer(conn, "Felix", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &worker_felix,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_sterling = create_trainer(conn, "Sterling", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &worker_sterling,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_don = create_trainer(conn, "Don", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &worker_don,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let clerk_m_isaac = create_trainer(conn, "Isaac", &clerk_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &clerk_m_isaac,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &31);

    level_up(conn, &battle, &team_member_palpitoad, &33);

    let clerk_f_katie = create_trainer(conn, "Katie", &clerk_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &clerk_f_katie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_clay = create_trainer(conn, "Clay", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &leader_clay,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &driftveil_city_unova,
        &pkmn_trainer_bianca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &32);

    let chargestone_cave_unova = create_location(conn, "Chargestone Cave", "Unova");

    let ace_trainer_jared = create_trainer(conn, "Jared", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &ace_trainer_jared,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let scientist_ronald = create_trainer(conn, "Ronald", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &scientist_ronald,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_hardy = create_trainer(conn, "Hardy", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &hiker_hardy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &32);

    let scientist_naoko = create_trainer(conn, "Naoko", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &scientist_naoko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let doctor_wayne = create_trainer(conn, "Wayne", &doctor);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &doctor_wayne,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &33);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_herdier, &32);

    let species_stoutland = create_species(conn, "Stoutland", &508, &5, "Normal", None);

    evolve(conn, &battle, &team_member_herdier, &species_stoutland);

    let team_member_stoutland = team_member_herdier;

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &33);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &34);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_allison = create_trainer(conn, "Allison", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &ace_trainer_allison,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &34);

    let ace_trainer_stella = create_trainer(conn, "Stella", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &ace_trainer_stella,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &33);

    level_up(conn, &battle, &team_member_stoutland, &34);

    let scientist_orville = create_trainer(conn, "Orville", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &scientist_orville,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_corky = create_trainer(conn, "Corky", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &ace_trainer_corky,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &34);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &pkmn_trainer_n,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let mistralton_city_unova = create_location(conn, "Mistralton City", "Unova");

    let route_7_unova = create_location(conn, "Route 7", "Unova");

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &youngster_mikey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &35);

    let youngster_parker = create_trainer(conn, "Parker", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &youngster_parker,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_terrance = create_trainer(conn, "Terrance", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &backpacker_terrance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_elmer = create_trainer(conn, "Elmer", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &ace_trainer_elmer,
        None,
        None,
        "Rotation",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &35);

    level_up(conn, &battle, &team_member_stoutland, &35);

    level_up(conn, &battle, &team_member_palpitoad, &35);

    let backpacker_ruth = create_trainer(conn, "Ruth", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &backpacker_ruth,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_mary = create_trainer(conn, "Mary", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &pkmn_ranger_mary,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &36);

    let pkmn_ranger_pedro = create_trainer(conn, "Pedro", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &pkmn_ranger_pedro,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let harlequin_pat = create_trainer(conn, "Pat", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &harlequin_pat,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &36);

    let harlequin_ian = create_trainer(conn, "Ian", &harlequin);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_7_unova,
        &harlequin_ian,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &36);

    let twist_mountain_unova = create_location(conn, "Twist Mountain", "Unova");

    let hiker_terrell = create_trainer(conn, "Terrell", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &hiker_terrell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_palpitoad, &36);

    let species_seismitoad = create_species(conn, "Seismitoad", &537, &5, "Water", Some("Ground"));

    evolve(conn, &battle, &team_member_palpitoad, &species_seismitoad);

    let team_member_seismitoad = team_member_palpitoad;

    let celestial_tower_unova = create_location(conn, "Celestial Tower", "Unova");

    let psychic_doreen = create_trainer(conn, "Doreen", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &psychic_doreen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &37);

    let lass_kara = create_trainer(conn, "Kara", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &lass_kara,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_jude = create_trainer(conn, "Jude", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &pokéfan_jude,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pokéfan_georgia = create_trainer(conn, "Georgia", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &pokéfan_georgia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_belle = create_trainer(conn, "Belle", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &psychic_belle,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archen, &37);

    let species_archeops = create_species(conn, "Archeops", &567, &5, "Rock", Some("Flying"));

    evolve(conn, &battle, &team_member_archen, &species_archeops);

    let team_member_archeops = team_member_archen;

    let psychic_lin = create_trainer(conn, "Lin", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &psychic_lin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_micki = create_trainer(conn, "Micki", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &psychic_micki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let psychic_bryce = create_trainer(conn, "Bryce", &psychic);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &psychic_bryce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let nurse_sachiko = create_trainer(conn, "Sachiko", &nurse);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &nurse_sachiko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &37);

    let ace_trainer_beckett = create_trainer(conn, "Beckett", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &ace_trainer_beckett,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &celestial_tower_unova,
        &ace_trainer_kassandra,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &37);

    let worker_cliff = create_trainer(conn, "Cliff", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_city_unova,
        &worker_cliff,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &38);

    let worker_brady = create_trainer(conn, "Brady", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_city_unova,
        &worker_brady,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pilot = create_trainer_class(conn, "Pilot");

    let pilot_ted = create_trainer(conn, "Ted", &pilot);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_city_unova,
        &pilot_ted,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pilot_chase = create_trainer(conn, "Chase", &pilot);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_city_unova,
        &pilot_chase,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &39);

    let worker_arnold = create_trainer(conn, "Arnold", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_city_unova,
        &worker_arnold,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_skyla = create_trainer(conn, "Skyla", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_city_unova,
        &leader_skyla,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &40);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &38);

    let route_1_unova = create_location(conn, "Route 1", "Unova");

    let pkmn_ranger_brenda = create_trainer(conn, "Brenda", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_unova,
        &pkmn_ranger_brenda,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &38);

    let fisherman_sean = create_trainer(conn, "Sean", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_unova,
        &fisherman_sean,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ranger = create_trainer_class(conn, "Ranger");

    let ranger_claude = create_trainer(conn, "Claude", &ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_1_unova,
        &ranger_claude,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_17_unova = create_location(conn, "Route 17", "Unova");

    let fisherman_lydon = create_trainer(conn, "Lydon", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &fisherman_lydon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &38);

    level_up(conn, &battle, &team_member_stoutland, &39);

    let swimmer_f = create_trainer_class(conn, "Swimmer F");

    let swimmer_f_joyce = create_trainer(conn, "Joyce", &swimmer_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &swimmer_f_joyce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_m = create_trainer_class(conn, "Swimmer M");

    let swimmer_m_wright = create_trainer(conn, "Wright", &swimmer_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &swimmer_m_wright,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &39);

    let hiker_jeremiah = create_trainer(conn, "Jeremiah", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &hiker_jeremiah,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &40);

    let backpacker_kumiko = create_trainer(conn, "Kumiko", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &backpacker_kumiko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let backpacker_sam = create_trainer(conn, "Sam", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &backpacker_sam,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &39);

    let veteran_ray = create_trainer(conn, "Ray", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &veteran_ray,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let p2_laboratory_unova = create_location(conn, "P2 Laboratory", "Unova");

    let scientist_nathan = create_trainer(conn, "Nathan", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &scientist_nathan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_m_berke = create_trainer(conn, "Berke", &swimmer_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &swimmer_m_berke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &40);

    let swimmer_f_kelsey = create_trainer(conn, "Kelsey", &swimmer_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &swimmer_f_kelsey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle_girl_hillary = create_trainer(conn, "Hillary", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_17_unova,
        &battle_girl_hillary,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let mistralton_cave_unova = create_location(conn, "Mistralton Cave", "Unova");

    let hiker_hugh = create_trainer(conn, "Hugh", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_cave_unova,
        &hiker_hugh,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &40);

    let battle = create_battle(
        conn,
        &playthrough,
        &mistralton_cave_unova,
        &hiker_clarke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_darrell = create_trainer(conn, "Darrell", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &hiker_darrell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &41);

    let ace_trainer_caroll = create_trainer(conn, "Caroll", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &ace_trainer_caroll,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &41);

    let battle_girl_sharon = create_trainer(conn, "Sharon", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &battle_girl_sharon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &41);

    let worker_brand = create_trainer(conn, "Brand", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &worker_brand,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_heath = create_trainer(conn, "Heath", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &worker_heath,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let worker_rob = create_trainer(conn, "Rob", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &worker_rob,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &42);

    let worker_cairn = create_trainer(conn, "Cairn", &worker);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &worker_cairn,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let doctor_hank = create_trainer(conn, "Hank", &doctor);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &doctor_hank,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &41);

    let ace_trainer_jordan = create_trainer(conn, "Jordan", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &twist_mountain_unova,
        &ace_trainer_jordan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &42);

    let dragonspiral_tower_unova = create_location(conn, "Dragonspiral Tower", "Unova");

    let wild_mienfoo = create_trainer(conn, "Mienfoo", &wild);

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &wild_mienfoo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let species_mienfoo = create_species(conn, "Mienfoo", &619, &5, "Fighting", None);

    let team_member_mienfoo = catch_pokemon(
        conn,
        &playthrough,
        &5,
        &species_mienfoo,
        None,
        "Grass",
        "2023-08-22",
        &dragonspiral_tower_unova,
        &32,
        "M",
        "Great Ball",
    );

    let route_8_unova = create_location(conn, "Route 8", "Unova");

    let pkmn_ranger_lewis = create_trainer(conn, "Lewis", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_unova,
        &pkmn_ranger_lewis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let parasol_lady_melita = create_trainer(conn, "Melita", &parasol_lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_unova,
        &parasol_lady_melita,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let parasol_lady_lumi = create_trainer(conn, "Lumi", &parasol_lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_unova,
        &parasol_lady_lumi,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &33);

    let fisherman_bruce = create_trainer(conn, "Bruce", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_unova,
        &fisherman_bruce,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_annie = create_trainer(conn, "Annie", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_unova,
        &pkmn_ranger_annie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &34);

    let moor_of_icirrus_unova = create_location(conn, "Moor of Icirrus", "Unova");

    let pkmn_ranger_harry = create_trainer(conn, "Harry", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &moor_of_icirrus_unova,
        &pkmn_ranger_harry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let pkmn_ranger_chloris = create_trainer(conn, "Chloris", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &moor_of_icirrus_unova,
        &pkmn_ranger_chloris,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &35);

    let fisherman_damon = create_trainer(conn, "Damon", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &moor_of_icirrus_unova,
        &fisherman_damon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let icirrus_city_unova = create_location(conn, "Icirrus City", "Unova");

    let black_belt_grant = create_trainer(conn, "Grant", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &black_belt_grant,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &36);

    let battle_girl_miriam = create_trainer(conn, "Miriam", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &battle_girl_miriam,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &42);

    let black_belt_kendrew = create_trainer(conn, "Kendrew", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &black_belt_kendrew,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle_girl_mikiko = create_trainer(conn, "Mikiko", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &battle_girl_mikiko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle_girl_chandra = create_trainer(conn, "Chandra", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &battle_girl_chandra,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &37);

    let black_belt_thomas = create_trainer(conn, "Thomas", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &black_belt_thomas,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_brycen = create_trainer(conn, "Brycen", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &icirrus_city_unova,
        &leader_brycen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &43);

    level_up(conn, &battle, &team_member_mienfoo, &38);

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &42);

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pansear, &43);

    level_up(conn, &battle, &team_member_mienfoo, &39);

    let species_simisear = create_species(conn, "Simisear", &514, &5, "Fire", None);

    evolve(conn, &battle, &team_member_pansear, &species_simisear);

    let team_member_simisear = team_member_pansear;

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &40);

    level_up(conn, &battle, &team_member_seismitoad, &43);

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &41);

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &43);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_8_unova,
        &pkmn_trainer_bianca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &42);

    let route_9_unova = create_location(conn, "Route 9", "Unova");

    let biker_phillip = create_trainer(conn, "Phillip", &biker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &biker_phillip,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let roughneck = create_trainer_class(conn, "Roughneck");

    let roughneck_reese = create_trainer(conn, "Reese", &roughneck);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &roughneck_reese,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hooligans = create_trainer_class(conn, "Hooligans");

    let hooligans_jim_and_cas = create_trainer(conn, "Jim & Cas", &hooligans);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &hooligans_jim_and_cas,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &biker_zeke,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let roughneck_chance = create_trainer(conn, "Chance", &roughneck);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &roughneck_chance,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let waitress_flo = create_trainer(conn, "Flo", &waitress);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &waitress_flo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &43);

    let rich_boy_manuel = create_trainer(conn, "Manuel", &rich_boy);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &rich_boy_manuel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &44);

    let lady_isabel = create_trainer(conn, "Isabel", &lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &lady_isabel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let waiter_bert = create_trainer(conn, "Bert", &waiter);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &waiter_bert,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let wild_pawniard = create_trainer(conn, "Pawniard", &wild);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_9_unova,
        &wild_pawniard,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let species_pawniard = create_species(conn, "Pawniard", &624, &5, "Dark", Some("Steel"));

    let team_member_pawniard = catch_pokemon(
        conn,
        &playthrough,
        &6,
        &species_pawniard,
        None,
        "Grass",
        "2023-08-26",
        &route_9_unova,
        &32,
        "M",
        "Ultra Ball",
    );

    let opelucid_city_unova = create_location(conn, "Opelucid City", "Unova");

    let ace_trainer_eileen = create_trainer(conn, "Eileen", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_eileen,
        None,
        None,
        "Rotation",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &33);

    let ace_trainer_lou = create_trainer(conn, "Lou", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_lou,
        None,
        None,
        "Rotation",
        &0,
        &false,
    );

    let ace_trainer_webster = create_trainer(conn, "Webster", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_webster,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_olwen = create_trainer(conn, "Olwen", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_olwen,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &34);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_jose,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &44);

    let ace_trainer_clara = create_trainer(conn, "Clara", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_clara,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let veteran_hugo = create_trainer(conn, "Hugo", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &veteran_hugo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &35);

    let ace_trainer_tom = create_trainer(conn, "Tom", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_tom,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_dara = create_trainer(conn, "Dara", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &ace_trainer_dara,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &36);

    let veteran_kim = create_trainer(conn, "Kim", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &veteran_kim,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let leader_drayden = create_trainer(conn, "Drayden", &leader);

    let battle = create_battle(
        conn,
        &playthrough,
        &opelucid_city_unova,
        &leader_drayden,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &37);

    level_up(conn, &battle, &team_member_seismitoad, &44);

    let route_10_unova = create_location(conn, "Route 10", "Unova");

    let battle_girl_amy = create_trainer(conn, "Amy", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &battle_girl_amy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &38);

    let ace_trainer_johan = create_trainer(conn, "Johan", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &ace_trainer_johan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &45);

    let veteran_karla = create_trainer(conn, "Karla", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &veteran_karla,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &39);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &pkmn_trainer_cheren,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &40);

    level_up(conn, &battle, &team_member_simisear, &44);

    let black_belt_corey = create_trainer(conn, "Corey", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &black_belt_corey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let hiker_bret = create_trainer(conn, "Bret", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &hiker_bret,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &41);

    let ace_trainer_cheyenne = create_trainer(conn, "Cheyenne", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &ace_trainer_cheyenne,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &44);

    level_up(conn, &battle, &team_member_stoutland, &45);

    let veteran_chester = create_trainer(conn, "Chester", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_10_unova,
        &veteran_chester,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &42);

    let victory_road_unova = create_location(conn, "Victory Road", "Unova");

    let ace_trainer_shanta = create_trainer(conn, "Shanta", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &ace_trainer_shanta,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_dwayne = create_trainer(conn, "Dwayne", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &ace_trainer_dwayne,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &43);

    let veteran_tiffany = create_trainer(conn, "Tiffany", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &veteran_tiffany,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let ace_trainer_cathy = create_trainer(conn, "Cathy", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &ace_trainer_cathy,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &46);

    level_up(conn, &battle, &team_member_pawniard, &44);

    let black_belt_tyrone = create_trainer(conn, "Tyrone", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &black_belt_tyrone,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let doctor_logan = create_trainer(conn, "Logan", &doctor);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &doctor_logan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &47);

    level_up(conn, &battle, &team_member_pawniard, &45);

    let ace_trainer_david = create_trainer(conn, "David", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &ace_trainer_david,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &45);

    let veteran_martell = create_trainer(conn, "Martell", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &veteran_martell,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &victory_road_unova,
        &veteran_martell,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &46);

    let pokémon_league_unova = create_location(conn, "Pokémon League", "Unova");

    let elite_four_marshal = create_trainer(conn, "Marshal", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &45);

    let elite_four_shauntal = create_trainer(conn, "Shauntal", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let elite_four_grimsley = create_trainer(conn, "Grimsley", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &45);

    let elite_four_caitlin = create_trainer(conn, "Caitlin", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &46);

    level_up(conn, &battle, &team_member_archeops, &48);

    level_up(conn, &battle, &team_member_mienfoo, &46);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_pawniard, &46);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &46);

    level_up(conn, &battle, &team_member_simisear, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_pawniard, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &47);

    level_up(conn, &battle, &team_member_archeops, &49);

    level_up(conn, &battle, &team_member_seismitoad, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &47);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &48);

    level_up(conn, &battle, &team_member_archeops, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_seismitoad, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &49);

    level_up(conn, &battle, &team_member_mienfoo, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &48);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_pawniard, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &52);

    level_up(conn, &battle, &team_member_simisear, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &50);

    level_up(conn, &battle, &team_member_seismitoad, &49);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienfoo, &50);

    let species_mienshao = create_species(conn, "Mienshao", &620, &5, "Fighting", None);

    evolve(conn, &battle, &team_member_mienfoo, &species_mienshao);

    let team_member_mienshao = team_member_mienfoo;

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &50);

    level_up(conn, &battle, &team_member_mienshao, &52);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_pawniard, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &53);

    level_up(conn, &battle, &team_member_seismitoad, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &50);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &53);

    level_up(conn, &battle, &team_member_simisear, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &52);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &53);

    level_up(conn, &battle, &team_member_simisear, &54);

    let ns_castle_unova = create_location(conn, "N's Castle", "Unova");

    let team_plasma_n = create_trainer(conn, "N", &team_plasma);

    let battle = create_battle(
        conn,
        &playthrough,
        &ns_castle_unova,
        &team_plasma_n,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &51);

    let battle = create_battle(
        conn,
        &playthrough,
        &ns_castle_unova,
        &team_plasma_n,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &52);

    level_up(conn, &battle, &team_member_mienshao, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &ns_castle_unova,
        &team_plasma_n,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &53);

    let battle = create_battle(
        conn,
        &playthrough,
        &ns_castle_unova,
        &team_plasma_n,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &55);

    level_up(conn, &battle, &team_member_simisear, &55);

    level_up(conn, &battle, &team_member_stoutland, &54);

    level_up(conn, &battle, &team_member_archeops, &54);

    let team_plasma_ghetsis = create_trainer(conn, "Ghetsis", &team_plasma);

    let battle = create_battle(
        conn,
        &playthrough,
        &ns_castle_unova,
        &team_plasma_ghetsis,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_pawniard, &52);

    level_up(conn, &battle, &team_member_seismitoad, &52);

    let species_bisharp = create_species(conn, "Bisharp", &625, &5, "Dark", Some("Steel"));

    evolve(conn, &battle, &team_member_pawniard, &species_bisharp);

    let team_member_bisharp = team_member_pawniard;

    let scientist_markus = create_trainer(conn, "Markus", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &dreamyard_unova,
        &scientist_markus,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &55);

    let scientist_kathrine = create_trainer(conn, "Kathrine", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &dreamyard_unova,
        &scientist_kathrine,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &relic_castle_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &53);

    let royal_unova_unova = create_location(conn, "Royal Unova", "Unova");

    let ace_trainer_mariana = create_trainer(conn, "Mariana", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &royal_unova_unova,
        &ace_trainer_mariana,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &53);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &54);

    let battle = create_battle(
        conn,
        &playthrough,
        &chargestone_cave_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let challengers_cave_unova = create_location(conn, "Challenger's Cave", "Unova");

    let ace_trainer_terry = create_trainer(conn, "Terry", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &challengers_cave_unova,
        &ace_trainer_terry,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &54);

    level_up(conn, &battle, &team_member_archeops, &55);

    let ace_trainer_beverly = create_trainer(conn, "Beverly", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &challengers_cave_unova,
        &ace_trainer_beverly,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let veteran_shaun = create_trainer(conn, "Shaun", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &challengers_cave_unova,
        &veteran_shaun,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_seismitoad, &55);

    level_up(conn, &battle, &team_member_archeops, &56);

    level_up(conn, &battle, &team_member_simisear, &56);

    let backpacker_toru = create_trainer(conn, "Toru", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &challengers_cave_unova,
        &backpacker_toru,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &55);

    let battle = create_battle(
        conn,
        &playthrough,
        &challengers_cave_unova,
        &veteran_shaun,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &56);

    level_up(conn, &battle, &team_member_mienshao, &56);

    let backpacker_lora = create_trainer(conn, "Lora", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_16_unova,
        &backpacker_lora,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let marvelous_bridge_unova = create_location(conn, "Marvelous Bridge", "Unova");

    let ace_trainer_glinda = create_trainer(conn, "Glinda", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &marvelous_bridge_unova,
        &ace_trainer_glinda,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_15_unova = create_location(conn, "Route 15", "Unova");

    let pokéfan_elliot = create_trainer(conn, "Elliot", &pokéfan);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_unova,
        &pokéfan_elliot,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &56);

    let hiker_kit = create_trainer(conn, "Kit", &hiker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_unova,
        &hiker_kit,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &56);

    let pkmn_ranger_keith = create_trainer(conn, "Keith", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_unova,
        &pkmn_ranger_keith,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &57);

    let battle_girl_susie = create_trainer(conn, "Susie", &battle_girl);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_unova,
        &battle_girl_susie,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &57);

    let pkmn_ranger_shelly = create_trainer(conn, "Shelly", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_15_unova,
        &pkmn_ranger_shelly,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_city_unova = create_location(conn, "Black City", "Unova");

    let veteran_ken = create_trainer(conn, "Ken", &veteran);

    let battle = create_battle(
        conn,
        &playthrough,
        &black_city_unova,
        &veteran_ken,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_ryder = create_trainer(conn, "Ryder", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &black_city_unova,
        &black_belt_ryder,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &58);

    let roughneck_johnny = create_trainer(conn, "Johnny", &roughneck);

    let battle = create_battle(
        conn,
        &playthrough,
        &black_city_unova,
        &roughneck_johnny,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &57);

    let backpacker_kiyo = create_trainer(conn, "Kiyo", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &black_city_unova,
        &backpacker_kiyo,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &58);

    level_up(conn, &battle, &team_member_bisharp, &57);

    let roughneck_dave = create_trainer(conn, "Dave", &roughneck);

    let battle = create_battle(
        conn,
        &playthrough,
        &black_city_unova,
        &roughneck_dave,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_14_unova = create_location(conn, "Route 14", "Unova");

    let ace_trainer_junko = create_trainer(conn, "Junko", &ace_trainer);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_unova,
        &ace_trainer_junko,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &57);

    level_up(conn, &battle, &team_member_stoutland, &57);

    let backpacker_vicki = create_trainer(conn, "Vicki", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_unova,
        &backpacker_vicki,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let black_belt_jay = create_trainer(conn, "Jay", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_unova,
        &black_belt_jay,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_sid = create_trainer(conn, "Sid", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_14_unova,
        &fisherman_sid,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &58);

    let undella_bay_unova = create_location(conn, "Undella Bay", "Unova");

    let swimmer_f_tyra = create_trainer(conn, "Tyra", &swimmer_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &undella_bay_unova,
        &swimmer_f_tyra,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_m_matt = create_trainer(conn, "Matt", &swimmer_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &undella_bay_unova,
        &swimmer_m_matt,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_f_larissa = create_trainer(conn, "Larissa", &swimmer_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &undella_bay_unova,
        &swimmer_f_larissa,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &58);

    let swimmer_m_tim = create_trainer(conn, "Tim", &swimmer_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &undella_bay_unova,
        &swimmer_m_tim,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let swimmer_f_rebecca = create_trainer(conn, "Rebecca", &swimmer_f);

    let battle = create_battle(
        conn,
        &playthrough,
        &undella_bay_unova,
        &swimmer_f_rebecca,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &59);

    let swimmer_m_bart = create_trainer(conn, "Bart", &swimmer_m);

    let battle = create_battle(
        conn,
        &playthrough,
        &undella_bay_unova,
        &swimmer_m_bart,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &59);

    let route_13_unova = create_location(conn, "Route 13", "Unova");

    let socialite = create_trainer_class(conn, "Socialite");

    let socialite_marian = create_trainer(conn, "Marian", &socialite);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &socialite_marian,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let gentleman_yan = create_trainer(conn, "Yan", &gentleman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &gentleman_yan,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &58);

    let twins_emy_and_lin = create_trainer(conn, "Emy & Lin", &twins);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &twins_emy_and_lin,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &58);

    let black_belt_benjamin = create_trainer(conn, "Benjamin", &black_belt);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &black_belt_benjamin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_pete = create_trainer(conn, "Pete", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &fisherman_pete,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_vince = create_trainer(conn, "Vince", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &fisherman_vince,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let parasol_lady_laura = create_trainer(conn, "Laura", &parasol_lady);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &parasol_lady_laura,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_mick = create_trainer(conn, "Mick", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &fisherman_mick,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let fisherman_jones = create_trainer(conn, "Jones", &fisherman);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &fisherman_jones,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &59);

    let artist_zach = create_trainer(conn, "Zach", &artist);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &artist_zach,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &59);

    let youngster_astor = create_trainer(conn, "Astor", &youngster);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &youngster_astor,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let lass_fey = create_trainer(conn, "Fey", &lass);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_13_unova,
        &lass_fey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_11_unova = create_location(conn, "Route 11", "Unova");

    let backpacker_talon = create_trainer(conn, "Talon", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_unova,
        &backpacker_talon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &59);

    let pkmn_ranger_thalia = create_trainer(conn, "Thalia", &pkmn_ranger);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_unova,
        &pkmn_ranger_thalia,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &59);

    let backpacker_corin = create_trainer(conn, "Corin", &backpacker);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_11_unova,
        &backpacker_corin,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let village_bridge_unova = create_location(conn, "Village Bridge", "Unova");

    let school_kid_serena = create_trainer(conn, "Serena", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &village_bridge_unova,
        &school_kid_serena,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &60);

    let scientist_shannon = create_trainer(conn, "Shannon", &scientist);

    let battle = create_battle(
        conn,
        &playthrough,
        &village_bridge_unova,
        &scientist_shannon,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &60);

    let baker_chris = create_trainer(conn, "Chris", &baker);

    let battle = create_battle(
        conn,
        &playthrough,
        &village_bridge_unova,
        &baker_chris,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let route_12_unova = create_location(conn, "Route 12", "Unova");

    let pkmn_breeder_ethel = create_trainer(conn, "Ethel", &pkmn_breeder);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_unova,
        &pkmn_breeder_ethel,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &60);

    let pkmn_breeder_eustace = create_trainer(conn, "Eustace", &pkmn_breeder);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_unova,
        &pkmn_breeder_eustace,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let school_kid_jem = create_trainer(conn, "Jem", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_unova,
        &school_kid_jem,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &60);

    let backers = create_trainer_class(conn, "Backers");

    let backers_fey_and_sue = create_trainer(conn, "Fey & Sue", &backers);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_unova,
        &backers_fey_and_sue,
        None,
        None,
        "Double",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &60);

    let school_kid_ann = create_trainer(conn, "Ann", &school_kid);

    let battle = create_battle(
        conn,
        &playthrough,
        &route_12_unova,
        &school_kid_ann,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_bisharp, &60);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &61);

    level_up(conn, &battle, &team_member_bisharp, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_archeops, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_mienshao, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_seismitoad, &61);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_archeops, &62);

    level_up(conn, &battle, &team_member_seismitoad, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_mienshao, &62);

    level_up(conn, &battle, &team_member_bisharp, &62);

    level_up(conn, &battle, &team_member_stoutland, &62);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_bisharp, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &63);

    level_up(conn, &battle, &team_member_archeops, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_seismitoad, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_bisharp, &64);

    level_up(conn, &battle, &team_member_stoutland, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_mienshao, &63);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_mienshao, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_archeops, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &64);

    level_up(conn, &battle, &team_member_mienshao, &65);

    level_up(conn, &battle, &team_member_archeops, &65);

    level_up(conn, &battle, &team_member_simisear, &65);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &64);

    let elite_four_grimlsey = create_trainer(conn, "Grimlsey", &elite_four);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimlsey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &64);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &65);

    level_up(conn, &battle, &team_member_archeops, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &66);

    level_up(conn, &battle, &team_member_bisharp, &65);

    level_up(conn, &battle, &team_member_seismitoad, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &65);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_stoutland, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &66);

    level_up(conn, &battle, &team_member_mienshao, &67);

    level_up(conn, &battle, &team_member_bisharp, &66);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_seismitoad, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &67);

    level_up(conn, &battle, &team_member_mienshao, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_archeops, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &67);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_bisharp, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_seismitoad, &69);

    level_up(conn, &battle, &team_member_simisear, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &69);

    level_up(conn, &battle, &team_member_stoutland, &68);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_stoutland, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_simisear, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimsley,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_mienshao, &70);

    level_up(conn, &battle, &team_member_bisharp, &69);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_marshal,
        None,
        None,
        "Single",
        &0,
        &true,
    );

    level_up(conn, &battle, &team_member_simisear, &70);

    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimlsey,
        None,
        None,
        "Single",
        &0,
        &false,
    );

    level_up(conn, &battle, &team_member_archeops, &69);
    level_up(conn, &battle, &team_member_mienshao, &71);
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );
    level_up(conn, &battle, &team_member_bisharp, &70);
    level_up(conn, &battle, &team_member_archeops, &70);
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimlsey,
        None,
        None,
        "Single",
        &0,
        &false,
    );
    level_up(conn, &battle, &team_member_stoutland, &70);
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &false,
    );
    level_up(conn, &battle, &team_member_stoutland, &71);
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &true,
    );
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_grimlsey,
        None,
        None,
        "Single",
        &0,
        &false,
    );
    level_up(conn, &battle, &team_member_mienshao, &72);
    level_up(conn, &battle, &team_member_seismitoad, &70);
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_shauntal,
        None,
        None,
        "Single",
        &0,
        &false,
    );
    level_up(conn, &battle, &team_member_seismitoad, &71);
    let battle = create_battle(
        conn,
        &playthrough,
        &pokémon_league_unova,
        &elite_four_caitlin,
        None,
        None,
        "Single",
        &0,
        &true,
    );
}
