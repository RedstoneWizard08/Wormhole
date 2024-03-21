use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::plugin::Plugin;

lazy_static! {
    pub static ref PLUGINS: Arc<Mutex<HashMap<i32, Plugin>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn register_plugin(plugin: Plugin) {
    PLUGINS.lock().unwrap().insert(plugin.id, plugin);
}
