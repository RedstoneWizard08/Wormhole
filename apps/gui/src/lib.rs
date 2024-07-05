#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

//! # Wormhole GUI
//!
//! Wormhole's Tauri-based GUI application.
//! This is also a library crate so it's API can be used elsewhere. This API will eventually
//! be moved into a separate crate.

mod log;
mod run;

pub use run::run;

pub extern crate api;
pub extern crate commands;
pub extern crate specta;
pub extern crate tauri;
pub extern crate tokio;
pub extern crate whcore;
