use std::{collections::HashMap, path::PathBuf};

#[cfg(target_os = "windows")]
use std::{env, path::Path};

use anyhow::Result;

use super::finder::InstallFinder;

#[derive(Default)]
pub struct PrivateDivision;

impl PrivateDivision {
    #[cfg(windows)]
    pub fn games_dir() -> Option<PathBuf> {
        Some(Path::new(env::var("PROGRAMFILES").unwrap().as_str()).join("Private Division"))
    }

    #[cfg(not(target_os = "windows"))]
    pub fn games_dir() -> Option<PathBuf> {
        None
    }
}

impl InstallFinder for PrivateDivision {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    // I don't know how the PD launcher works, sorry :(
    // If someone wants to contribute, please do.
    fn discover(&self) -> Result<HashMap<String, PathBuf>> {
        Ok(HashMap::new())
    }

    fn find_game(&self, _game: impl AsRef<str>) -> Result<Option<PathBuf>> {
        Ok(None)
    }
}
