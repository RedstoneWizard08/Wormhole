#![feature(associated_type_defaults)]
#![allow(unused_imports)]

#[macro_use]
extern crate async_trait;

#[macro_use]
pub extern crate anyhow;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

pub(crate) const CURSEFORGE_KEY: &str = env!("CURSEFORGE_KEY");
pub(crate) const MODRINTH_KEY: Option<&str> = option_env!("MODRINTH_KEY");

/// This backend is currently not implemented.
pub mod nexus;

pub mod ckan;
pub mod conv;
pub mod curse;
pub mod macros;
pub mod mod_;
pub mod modrinth;
pub mod source;
pub mod spacedock;
pub mod thunderstore;

#[async_trait]
pub trait IntoAsync<T> {
    async fn into_async(self) -> T;
}
