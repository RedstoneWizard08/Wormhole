use super::{info::SharedAuthor, version::ModVersion};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BrowseModInfo {
    pub name: String,
    pub id: i32,
    pub game: String,
    pub game_id: i32,
    pub short_description: String,
    pub downloads: i32,
    pub followers: i32,
    pub author: String,
    pub default_version_id: i32,
    pub shared_authors: Vec<SharedAuthor>,
    pub background: String,
    pub bg_offset_y: String,
    pub license: String,
    pub website: String,
    pub donations: String,
    pub source_code: String,
    pub url: String,
    pub versions: Vec<ModVersion>,
}
