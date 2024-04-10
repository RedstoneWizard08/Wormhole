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
