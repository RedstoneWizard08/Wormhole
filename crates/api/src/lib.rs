use std::sync::Arc;

use ::tauri::AppHandle;
use crossbeam_channel::{unbounded, Receiver, Sender};
use install::progress::ProgressPayload;
use plugin::PluginInfo;
use plugins::register_defaults;
use specta::{NamedType, TypeMap};
use tokio::sync::Mutex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate async_trait;

pub extern crate whcore;

pub mod install;
pub mod macros;
pub mod plugin;
pub mod plugins;
pub mod register;
pub mod res;
pub mod tauri;

#[cfg(test)]
pub mod test_util;

// This isn't just lazily initialized, it's also "lazy" because
// I'm too lazy to actually pass around a struct.
lazy_static! {
    pub static ref TAURI_HANDLE: Arc<Mutex<Option<AppHandle>>> = Arc::new(Mutex::new(None));
    pub static ref EVENT_BUS: Arc<(Sender<ProgressPayload>, Receiver<ProgressPayload>)> =
        Arc::new(unbounded());
}

pub async fn init() {
    register_defaults().await;
}

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = PluginInfo::named_data_type(&mut map, &[]);
    map.insert(PluginInfo::SID, ty);

    let ty = ProgressPayload::named_data_type(&mut map, &[]);
    map.insert(ProgressPayload::SID, ty);

    map
}
