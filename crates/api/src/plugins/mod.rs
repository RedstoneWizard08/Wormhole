pub mod ksp1;
pub mod ksp2;
pub mod minecraft;

pub use ksp1::Kerbal1Plugin;
pub use ksp2::Kerbal2Plugin;
pub use minecraft::MinecraftPlugin;

use crate::{plugin::Plugin, register::register_plugin};

pub fn default_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(Kerbal1Plugin::new()),
        Box::new(Kerbal2Plugin::new()),
        Box::new(MinecraftPlugin::new()),
    ]
}

pub fn register_defaults() {
    for plugin in default_plugins() {
        register_plugin(plugin);
    }
}
