#![warn(missing_docs, rustdoc::broken_intra_doc_links)]
//! # Plugins for Wormhole.
//!
//! This implements all of Wormhole's plugins and game supports.

use crate::unity::{ContentWarningPlugin, Kerbal1Plugin, Kerbal2Plugin, LethalCompanyPlugin};
use api::{
    plugin::{Plugin, PluginExt},
    register::register_plugin,
};
use minecraft::MinecraftPlugin;

#[macro_use]
pub extern crate async_trait;

#[macro_use]
extern crate tracing;

pub extern crate api;
pub extern crate base64;
pub extern crate const_format;
pub extern crate query;
pub extern crate whcore;

pub mod macros;
pub mod minecraft;
pub mod unity;

/// Default plugins.
pub fn default_plugins() -> Vec<Box<dyn PluginExt>> {
    vec![
        Box::new(Kerbal1Plugin::new()),
        Box::new(Kerbal2Plugin::new()),
        Box::new(ContentWarningPlugin::new()),
        Box::new(LethalCompanyPlugin::new()),
        Box::new(MinecraftPlugin::new()),
    ]
}

/// Register default plugins.
pub async fn register_defaults() {
    for plugin in default_plugins() {
        register_plugin(plugin).await;
    }
}
