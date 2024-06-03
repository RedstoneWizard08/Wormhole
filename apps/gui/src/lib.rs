#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

//! # Wormhole GUI
//!
//! Wormhole's Tauri-based GUI application.
//! This is also a library crate so it's API can be used elsewhere. This API will eventually
//! be moved into a separate crate.

mod log;
mod macros;
mod run;

pub use run::run;

pub extern crate api;
pub extern crate commands;
pub extern crate macros as whmacros;
pub extern crate specta;
pub extern crate tauri;
pub extern crate tauri_specta;
pub extern crate tokio;
pub extern crate whcore;

use api::install::progress::ProgressPayload;

commands![
    data, api, whcore, mcmeta;

    commands::base::get_plugins,
    commands::base::get_dirs,
    commands::base::get_source_id,
    commands::instance::get_instances,
    commands::instance::get_instance,
    commands::instance::add_instance,
    commands::instance::delete_instance,
    commands::instance::create_instance,
    commands::instance::update_instance,
    commands::plugin::info,
    commands::plugin::search_mods,
    commands::plugin::get_mod,
    commands::plugin::get_mod_versions,
    commands::plugin::get_mod_version,
    commands::plugin::get_download_url,
    commands::plugin::launch_game,
    commands::plugin::sources,
    commands::plugin::get_latest_version,
    commands::mods::install_mod,
    commands::mods::uninstall_mod,
    commands::mods::get_mods,
    commands::loader::get_loaders,
    commands::loader::get_latest_loader,
    commands::loader::install_loader;

    ProgressPayload
];
