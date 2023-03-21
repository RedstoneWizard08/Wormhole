use wormhole_common::{installer::{spacedock::SpaceDockModInstaller, mods::ModInstaller}, instances::Instance};

pub async fn install_mod(id: i32, instance_id: i32, verbose: bool) {
    if verbose {
        println!("Creating mod installer...");
    }

    let instance = Instance::from_id(instance_id).unwrap();
    let installer = SpaceDockModInstaller::new(instance.install_path);

    if verbose {
        println!("Installing mod...");
    }

    installer.install(id, instance_id).await;

    if verbose {
        println!("Mod installed!");
    }
}
