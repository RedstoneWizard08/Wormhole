use plugin::PluginInfo;
use plugins::register_defaults;
use specta::{NamedType, TypeMap};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

pub extern crate whcore;

pub mod macros;
pub mod plugin;
pub mod plugins;
pub mod register;
pub mod res;
pub mod tauri;

#[cfg(test)]
pub mod test_util;

pub fn init() {
    register_defaults();
}

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = PluginInfo::named_data_type(&mut map, &[]);
    map.insert(PluginInfo::SID, ty);

    map
}
