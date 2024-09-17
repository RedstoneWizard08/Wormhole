use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::models::pack::Pack;

impl Pack {
    pub fn save(&self, dir: Option<PathBuf>) -> Result<()> {
        let data = toml::to_string_pretty(self)?;

        fs::write(dir.unwrap_or(PathBuf::from(".")).join("pack.toml"), data)?;

        Ok(())
    }
}
