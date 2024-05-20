// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    instances (id) {
        id -> Nullable<Integer>,
        name -> Text,
        game_id -> Integer,
        data_dir -> Text,
        cache_dir -> Text,
        description -> Text,
        created -> BigInt,
        updated -> BigInt,
        install_dir -> Text,
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

diesel::joinable!(instances -> games (game_id));
diesel::joinable!(mods -> instances (instance_id));
diesel::joinable!(mods -> sources (source_id));
diesel::joinable!(supported_sources -> games (game_id));
diesel::joinable!(supported_sources -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(games, instances, mods, sources, supported_sources,);
