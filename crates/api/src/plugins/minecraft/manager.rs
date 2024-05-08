use std::{fs, path::PathBuf};

use anyhow::Result;
use mcmeta::cmd::modded::ModLoader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftManager {
    pub dir: PathBuf,
    pub loader: ModLoader,
}

impl MinecraftManager {
    pub fn load(dir: PathBuf) -> Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(
            dir.join("metadata.json"),
        )?)?)
    }
}
