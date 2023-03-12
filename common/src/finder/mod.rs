use std::{fs, path::PathBuf};

pub mod pdlauncher;
pub mod resolver;
pub mod steam;

use pdlauncher::PDLauncherInstallFinder;
use steam::SteamInstallFinder;

pub fn find_install_dir() -> PathBuf {
    let mut steam_install_finder = SteamInstallFinder::default();
    let pdlauncher_install_finder = PDLauncherInstallFinder::default();

    let steam_install_dir = steam_install_finder.find_ksp2_dir();
    let pdlauncher_install_dir = pdlauncher_install_finder.find_ksp2_dir();

    if let Some(dir) = steam_install_dir {
        return dir;
    }

    if let Some(dir) = pdlauncher_install_dir {
        return dir;
    }

    return None.expect("No KSP2 install found!");
}
