//! The hub for all of Wormhole's API clients.
//!
//! Each client has the following functions:
//! - Searching projects (filtered based on game versions)
//! - Getting individual projects
//! - Getting project versions (filtered based on game versions)
//! - Getting individual project versions
//! - Getting links to directly download files
//! - Getting project dependencies
//!
//! Each client will have a context (<T>) that the game provider needs to
//! fulfill, as each clients' methods will be duplicated on the game
//! provider's end, with the extra argument of which one to use. This
//! allows games like Minecraft to provide version information, while Unity
//! games won't, as BepInEx and MelonLoader mods are mostly universal.

use models::project::Project;
use once_cell::sync::Lazy;
use whcore::type_map;

pub mod conversions;
pub mod models;
pub mod modrinth;

#[macro_use]
extern crate envcrypt;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

#[allow(unused)]
pub(crate) const CURSEFORGE_KEY: Lazy<&str> = Lazy::new(|| envc!("CURSEFORGE_KEY"));

#[allow(unused)]
pub(crate) const MODRINTH_KEY: Lazy<Option<&str>> = Lazy::new(|| option_envc!("MODRINTH_KEY"));

#[allow(unused)]
pub(crate) const NEXUS_API_KEY: Lazy<&str> = Lazy::new(|| envc!("NEXUS_API_KEY"));

type_map! {
    Project,
}
