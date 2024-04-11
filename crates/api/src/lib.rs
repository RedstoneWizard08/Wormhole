use plugins::register_defaults;

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

pub fn init() {
    register_defaults();
}
