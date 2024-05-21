pub mod common;
pub mod ksp1;
pub mod ksp2;
pub mod minecraft;

pub use ksp1::Kerbal1Plugin;
pub use ksp2::Kerbal2Plugin;
pub use minecraft::MinecraftPlugin;

use crate::{plugin::Plugin, register::register_plugin, tauri::TauriPluginTrait};

pub fn default_plugins() -> Vec<Box<dyn TauriPluginTrait>> {
    vec![
        Box::new(Kerbal1Plugin::new()),
        Box::new(Kerbal2Plugin::new()),
        Box::new(MinecraftPlugin::new()),
    ]
}

pub async fn register_defaults() {
    for plugin in default_plugins() {
        register_plugin(plugin).await;
    }
}
