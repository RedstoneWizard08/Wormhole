use std::{cell::LazyCell, fs, path::PathBuf};

use dirs::data_dir;
use specta::Type;

pub const INSTANCE: LazyCell<CoreManager> = LazyCell::new(CoreManager::new);

#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
pub struct CoreManager {
    root: PathBuf,
}

impl CoreManager {
    pub fn new() -> Self {
        Self {
            root: data_dir().unwrap().join("Wormhole"),
        }
    }

    pub fn get() -> Self {
        INSTANCE.clone()
    }

    pub fn games(&self) -> PathBuf {
        self.root.join("games")
    }

    pub fn game_data_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.games().join(game.as_ref())
    }

    pub fn create_dir(&self, dir: impl Into<PathBuf>) {
        fs::create_dir_all(dir.into()).unwrap();
    }

    pub fn dir(&self, dir: impl AsRef<str>) -> PathBuf {
        self.root.join(dir.as_ref())
    }
}
