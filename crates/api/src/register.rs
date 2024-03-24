use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::plugin::Plugin;

lazy_static! {
    pub static ref PLUGINS: Arc<Mutex<HashMap<i32, Box<dyn Plugin>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn register_plugin(plugin: Box<dyn Plugin>) {
    PLUGINS.lock().unwrap().insert(plugin.game(), plugin);
}
