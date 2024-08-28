#[macro_use]
pub extern crate serde;

#[macro_use]
pub extern crate async_trait;

pub extern crate specta;

pub mod macros;
pub mod manager;
pub mod traits;

pub use manager::CoreManager;
pub use traits::*;

type_map! {
    CoreManager,
}
