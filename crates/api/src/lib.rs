#![warn(missing_docs, rustdoc::broken_intra_doc_links)]
//! # Wormhole's API.
//!
//! This implements the plugin structure, mod installers,
//! game supports, and a lot more.
//!
//! In the future, this will hopefully be able to support
//! WASI-based plugins for extensibility.

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

#[macro_use]
extern crate async_trait;

pub extern crate whcore;

pub mod install;
pub mod macros;
pub mod plugin;
pub mod register;
pub mod res;
pub mod unity;
