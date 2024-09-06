use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::mod_structs::Mod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub game_id: i32,
    pub class_id: Option<i32>,
    pub category_id: Option<i32>,
    pub category_ids: Option<Vec<i32>>,
    pub game_version: Option<String>,
    pub game_versions: Option<Vec<String>>,
    pub search_filter: Option<String>,
    pub sort_field: Option<ModsSearchSortField>,
    pub sort_order: Option<SortOrder>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub mod_loader_types: Option<Vec<ModLoaderType>>,
    pub game_version_type_id: Option<i32>,
    pub author_id: Option<i32>,
    pub primary_author_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    /// The maximum is 50.
    pub page_size: Option<i32>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModsSearchSortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
    EarlyAccess = 9,
    FeaturedReleased = 10,
    ReleasedDate = 11,
    Rating = 12,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,

    #[serde(rename = "desc")]
    Descending,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub pagination: Pagination,
    pub data: Vec<Mod>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub index: i32,
    pub page_size: i32,
    pub result_count: i32,
    pub total_count: i32,
}
