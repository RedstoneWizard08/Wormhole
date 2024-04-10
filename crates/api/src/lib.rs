use plugins::register_defaults;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate async_trait;

pub extern crate whcore;

pub mod plugin;
pub mod plugins;
pub mod register;
pub mod res;

pub fn init() {
    register_defaults();
}
