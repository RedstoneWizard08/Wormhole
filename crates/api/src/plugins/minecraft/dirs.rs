use std::path::PathBuf;

use whcore::manager::CoreManager;

#[derive(Debug, Clone)]
pub struct MinecraftDirs {
    pub libs: PathBuf,
    pub natives: PathBuf,
    pub temp: PathBuf,
    pub java: PathBuf,
    pub assets_base: PathBuf,
}

impl MinecraftDirs {
    pub fn collect() -> Self {
        Self {
            libs: CoreManager::get()
                .game_data_dir("minecraft")
                .join("libraries"),
            natives: CoreManager::get()
                .game_data_dir("minecraft")
                .join("natives"),
            temp: CoreManager::get().game_data_dir("minecraft").join("temp"),
            java: CoreManager::get().game_data_dir("java"),
            assets_base: CoreManager::get().game_data_dir("minecraft").join("assets"),
        }
    }

    pub fn assets(&self, version: impl AsRef<str>) -> PathBuf {
        self.assets_base.join(version.as_ref())
    }
}

impl Default for MinecraftDirs {
    fn default() -> Self {
        Self::collect()
    }
}
