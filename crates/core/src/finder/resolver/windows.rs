use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_steam_library_folders_file() -> PathBuf {
    let binding = env::var("PROGRAMFILES(X86)").unwrap();
    let program_files = Path::new(binding.as_str());
    let config_dir = program_files.join("Steam").join("config");

    config_dir.join("libraryfolders.vdf")
}
