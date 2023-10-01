
use crate::dbi::connection::connect;
use crate::dbi::events::*;
use crate::dbi::structs::*;

pub fn run() {
    // connection
    let conn = &mut connect();


    let playthrough = create_playthrough(conn, "26852", "Ben", "Black", "2023-06-24");
    

    let nuvema_town_unova = create_location(conn, "Nuvema Town", "Unova");
    

        let species_lillipup = create_species(conn, &506, "Lillipup", None, &5, "Normal", None);
    

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
            "Poke Ball",
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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &7,
    );
    

    let route_2_unova = create_location(conn, "Route 2", "Unova");
    

            let youngster = create_trainer_class(conn, "Youngster");
            

        let youngster_jimmy = create_trainer(conn, "Jimmy", &youngster);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &8,
    );
    

            let lass = create_trainer_class(conn, "Lass");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &9,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &10,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &11,
    );
    

        let youngster_joey = create_trainer(conn, "Joey", &youngster);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &12,
    );
    

        let species_pansear = create_species(conn, &513, "Pansear", None, &5, "Fire", None);
    

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
            "Poke Ball",
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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &11,
    );
    

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
    

            let leader = create_trainer_class(conn, "Leader");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &13,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &12,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &14,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &13,
    );
    

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
    

            let twins = create_trainer_class(conn, "Twins");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &14,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &15,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &15,
    );
    

            let school_kid = create_trainer_class(conn, "School Kid");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &16,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_lillipup,
        &16,
    );
    

        let species_herdier = create_species(conn, &507, "Herdier", None, &5, "Normal", None);
    

        evolve(conn,
            &battle,
            &team_member_lillipup,
            &species_herdier,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &17,
    );
    

            let wild = create_trainer_class(conn, "Wild");
            

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
    

        let species_tympole = create_species(conn, &535, "Tympole", None, &5, "Water", None);
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &18,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &13,
    );
    

            let black_belt = create_trainer_class(conn, "Black Belt");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &14,
    );
    

            let scientist = create_trainer_class(conn, "Scientist");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &15,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &17,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &19,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &20,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &16,
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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &17,
    );
    

            let pkmn_ranger = create_trainer_class(conn, "PKMN Ranger");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &21,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &18,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &18,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &22,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &19,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &19,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &20,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &23,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &20,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &21,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &21,
    );
    

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
    

            let fisherman = create_trainer_class(conn, "Fisherman");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &22,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &22,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &24,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &23,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &23,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &24,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &24,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &25,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &25,
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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &26,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_tympole,
        &25,
    );
    

        let species_palpitoad = create_species(conn, &536, "Palpitoad", None, &5, "Water", Some("Ground"));
    

        evolve(conn,
            &battle,
            &team_member_tympole,
            &species_palpitoad,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &26,
    );
    

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
    

            let psychic = create_trainer_class(conn, "Psychic");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &26,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &27,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &27,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &27,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &28,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &28,
    );
    

    let route_16_unova = create_location(conn, "Route 16", "Unova");
    

            let policeman = create_trainer_class(conn, "Policeman");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &28,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &29,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &29,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &29,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &30,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &31,
    );
    

        let species_archen = create_species(conn, &566, "Archen", None, &5, "Rock", Some("Flying"));
    

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
            "Poke Ball",
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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &30,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &32,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &26,
    );
    

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
    

        let youngster_albert = create_trainer(conn, "Albert", &youngster);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &27,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &30,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &28,
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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &31,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &29,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &31,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &30,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &31,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &33,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &32,
    );
    

    let chargestone_cave_unova = create_location(conn, "Chargestone Cave", "Unova");
    

            let ace_trainer = create_trainer_class(conn, "Ace Trainer");
            

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
    

            let hiker = create_trainer_class(conn, "Hiker");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &32,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &33,
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
    

    level_up(
        conn,
        &battle,
        &team_member_herdier,
        &32,
    );
    

        let species_stoutland = create_species(conn, &508, "Stoutland", None, &5, "Normal", None);
    

        evolve(conn,
            &battle,
            &team_member_herdier,
            &species_stoutland,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &33,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &34,
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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &34,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &33,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &34,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &34,
    );
    

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
    

        let youngster_mikey = create_trainer(conn, "Mikey", &youngster);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &35,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &35,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &35,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &35,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &36,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &36,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &36,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_palpitoad,
        &36,
    );
    

        let species_seismitoad = create_species(conn, &537, "Seismitoad", None, &5, "Water", Some("Ground"));
    

        evolve(conn,
            &battle,
            &team_member_palpitoad,
            &species_seismitoad,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &37,
    );
    

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
    

            let pokéfan = create_trainer_class(conn, "Pokéfan");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_archen,
        &37,
    );
    

        let species_archeops = create_species(conn, &567, "Archeops", None, &5, "Rock", Some("Flying"));
    

        evolve(conn,
            &battle,
            &team_member_archen,
            &species_archeops,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &37,
    );
    

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
    

        let ace_trainer_kassandra = create_trainer(conn, "Kassandra", &ace_trainer);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &37,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &38,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &39,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &40,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &38,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &38,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &38,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &39,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &39,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &40,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &39,
    );
    

            let veteran = create_trainer_class(conn, "Veteran");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &40,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &40,
    );
    

        let hiker_clarke = create_trainer(conn, "Clarke", &hiker);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &41,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &41,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &41,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &42,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &41,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &42,
    );
    

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
    

        let species_mienfoo = create_species(conn, &619, "Mienfoo", None, &5, "Fighting", None);
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &33,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &34,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &35,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &36,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &42,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &37,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &43,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &38,
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
        &dragonspiral_tower_unova,
        &team_plasma_grunt,
        None,
        None,
        "Single",
        &0,
        &false,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &42,
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
    

    level_up(
        conn,
        &battle,
        &team_member_pansear,
        &43,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &39,
    );
    

        let species_simisear = create_species(conn, &514, "Simisear", None, &5, "Fire", None);
    

        evolve(conn,
            &battle,
            &team_member_pansear,
            &species_simisear,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &40,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &43,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &41,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &43,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &42,
    );
    

    let route_9_unova = create_location(conn, "Route 9", "Unova");
    

            let biker = create_trainer_class(conn, "Biker");
            

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
    

        let biker_zeke = create_trainer(conn, "Zeke", &biker);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &43,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &44,
    );
    

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
    

        let species_pawniard = create_species(conn, &624, "Pawniard", None, &5, "Dark", Some("Steel"));
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &33,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &34,
    );
    

        let ace_trainer_jose = create_trainer(conn, "Jose", &ace_trainer);
        

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &44,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &35,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &36,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &37,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &44,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &38,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &45,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &39,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &40,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &44,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &41,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &44,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &45,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &42,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &43,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &46,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &44,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &47,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &45,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &45,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &46,
    );
    

    let pokémon_league_unova = create_location(conn, "Pokémon League", "Unova");
    

            let elite_four = create_trainer_class(conn, "Elite Four");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &45,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &45,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &46,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &48,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &46,
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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &46,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &46,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &47,
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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &47,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &47,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &49,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &47,
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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &47,
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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &48,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &50,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &48,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &48,
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
        &true,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &49,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &49,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &48,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &51,
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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &48,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &49,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &52,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &49,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &50,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &49,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienfoo,
        &50,
    );
    

        let species_mienshao = create_species(conn, &620, "Mienshao", None, &5, "Fighting", None);
    

        evolve(conn,
            &battle,
            &team_member_mienfoo,
            &species_mienshao,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &51,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &50,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &52,
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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &51,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &50,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &53,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &51,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &50,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &53,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &51,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &52,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &53,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &54,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &51,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &52,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &54,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &53,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &55,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &55,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &54,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &54,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_pawniard,
        &52,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &52,
    );
    

        let species_bisharp = create_species(conn, &625, "Bisharp", None, &5, "Dark", Some("Steel"));
    

        evolve(conn,
            &battle,
            &team_member_pawniard,
            &species_bisharp,
        );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &55,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &53,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &53,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &54,
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
    

    let challengers_cave_unova = create_location(conn, "Challenger's Cave", "Unova");
    

            let acd_trainer = create_trainer_class(conn, "Acd Trainer");
            

        let acd_trainer_terry = create_trainer(conn, "Terry", &acd_trainer);
        

    let battle = create_battle(
        conn,
        &playthrough,
        &challengers_cave_unova,
        &acd_trainer_terry,
        None,
        None,
        "Single",
        &0,
        &false,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &54,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &55,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &55,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &56,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &56,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &55,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &56,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &56,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &56,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &56,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &57,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &57,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &58,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &57,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &58,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &57,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &57,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &57,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &58,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &58,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &59,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &59,
    );
    

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
    

            let gentleman = create_trainer_class(conn, "Gentleman");
            

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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &58,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &58,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &59,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &59,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &59,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &59,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &60,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &60,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &60,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &60,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &60,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &60,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &61,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &61,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &61,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &61,
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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &61,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &61,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &62,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &62,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &62,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &62,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &62,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &62,
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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &63,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &63,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &63,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &63,
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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &64,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &63,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &63,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &64,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &64,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &64,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &65,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &65,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &65,
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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &64,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &64,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &65,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &66,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &66,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &65,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &66,
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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &65,
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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &66,
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
        &true,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &67,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &66,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &67,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &66,
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
        &true,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &67,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &67,
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
        &true,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &68,
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
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &67,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &68,
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
        &true,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_archeops,
        &68,
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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &67,
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
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &68,
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
    

    level_up(
        conn,
        &battle,
        &team_member_seismitoad,
        &69,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &68,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &69,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &68,
    );
    

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
    

    level_up(
        conn,
        &battle,
        &team_member_stoutland,
        &69,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &69,
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
    

    level_up(
        conn,
        &battle,
        &team_member_mienshao,
        &70,
    );
    

    level_up(
        conn,
        &battle,
        &team_member_bisharp,
        &69,
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
    

    level_up(
        conn,
        &battle,
        &team_member_simisear,
        &70,
    );
    

}

