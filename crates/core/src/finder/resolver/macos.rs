use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_steam_library_folders_file() -> PathBuf {
    let binding = env::var("HOME").unwrap();
    let home_dir = Path::new(binding.as_str());

    let config_dir = home_dir
        .join("Library")
        .join("Application Support")
        .join("Steam")
        .join("config");

    config_dir.join("libraryfolders.vdf")
}
