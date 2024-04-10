// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    instance_meta (id) {
        id -> Nullable<Integer>,
        instance_id -> Integer,
        game_id -> Integer,
        data_dir -> Text,
        cache_dir -> Text,
        description -> Text,
        created -> BigInt,
        updated -> BigInt,
    }
}

diesel::table! {
    instances (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    mods (id) {
        id -> Nullable<Integer>,
        mod_id -> Text,
        version_id -> Nullable<Text>,
        name -> Text,
        file_name -> Text,
        source_id -> Nullable<Integer>,
        instance_id -> Nullable<Integer>,
        size -> Nullable<Integer>,
        hash -> Nullable<Text>,
    }
}

diesel::table! {
    sources (id) {
        id -> Nullable<Integer>,
        name -> Text,
        base_url -> Text,
    }
}

diesel::table! {
    supported_sources (id) {
        id -> Nullable<Integer>,
        is_supported -> Bool,
        source_id -> Integer,
        game_id -> Integer,
    }
}

diesel::joinable!(instance_meta -> games (game_id));
diesel::joinable!(instance_meta -> instances (instance_id));
diesel::joinable!(mods -> instances (instance_id));
diesel::joinable!(mods -> sources (source_id));
diesel::joinable!(supported_sources -> games (game_id));
diesel::joinable!(supported_sources -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    instance_meta,
    instances,
    mods,
    sources,
    supported_sources,
);
