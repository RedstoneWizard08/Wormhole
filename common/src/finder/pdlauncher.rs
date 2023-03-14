use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::{env, path::Path};

#[derive(Default)]
pub struct PDLauncherInstallFinder {}

impl PDLauncherInstallFinder {
    #[cfg(target_os = "windows")]
    pub fn find_ksp2_dir(&self) -> Option<PathBuf> {
        let default_install_folder = Path::new(env::var("PROGRAMFILES").unwrap().as_str())
            .join("Private Division")
            .join("Kerbal Space Program 2");

        if default_install_folder.exists() {
            return Some(default_install_folder.to_path_buf());
        }

        return None;
    }

    #[cfg(not(target_os = "windows"))]
    pub fn find_ksp2_dir(&self) -> Option<PathBuf> {
        return None;
    }
}
