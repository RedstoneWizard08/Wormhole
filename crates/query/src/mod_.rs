use crate::source::ModSource;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Mod {
    /// The mod's ID in its source.
    /// This could be an integer or a string,
    /// and since we support multiple platforms,
    /// a string is the most flexible.
    pub id: String,

    /// The game ID.
    pub game_id: Option<i32>,

    /// The mod's versions.
    pub versions: Vec<ModVersion>,

    /// The mod's name.
    pub name: String,

    /// Where the mod came from.
    pub source: ModSource,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ModVersion {
    /// The version ID.
    pub id: String,

    /// The version name. Some sources may not have this.
    pub name: Option<String>,

    /// The file name.
    pub file_name: String,

    /// The size in bytes of the file.
    pub size: Option<String>,

    /// The SHA-512 hash of the file.
    pub hash: Option<String>,

    /// The download URL.
    pub url: String,
}
