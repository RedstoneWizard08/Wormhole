use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestSchema {
    pub friendly_version: String,
    pub game_version: String,
    pub id: i32,
    pub created: String,
    pub download_path: String,
    pub changelog: Option<String>,
    pub downloads: i32,
}
