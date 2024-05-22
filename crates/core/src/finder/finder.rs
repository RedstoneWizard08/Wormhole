use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

pub trait InstallFinder {
    fn new() -> Self
    where
        Self: Sized;

    fn find_game(&self, game: impl AsRef<str>) -> Result<Option<PathBuf>>;
    fn discover(&self) -> Result<HashMap<String, PathBuf>>;

    fn chain(&self, game: impl AsRef<str>, other: impl InstallFinder) -> Result<Option<PathBuf>> {
        if let Some(dir) = self.find_game(&game)? {
            Ok(Some(dir))
        } else {
            Ok(other.find_game(game)?)
        }
    }
}
