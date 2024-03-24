#![feature(associated_type_defaults)]
#![allow(unused_imports)]

#[macro_use]
extern crate async_trait;

#[macro_use]
pub extern crate anyhow;

#[macro_use]
extern crate serde;

/// This backend is currently not implemented.
pub mod ckan;

/// This backend is currently not implemented.
pub mod curse;

/// This backend is currently not implemented.
pub mod modrinth;

/// This backend is currently not implemented.
pub mod nexus;

pub mod macros;
pub mod mod_;
pub mod source;
pub mod spacedock;
/// This backend is currently not implemented.
pub mod thunderstore;
