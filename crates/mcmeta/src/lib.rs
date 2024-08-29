#![feature(path_add_extension)]

use fabric::{meta::FabricMetaVersion, Fabric};
use forge::Forge;
use maven::{
    artifact::Artifact,
    metadata::{Metadata, Versioning, Versions},
    MavenRepo,
};
use neoforge::{
    meta::{ReposiliteVersion, ReposiliteVersions},
    version::NeoForgeVersion,
    NeoForge,
};
use quilt::Quilt;
use vanilla::Vanilla;
use whcore::type_map;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

pub extern crate anyhow;

pub mod fabric;
pub mod forge;
pub mod installer;
pub mod loader;
pub mod maven;
pub mod neoforge;
pub mod quilt;
pub mod util;
pub mod vanilla;

#[cfg(test)]
pub mod tests;

type_map! {
    MavenRepo,
    Artifact,
    Metadata,
    Versioning,
    Versions,
    Forge,
    NeoForge,
    NeoForgeVersion,
    ReposiliteVersions,
    ReposiliteVersion,
    FabricMetaVersion,
    Fabric,
    Quilt,
    Vanilla,
}
