// @generated automatically by Diesel CLI.

diesel::table! {
    Ball (name) {
        name -> Text,
    }
}

diesel::table! {
    Battle_Event (no) {
        no -> Integer,
    }
}

diesel::table! {
    Event (no) {
        no -> Integer,
        playthrough_id_no -> Text,
        location_name -> Text,
        location_region -> Text,
    }
}

diesel::table! {
    Location (name, region) {
        name -> Text,
        region -> Text,
    }
}

diesel::table! {
    Playthrough (id_no) {
        id_no -> Text,
        name -> Text,
        version -> Text,
        adventure_started -> Text,
    }
}

diesel::table! {
    Region (name) {
        name -> Text,
    }
}

diesel::table! {
    Species (dex_no, form) {
        dex_no -> Integer,
        name -> Text,
        form -> Text,
        generation -> Integer,
    }
}

diesel::table! {
    SpeciesVersion (species_dex_no, species_form, version) {
        species_dex_no -> Integer,
        species_form -> Text,
        version -> Text,
        type1 -> Text,
        type2 -> Nullable<Text>,
        color -> Text,
    }
}

diesel::table! {
    Team_Member (playthrough_id_no, slot) {
        playthrough_id_no -> Text,
        slot -> Text,
        nickname -> Nullable<Text>,
        caught_date -> Text,
        caught_location_name -> Text,
        caught_location_region -> Text,
        caught_level -> Integer,
        ball -> Text,
        gender -> Text,
    }
}

diesel::table! {
    Trainer (name, class) {
        name -> Text,
        class -> Text,
    }
}

diesel::table! {
    Trainer_Class (name) {
        name -> Text,
    }
}

diesel::table! {
    Type (name) {
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    Version (name) {
        name -> Text,
        generation -> Integer,
    }
}

diesel::joinable!(Battle_Event -> Event (no));
diesel::joinable!(Event -> Playthrough (playthrough_id_no));
diesel::joinable!(Location -> Region (region));
diesel::joinable!(Playthrough -> Version (version));
diesel::joinable!(SpeciesVersion -> Version (version));
diesel::joinable!(Team_Member -> Ball (ball));
diesel::joinable!(Team_Member -> Playthrough (playthrough_id_no));
diesel::joinable!(Trainer -> Trainer_Class (class));

diesel::allow_tables_to_appear_in_same_query!(
    Ball,
    Battle_Event,
    Event,
    Location,
    Playthrough,
    Region,
    Species,
    SpeciesVersion,
    Team_Member,
    Trainer,
    Trainer_Class,
    Type,
    Version,
);
