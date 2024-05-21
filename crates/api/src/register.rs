use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::tauri::TauriPluginTrait;

lazy_static! {
    pub static ref PLUGINS: Arc<Mutex<HashMap<i32, Box<dyn TauriPluginTrait + Send + Sync>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub async fn register_plugin(plugin: Box<dyn TauriPluginTrait + Send + Sync>) {
    PLUGINS.lock().await.insert(plugin.game(), plugin);
}
