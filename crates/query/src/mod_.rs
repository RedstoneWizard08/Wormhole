use crate::source::ModSource;

#[derive(Debug, Clone)]
pub struct Mod {
    /// The mod's ID in its source.
    /// This could be an integer or a string,
    /// and since we support multiple platforms,
    /// a string is the most flexible.
    pub id: String,

    /// The mod's version ID in its source.
    pub version_id: Option<String>,

    /// The mod's name.
    pub name: String,

    /// The file name.
    pub file_name: String,

    /// Where the mod came from.
    pub source: ModSource,

    /// The size in bytes of the mod.
    pub size: usize,

    /// The SHA-512 hash of the mod.
    pub hash: String,
}
