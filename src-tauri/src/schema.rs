// @generated automatically by Diesel CLI.

diesel::table! {
    Ball (name) {
        name -> Text,
    }
}

diesel::table! {
    Battle_Event (no) {
        no -> Integer,
        battle_type -> Text,
        opponent1_name -> Text,
        opponent1_class -> Text,
        opponent2_name -> Nullable<Text>,
        opponent2_class -> Nullable<Text>,
        partner_name -> Nullable<Text>,
        partner_class -> Nullable<Text>,
        round -> Integer,
        lost -> Bool,
    }
}

diesel::table! {
    Battle_Type (name) {
        name -> Text,
    }
}

diesel::table! {
    Catch_Event (no) {
        no -> Integer,
        enounter_type -> Text,
    }
}

diesel::table! {
    Catch_Type (name) {
        name -> Text,
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
    Item (name) {
        name -> Text,
    }
}

diesel::table! {
    Item_Event (no) {
        no -> Integer,
        item -> Text,
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
        form -> Nullable<Text>,
        generation -> Integer,
        type1 -> Text,
        type2 -> Nullable<Text>,
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
    Team_Member_Changes (no) {
        no -> Integer,
        team_member_playthrough_id_no -> Text,
        team_member_slot -> Text,
        event_no -> Integer,
        level_change -> Nullable<Integer>,
        species_dex_no -> Nullable<Integer>,
        species_form -> Nullable<Text>,
    }
}

diesel::table! {
    Trainer (name, class) {
        name -> Nullable<Text>,
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

diesel::joinable!(Battle_Event -> Battle_Type (battle_type));
diesel::joinable!(Battle_Event -> Event (no));
diesel::joinable!(Catch_Event -> Catch_Type (enounter_type));
diesel::joinable!(Catch_Event -> Event (no));
diesel::joinable!(Event -> Playthrough (playthrough_id_no));
diesel::joinable!(Item_Event -> Event (no));
diesel::joinable!(Item_Event -> Item (item));
diesel::joinable!(Location -> Region (region));
diesel::joinable!(Playthrough -> Version (version));
diesel::joinable!(Team_Member -> Ball (ball));
diesel::joinable!(Team_Member -> Playthrough (playthrough_id_no));
diesel::joinable!(Team_Member_Changes -> Event (event_no));
diesel::joinable!(Trainer -> Trainer_Class (class));

diesel::allow_tables_to_appear_in_same_query!(
    Ball,
    Battle_Event,
    Battle_Type,
    Catch_Event,
    Catch_Type,
    Event,
    Item,
    Item_Event,
    Location,
    Playthrough,
    Region,
    Species,
    Team_Member,
    Team_Member_Changes,
    Trainer,
    Trainer_Class,
    Type,
    Version,
);
