use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub const GENERATED_SUMMARY_LENGTH: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Project {
    /// The ID of the project to find it in its source.
    pub id: String,

    /// The source the project can be found in. See [`crate::models::QueryClient::id`].
    pub source_id: String,

    /// The name of the project.
    pub name: String,

    /// The slug (name in the URL) of the project.
    pub slug: Option<String>,

    /// The URL to find this project on the source's website.
    pub url: String,

    /// The project author. This account owns the project.
    pub author: String,

    /// The icon URL for this project.
    pub icon: Option<String>,

    /// An optional banner image for this project.
    pub banner: Option<String>,

    /// How many downloads this mod has.
    pub downloads: Option<u64>,

    /// How many followers this project has.
    pub follows: Option<u64>,

    /// A short-form description of this project.
    pub summary: Option<String>,

    /// The description of this project.
    pub description: Option<String>,

    /// A map of version IDs to version names and their publication dates for this project.
    pub versions: Option<HashMap<String, (String, DateTime<Utc>)>>,
}

impl Project {
    pub fn summary(&self) -> Option<String> {
        if let Some(summary) = &self.summary {
            Some(summary.clone())
        } else if let Some(desc) = &self.description {
            Some(desc.clone()[0..GENERATED_SUMMARY_LENGTH].to_string())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum ProjectKind {
    Mod,
    Modpack,
    Shader,
    ResourcePack,
    Datapack,
}
