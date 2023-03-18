use wormhole_common::{
    finder::find_install_dir, installer::mods::ModInstaller, instances::KSPGame,
};

pub async fn install_mod(id: i32, verbose: bool) {
    if verbose {
        println!("Creating mod installer...");
    }

    let installer = ModInstaller::new(find_install_dir(KSPGame::KSP2));

    if verbose {
        println!("Installing mod...");
    }

    installer.install_from_spacedock(id).await;

    if verbose {
        println!("Mod installed!");
    }
}
