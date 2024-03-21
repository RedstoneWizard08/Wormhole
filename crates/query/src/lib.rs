#![feature(associated_type_defaults)]

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate anyhow;

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

/// This backend is currently not implemented.
pub mod thunderstore;

pub mod mod_;
pub mod source;
pub mod spacedock;
