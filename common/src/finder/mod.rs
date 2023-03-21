use std::path::PathBuf;

use pdlauncher::PDLauncherInstallFinder;
use steam::SteamInstallFinder;

use crate::instances::KSPGame;

pub mod pdlauncher;
pub mod resolver;
pub mod steam;

pub fn find_ksp2_install_dir() -> PathBuf {
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

pub fn find_ksp1_install_dir() -> PathBuf {
    let mut steam_install_finder = SteamInstallFinder::default();
    let pdlauncher_install_finder = PDLauncherInstallFinder::default();

    let steam_install_dir = steam_install_finder.find_ksp1_dir();
    let pdlauncher_install_dir = pdlauncher_install_finder.find_ksp1_dir();

    if let Some(dir) = steam_install_dir {
        return dir;
    }

    if let Some(dir) = pdlauncher_install_dir {
        return dir;
    }

    return None.expect("No KSP1 install found!");
}

pub fn find_install_dir(game: KSPGame) -> PathBuf {
    return match game {
        KSPGame::KSP1 => find_ksp1_install_dir(),
        KSPGame::KSP2 => find_ksp2_install_dir(),
    };
}
