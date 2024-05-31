//! Happily registering plugins since 2024.

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::tauri::TauriPluginTrait;

lazy_static! {
    /// A map of game IDs to their plugin.
    /// This is where plugins are registered.
    pub static ref PLUGINS: Arc<Mutex<HashMap<i32, Box<dyn TauriPluginTrait + Send + Sync>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// Registers a plugin to the map.
pub async fn register_plugin(plugin: Box<dyn TauriPluginTrait + Send + Sync>) {
    PLUGINS.lock().await.insert(plugin.game(), plugin);
}
