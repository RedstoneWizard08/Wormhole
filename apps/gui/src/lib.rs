#![warn(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Wormhole GUI
//!
//! Wormhole's Tauri-based GUI application.
//! This is also a library crate so it's API can be used elsewhere. This API will eventually
//! be moved into a separate crate.

mod log;
mod run;

pub use run::run as run_app;

pub extern crate api;
pub extern crate commands;
pub extern crate specta;
pub extern crate tauri;
pub extern crate tokio;
pub extern crate whcore;

use anyhow::Result;

/// The Tauri entrypoint for the app.
#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    run_app().await
}
