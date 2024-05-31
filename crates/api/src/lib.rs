#![warn(missing_docs, rustdoc::broken_intra_doc_links)]
//! # Wormhole's API.
//! 
//! This implements all of the plugins, mod installers,
//! game supports, assets, and a lot more.
//! 
//! In the future, this will hopefully be able to support
//! WASI-based plugins for extensibility.

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

// This isn't just lazily initialized, it's also "lazy" because
// I'm too lazy to actually pass around a struct.
lazy_static! {
    /// A handle to the Tauri app.
    /// This will be [`None`] if the app hasn't been started yet, or
    /// this is running in the web UI.
    pub static ref TAURI_HANDLE: Arc<Mutex<Option<AppHandle>>> = Arc::new(Mutex::new(None));

    /// The "event bus". This is really just an unbounded channel
    /// of [`ProgressPayload`]s. This will eventually be expanded
    /// to support more event types.
    pub static ref EVENT_BUS: Arc<(Sender<ProgressPayload>, Receiver<ProgressPayload>)> =
        Arc::new(unbounded());
}

/// Initializes the API, registering all of the default plugins.
pub async fn init() {
    register_defaults().await;
}

/// Gets the [`TypeMap`] for the API.
pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = PluginInfo::named_data_type(&mut map, &[]);
    map.insert(PluginInfo::SID, ty);

    let ty = ProgressPayload::named_data_type(&mut map, &[]);
    map.insert(ProgressPayload::SID, ty);

    map
}
