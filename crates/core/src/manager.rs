use std::{fs, path::PathBuf};

use dirs::data_local_dir;

pub const WORMHOLE_DIR_NAME: &str = "Wormhole";
pub const CORE_MANAGER: CoreManager = CoreManager::new();

pub struct CoreManager;

impl CoreManager {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self) {
        self.create_dirs();
    }

    fn create_dirs(&self) {
        self.create_dir(self.dir());
        self.create_dir(self.data_dir());
        self.create_dir(self.cache_dir());
        self.create_dir(self.temp_dir());
    }

    fn create_dir(&self, path: PathBuf) {
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
    }

    pub fn dir(&self) -> PathBuf {
        data_local_dir().unwrap().join(WORMHOLE_DIR_NAME)
    }

    pub fn data_dir(&self) -> PathBuf {
        self.dir().join("data")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.dir().join("cache")
    }

    pub fn temp_dir(&self) -> PathBuf {
        self.dir().join("temp")
    }

    pub fn game_data_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.data_dir().join(game.as_ref())
    }

    pub fn game_cache_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.cache_dir().join(game.as_ref())
    }

    pub fn game_temp_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.temp_dir().join(game.as_ref())
    }
}
