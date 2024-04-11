#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use api::{
    plugin::Plugin,
    plugins::{Kerbal1Plugin, Kerbal2Plugin, MinecraftPlugin},
    tauri::TauriPlugin,
};
use wormhole_gui::invoker;

pub fn main() -> Result<()> {
    let db = init::boot()?;

    tauri::Builder::default()
        .plugin(TauriPlugin::new(Kerbal1Plugin::new())?)
        .plugin(TauriPlugin::new(Kerbal2Plugin::new())?)
        .plugin(TauriPlugin::new(MinecraftPlugin::new())?)
        .manage(db)
        .invoke_handler(invoker())
        .run(tauri::generate_context!())
        .expect("Error while starting Wormhole!");

    Ok(())
}
