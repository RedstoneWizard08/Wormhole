use clap::ValueEnum;
use std::collections::HashMap;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Type,
    ValueEnum,
    Default,
)]
pub enum Side {
    Client,
    Server,
    #[default]
    Both,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Type,
    ValueEnum,
    Default,
)]
pub enum ListFormat {
    Markdown,
    Txt,
    #[default]
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PackMeta {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PackLoader {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ExportFormat {
    pub side: Side,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IndexInfo {
    pub path: String,
    pub sha1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Pack {
    pub meta: PackMeta,
    pub index: IndexInfo,
    pub versions: HashMap<String, Vec<String>>,
    pub loaders: Vec<PackLoader>,
    pub export: Vec<ExportFormat>,
}

impl Default for IndexInfo {
    fn default() -> Self {
        Self {
            path: "index.lock".into(),
            sha1: "<unknown>".into(),
        }
    }
}
