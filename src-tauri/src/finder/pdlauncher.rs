use std::{
    env,
    path::{Path, PathBuf},
};

pub struct PDLauncherInstallFinder {}

impl PDLauncherInstallFinder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_ksp2_dir(&self) -> Option<PathBuf> {
        let default_install_folder = Path::new(env::var("ProgramFiles").unwrap().as_str())
            .join("Private Division")
            .join("Kerbal Space Program 2");

        if default_install_folder.exists() {
            return Some(default_install_folder.to_path_buf());
        }

        return None;
    }
}
