use cmd::modded::{ModLoader, ModLoaderType};
use specta::{NamedType, TypeMap};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

#[macro_use]
extern crate tracing;

pub extern crate anyhow;

pub mod cmd;
pub mod download;
pub mod fabric;
pub mod forge;
pub mod jar_mf;
pub mod launchwrapper;
pub mod macros;
pub mod manager;
pub mod maven;
pub mod neoforge;
pub mod piston;
pub mod quilt;
pub mod util;

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = ModLoader::named_data_type(&mut map, &[]);
    map.insert(ModLoader::sid(), ty);

    let ty = ModLoaderType::named_data_type(&mut map, &[]);
    map.insert(ModLoaderType::sid(), ty);

    map
}
