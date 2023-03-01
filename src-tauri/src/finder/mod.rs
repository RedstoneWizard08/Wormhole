use std::path::PathBuf;

pub mod pdlauncher;
pub mod steam;

pub fn find_install_dir() -> PathBuf {
    let mut steam_install_finder = steam::SteamInstallFinder::new();
    let pdlauncher_install_finder = pdlauncher::PDLauncherInstallFinder::new();

    let steam_install_dir = steam_install_finder.find_ksp2_dir();
    let pdlauncher_install_dir = pdlauncher_install_finder.find_ksp2_dir();

    if steam_install_dir.is_some() {
        return steam_install_dir.unwrap();
    }

    if pdlauncher_install_dir.is_some() {
        return pdlauncher_install_dir.unwrap();
    }

    return None.expect("No KSP2 install found!");
}
