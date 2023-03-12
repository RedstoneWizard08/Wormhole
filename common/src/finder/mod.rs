use std::{fs, path::PathBuf};

use pdlauncher::PDLauncherInstallFinder;
use steam::SteamInstallFinder;

pub mod pdlauncher;
pub mod resolver;
pub mod steam;

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

    let p = PathBuf::from(
        "/home/jacob/.local/share/steam/root/steamapps/common/Kerbal Space Program 2",
    );

    fs::create_dir_all(p.clone()).unwrap();

    return p;
    // return None.expect("No KSP2 install found!");
}
