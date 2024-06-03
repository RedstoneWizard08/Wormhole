pub mod config;
pub mod dirs;
pub mod finder;
pub mod manager;
pub mod messaging;
pub mod progress;
pub mod state;
pub mod traits;
pub mod fs_util;
pub mod errors;
pub mod typing;
pub mod util;

#[macro_use]
extern crate serde;

#[macro_use]
pub extern crate async_trait;

use dirs::Dirs;
use specta::{NamedType, TypeMap};

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = Dirs::named_data_type(&mut map, &[]);
    map.insert(Dirs::SID, ty);

    map
}
