use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};

use crate::models::pack::Pack;

impl Pack {
    pub fn load(dir: Option<PathBuf>) -> Result<Self> {
        let path = dir.unwrap_or(PathBuf::from(".")).join("pack.toml");

        if path.exists() {
            Ok(toml::from_str(&fs::read_to_string(path)?)?)
        } else {
            Err(anyhow!("Could not find a pack.toml file!"))
        }
    }
}
