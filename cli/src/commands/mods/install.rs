use wormhole_common::{installer::mods::ModInstaller, finder::find_install_dir};

pub async fn install_mod(id: i32, verbose: bool) {
    if verbose {
        println!("Creating mod installer...");
    }

    let installer = ModInstaller::new(find_install_dir());

    if verbose {
        println!("Installing mod...");
    }

    installer.install_from_spacedock(id).await;

    if verbose {
        println!("Mod installed!");
    }
}
