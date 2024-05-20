// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::{
    game::Game,
    schema::{sources, supported_sources},
    sources,
};
use diesel::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Queryable,
    Identifiable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Type,
)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<i32>,
    pub name: String,
    pub base_url: String,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Queryable,
    Identifiable,
    Selectable,
    Associations,
    Insertable,
    Serialize,
    Deserialize,
    Type,
)]
#[diesel(table_name = supported_sources)]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Source))]
#[diesel(primary_key(source_id, game_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SupportedSource {
    pub id: Option<i32>,
    pub is_supported: bool,
    pub source_id: i32,
    pub game_id: i32,
}

impl Source {
    pub fn mapping(&self) -> SourceMapping {
        Sources::from(self.clone()).into()
    }
}

sources!(
    Sources:
        SpaceDock = (0, "SpaceDock", "https://spacedock.info/api");
        Ckan = (1, "CKAN", "<ckandex>");
        Wormhole = (2, "Wormhole", "<TBD>");
        Local = (3, "Local", "<none>");
        CurseForge = (4, "CurseForge", "https://api.curseforge.com");
        Modrinth = (5, "Modrinth", "https://api.modrinth.com");
        Thunderstore = (6, "Thunderstore", "https://thunderstore.io/api");
        Nexus = (7, "Nexus Mods", "https://api.nexusmods.com");
        Unknown = (8, "Unknown", "<unknown>");
);
