use std::{path::PathBuf, time::Instant};

use query::mod_::Mod;

#[derive(Debug, Clone)]
pub struct InstanceMeta {
    /// The game's ID in Wormhole's database.
    pub game: i32,

    /// The directory where the instance's data is stored.
    pub data_dir: PathBuf,

    /// The cache directory.
    pub cache_dir: PathBuf,

    /// The instance's user-editable description.
    pub description: String,

    /// When the instance was created.
    pub created: Instant,

    /// When the instance was last modified.
    pub updated: Instant,
}

#[derive(Debug, Clone)]
pub struct Instance {
    /// The instance's ID.
    pub id: i32,

    /// The instance's name.
    pub name: String,

    /// The instance's metadata.
    pub meta: InstanceMeta,

    /// The mods in the instance.
    pub mods: Vec<Mod>,
}
