// @generated automatically by Diesel CLI.

diesel::table! {
    ball (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    battle_event (no) {
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
    battle_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    catch_event (no) {
        no -> Nullable<Integer>,
        catch_type_id -> Integer,
        team_member_id -> Integer,
    }
}

diesel::table! {
    catch_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
        detail -> Nullable<Text>,
    }
}

diesel::table! {
    event (no) {
        no -> Nullable<Integer>,
        playthrough_id_no -> Text,
        location_id -> Integer,
        date -> Text,
    }
}

diesel::table! {
    item (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    item_event (no) {
        no -> Nullable<Integer>,
        item_id -> Integer,
    }
}

diesel::table! {
    location (id) {
        id -> Nullable<Integer>,
        name -> Text,
        region_id -> Integer,
    }
}

diesel::table! {
    playthrough (id_no) {
        id_no -> Nullable<Text>,
        name -> Text,
        version_id -> Integer,
        adventure_started -> Text,
    }
}

diesel::table! {
    region (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    species (id) {
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
    team_member (id) {
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
    team_member_change (no) {
        no -> Nullable<Integer>,
        event_no -> Integer,
        team_member_id -> Integer,
        level -> Nullable<Integer>,
        species_id -> Nullable<Integer>,
    }
}

diesel::table! {
    trainer (id) {
        id -> Nullable<Integer>,
        name -> Text,
        class_id -> Integer,
    }
}

diesel::table! {
    trainer_class (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    #[sql_name = "type"]
    type_ (id) {
        id -> Nullable<Integer>,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    version (id) {
        id -> Nullable<Integer>,
        name -> Text,
        generation -> Integer,
    }
}

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
