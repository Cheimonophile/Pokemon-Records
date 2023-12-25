// @generated automatically by Diesel CLI.

diesel::table! {
    ball (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    battle_event (no) {
        no -> Integer,
        battle_type_id -> Integer,
        opponent1_id -> Integer,
        opponent2_id -> Nullable<Integer>,
        partner_id -> Nullable<Integer>,
        lost -> Bool,
        round -> Nullable<Integer>,
    }
}

diesel::table! {
    battle_type (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    catch_event (no) {
        no -> Integer,
        catch_type_id -> Integer,
        team_member_id -> Integer,
    }
}

diesel::table! {
    catch_type (id) {
        id -> Integer,
        name -> Text,
        detail -> Nullable<Text>,
    }
}

diesel::table! {
    event (no) {
        no -> Integer,
        playthrough_id_no -> Text,
        location_id -> Integer,
        date -> Text,
    }
}

diesel::table! {
    item (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    item_event (no) {
        no -> Integer,
        item_id -> Integer,
    }
}

diesel::table! {
    location (id) {
        id -> Integer,
        name -> Text,
        region_id -> Integer,
    }
}

diesel::table! {
    playthrough (id_no) {
        id_no -> Text,
        name -> Text,
        version_id -> Integer,
        adventure_started -> Text,
    }
}

diesel::table! {
    region (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    species (id) {
        id -> Integer,
        name -> Text,
        form -> Nullable<Text>,
        dex_no -> Integer,
        generation -> Integer,
        type1_id -> Integer,
        type2_id -> Nullable<Integer>,
    }
}

diesel::table! {
    team_member (id) {
        id -> Integer,
        playthrough_id_no -> Text,
        slot -> Integer,
        nickname -> Nullable<Text>,
        caught_date -> Text,
        caught_location_id -> Integer,
        caught_species_id -> Integer,
        caught_level -> Integer,
        ball_id -> Integer,
        gender -> Text,
    }
}

diesel::table! {
    team_member_change (no) {
        no -> Integer,
        event_no -> Integer,
        team_member_id -> Integer,
        level -> Nullable<Integer>,
        species_id -> Nullable<Integer>,
    }
}

diesel::table! {
    trainer (id) {
        id -> Integer,
        name -> Text,
        class_id -> Integer,
    }
}

diesel::table! {
    trainer_class (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    #[sql_name = "type"]
    type_ (id) {
        id -> Integer,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    version (id) {
        id -> Integer,
        name -> Text,
        generation -> Integer,
    }
}

diesel::joinable!(battle_event -> battle_type (battle_type_id));
diesel::joinable!(battle_event -> event (no));
diesel::joinable!(catch_event -> catch_type (catch_type_id));
diesel::joinable!(catch_event -> event (no));
diesel::joinable!(catch_event -> team_member (team_member_id));
diesel::joinable!(event -> location (location_id));
diesel::joinable!(event -> playthrough (playthrough_id_no));
diesel::joinable!(item_event -> event (no));
diesel::joinable!(item_event -> item (item_id));
diesel::joinable!(location -> region (region_id));
diesel::joinable!(playthrough -> version (version_id));
diesel::joinable!(team_member -> ball (ball_id));
diesel::joinable!(team_member -> location (caught_location_id));
diesel::joinable!(team_member -> playthrough (playthrough_id_no));
diesel::joinable!(team_member -> species (caught_species_id));
diesel::joinable!(team_member_change -> event (event_no));
diesel::joinable!(team_member_change -> species (species_id));
diesel::joinable!(team_member_change -> team_member (team_member_id));
diesel::joinable!(trainer -> trainer_class (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    ball,
    battle_event,
    battle_type,
    catch_event,
    catch_type,
    event,
    item,
    item_event,
    location,
    playthrough,
    region,
    species,
    team_member,
    team_member_change,
    trainer,
    trainer_class,
    type_,
    version,
);
