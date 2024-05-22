use anyhow::Result;
use std::{collections::HashMap, fs, path::PathBuf};

use super::{finder::InstallFinder, library::LibraryFolders};

pub struct Steam {
    pub library_folders: LibraryFolders,
}

impl Default for Steam {
    fn default() -> Self {
        let mut me = Self {
            library_folders: LibraryFolders::new(),
        };

        me.library_folders
            .discover()
            .expect("Failed to discover Steam library folders!");

        me
    }
}

impl InstallFinder for Steam {
    fn new() -> Self {
        Self::default()
    }

    fn find_game(&self, game: impl AsRef<str>) -> Result<Option<PathBuf>> {
        Ok(self.discover()?.get(game.as_ref()).cloned())
    }

    fn discover(&self) -> Result<HashMap<String, PathBuf>> {
        let mut games = HashMap::new();

        for folder in self.library_folders.clone().paths {
            let items = fs::read_dir(folder.join("common"))?;

            for item in items.flatten() {
                if item.path().is_dir() {
                    games.insert(item.file_name().into_string().unwrap(), item.path());
                }
            }
        }

        Ok(games)
    }
}
