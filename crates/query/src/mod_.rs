#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ModSource {
    SpaceDock = 0,
    Ckan = 1,
    Wormhole = 2,
    Local = 3,
    CurseForge = 4,
    Modrinth = 5,
    Thunderstore = 6,
    Nexus = 7,

    #[default]
    Unknown = 8,
}

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
