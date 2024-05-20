use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::tauri::TauriPluginTrait;

lazy_static! {
    pub static ref PLUGINS: Arc<Mutex<HashMap<i32, Box<dyn TauriPluginTrait + Send + Sync>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn register_plugin(plugin: Box<dyn TauriPluginTrait + Send + Sync>) {
    PLUGINS.lock().unwrap().insert(plugin.game(), plugin);
}
