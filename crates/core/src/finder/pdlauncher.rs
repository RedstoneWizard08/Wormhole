use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::{env, path::Path};

#[derive(Default)]
pub struct PDLauncherInstallFinder;

impl PDLauncherInstallFinder {
    #[cfg(windows)]
    pub fn games_dir() -> Option<PathBuf> {
        Some(Path::new(env::var("PROGRAMFILES").unwrap().as_str()).join("Private Division"))
    }

    #[cfg(not(target_os = "windows"))]
    pub fn games_dir() -> Option<PathBuf> {
        None
    }
}
