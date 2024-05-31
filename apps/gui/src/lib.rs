#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

//! # Wormhole GUI
//! 
//! Wormhole's Tauri-based GUI application.
//! This is also a library crate so it's API can be used elsewhere. This API will eventually
//! be moved into a separate crate.

mod cmd;
mod log;
mod macros;
mod run;

pub use cmd::*;
pub use run::run;

pub extern crate api;
pub extern crate macros as whmacros;
pub extern crate specta;
pub extern crate tauri;
pub extern crate tauri_specta;
pub extern crate tokio;
pub extern crate whcore;

#[macro_use]
extern crate tracing;

use api::install::progress::ProgressPayload;
use data::diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::state::TState;

/// Wormhole's state container.
/// This is wrapped in a [`TState`] to work with [`specta`] and [`tauri_specta`].
pub type AppState<'a> = TState<'a, Pool<ConnectionManager<SqliteConnection>>>;

commands![
    data, api, whcore, mcmeta;

    cmd::base::get_plugins,
    cmd::base::get_dirs,
    cmd::base::get_source_id,
    cmd::instance::get_instances,
    cmd::instance::get_instance,
    cmd::instance::add_instance,
    cmd::instance::delete_instance,
    cmd::instance::create_instance,
    cmd::instance::update_instance,
    cmd::plugin::info,
    cmd::plugin::search_mods,
    cmd::plugin::get_mod,
    cmd::plugin::get_mod_versions,
    cmd::plugin::get_mod_version,
    cmd::plugin::get_download_url,
    cmd::plugin::launch_game,
    cmd::plugin::sources,
    cmd::plugin::get_latest_version,
    cmd::mods::install_mod,
    cmd::mods::uninstall_mod,
    cmd::mods::get_mods,
    cmd::loader::get_loaders,
    cmd::loader::get_latest_loader,
    cmd::loader::install_loader;

    ProgressPayload
];
