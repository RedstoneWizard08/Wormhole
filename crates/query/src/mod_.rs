#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type, Default,
)]
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
    /// This is a reference to a source in the database.
    pub source: i32,

    /// The mod's icon.
    pub icon: Option<String>,

    /// The mod's banner.
    pub banner: Option<String>,

    /// The mod's description.
    pub desc: Option<String>,

    /// The mod's author.
    pub author: Option<String>,

    /// The mod's downloads.
    pub downloads: u64,

    /// The mod's followers.
    pub followers: u64,
}

unsafe impl Send for Mod {}
unsafe impl Sync for Mod {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct ModVersion {
    /// The version ID.
    pub id: String,

    /// The version name. Some sources may not have this.
    pub name: Option<String>,

    /// The file name.
    pub file_name: Option<String>,

    /// The size in bytes of the file.
    pub size: Option<String>,

    /// The SHA-512 hash of the file.
    pub hash: Option<String>,

    /// The download URL.
    pub url: Option<String>,
}

unsafe impl Send for ModVersion {}
unsafe impl Sync for ModVersion {}

impl ModVersion {
    pub fn file_name(&self) -> String {
        self.file_name.clone().unwrap_or_else(|| {
            self.url
                .clone()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_string()
        })
    }
}
