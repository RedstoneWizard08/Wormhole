use std::collections::HashMap;

use super::rules::Rule;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LibraryFile {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryFile>,
    #[serde(default)]
    pub classifiers: HashMap<String, LibraryFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    #[serde(default)]
    pub rules: Vec<Rule>,
}
