#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

//! Wormhole's commands.
//! These are utility functions that are invoked directly with Tauri and the web UI.

#[macro_use]
extern crate tracing;

pub extern crate api;
pub extern crate macros as whmacros;
pub extern crate specta;
pub extern crate tauri;
pub extern crate whcore;

use data::diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::state::TState;

/// Wormhole's state container.
/// This is wrapped in a [`TState`] to work with [`specta`] and [`tauri_specta`].
pub type AppState<'a> = TState<'a, Pool<ConnectionManager<SqliteConnection>>>;

/// The result type.
pub type Result<T, E = String> = core::result::Result<T, E>;

pub mod base;
pub mod instance;
pub mod loader;
pub mod macros;
pub mod mods;
pub mod plugin;
