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

pub mod base;
pub mod instance;
pub mod loader;
pub mod mods;
pub mod router;
