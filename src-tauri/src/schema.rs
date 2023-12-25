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
        catch_type -> Text,
        team_member_id -> Integer,
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
    Species (name) {
        name -> Text,
        dex_no -> Integer,
        generation -> Integer,
        type1 -> Text,
        type2 -> Nullable<Text>,
    }
}

diesel::table! {
    Team_Member (id) {
        id -> Integer,
        playthrough_id_no -> Text,
        slot -> Integer,
        nickname -> Nullable<Text>,
        caught_date -> Text,
        caught_location_name -> Text,
        caught_location_region -> Text,
        caught_species_name -> Text,
        caught_level -> Integer,
        ball -> Text,
        gender -> Text,
    }
}

diesel::table! {
    Team_Member_Change (id) {
        id -> Integer,
        team_member_id -> Integer,
        event_no -> Integer,
        level -> Nullable<Integer>,
        species_name -> Nullable<Text>,
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

diesel::table! {
    new_ball (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    new_battle_event (no) {
        no -> Nullable<Integer>,
        battle_type_id -> Integer,
        opponent1_id -> Integer,
        opponent2_id -> Nullable<Integer>,
        partner_id -> Nullable<Integer>,
        lost -> Bool,
        round -> Nullable<Integer>,
    }
}

diesel::table! {
    new_battle_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    new_catch_event (no) {
        no -> Nullable<Integer>,
        catch_type_id -> Integer,
        team_member_id -> Integer,
    }
}

diesel::table! {
    new_catch_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    new_event (no) {
        no -> Nullable<Integer>,
        playthrough_id_no -> Text,
        location_id -> Integer,
        date -> Text,
    }
}

diesel::table! {
    new_item (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    new_item_event (no) {
        no -> Nullable<Integer>,
        item_id -> Integer,
    }
}

diesel::table! {
    new_location (id) {
        id -> Nullable<Integer>,
        name -> Text,
        region_id -> Integer,
    }
}

diesel::table! {
    new_playthrough (id_no) {
        id_no -> Nullable<Text>,
        name -> Text,
        version_id -> Integer,
        adventure_started -> Text,
    }
}

diesel::table! {
    new_region (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    new_species (id) {
        id -> Nullable<Integer>,
        name -> Text,
        form -> Nullable<Text>,
        dex_no -> Integer,
        generation -> Integer,
        type1_id -> Integer,
        type2_id -> Nullable<Integer>,
    }
}

diesel::table! {
    new_team_member (id) {
        id -> Nullable<Integer>,
        playthrough_id_no -> Text,
        slot -> Integer,
        nickname -> Nullable<Text>,
        caught_date -> Text,
        caught_location_id -> Nullable<Integer>,
        caught_species_id -> Integer,
        caught_level -> Integer,
        ball_id -> Integer,
        gender -> Text,
    }
}

diesel::table! {
    new_team_member_change (no) {
        no -> Nullable<Integer>,
        event_no -> Integer,
        team_member_id -> Integer,
        level -> Nullable<Integer>,
        species_id -> Nullable<Integer>,
    }
}

diesel::table! {
    new_trainer (id) {
        id -> Nullable<Integer>,
        name -> Text,
        class_id -> Integer,
    }
}

diesel::table! {
    new_trainer_class (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    new_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    new_version (id) {
        id -> Nullable<Integer>,
        name -> Text,
        generation -> Integer,
    }
}

diesel::joinable!(Battle_Event -> Battle_Type (battle_type));
diesel::joinable!(Battle_Event -> Event (no));
diesel::joinable!(Catch_Event -> Catch_Type (catch_type));
diesel::joinable!(Catch_Event -> Event (no));
diesel::joinable!(Catch_Event -> Team_Member (team_member_id));
diesel::joinable!(Event -> Playthrough (playthrough_id_no));
diesel::joinable!(Item_Event -> Event (no));
diesel::joinable!(Item_Event -> Item (item));
diesel::joinable!(Location -> Region (region));
diesel::joinable!(Playthrough -> Version (version));
diesel::joinable!(Team_Member -> Ball (ball));
diesel::joinable!(Team_Member -> Playthrough (playthrough_id_no));
diesel::joinable!(Team_Member -> Species (caught_species_name));
diesel::joinable!(Team_Member_Change -> Event (event_no));
diesel::joinable!(Team_Member_Change -> Species (species_name));
diesel::joinable!(Team_Member_Change -> Team_Member (team_member_id));
diesel::joinable!(Trainer -> Trainer_Class (class));
diesel::joinable!(new_battle_event -> new_battle_type (battle_type_id));
diesel::joinable!(new_battle_event -> new_event (no));
diesel::joinable!(new_catch_event -> new_catch_type (catch_type_id));
diesel::joinable!(new_catch_event -> new_event (no));
diesel::joinable!(new_catch_event -> new_team_member (team_member_id));
diesel::joinable!(new_event -> new_location (location_id));
diesel::joinable!(new_event -> new_playthrough (playthrough_id_no));
diesel::joinable!(new_item_event -> new_event (no));
diesel::joinable!(new_item_event -> new_item (item_id));
diesel::joinable!(new_location -> new_region (region_id));
diesel::joinable!(new_playthrough -> new_version (version_id));
diesel::joinable!(new_team_member -> new_ball (ball_id));
diesel::joinable!(new_team_member -> new_location (caught_location_id));
diesel::joinable!(new_team_member -> new_playthrough (playthrough_id_no));
diesel::joinable!(new_team_member -> new_species (caught_species_id));
diesel::joinable!(new_team_member_change -> new_event (event_no));
diesel::joinable!(new_team_member_change -> new_species (species_id));
diesel::joinable!(new_team_member_change -> new_team_member (team_member_id));
diesel::joinable!(new_trainer -> new_trainer_class (class_id));

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
    Team_Member_Change,
    Trainer,
    Trainer_Class,
    Type,
    Version,
    new_ball,
    new_battle_event,
    new_battle_type,
    new_catch_event,
    new_catch_type,
    new_event,
    new_item,
    new_item_event,
    new_location,
    new_playthrough,
    new_region,
    new_species,
    new_team_member,
    new_team_member_change,
    new_trainer,
    new_trainer_class,
    new_type,
    new_version,
);
