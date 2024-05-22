pub extern crate diesel;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

use diesel::sqlite::SqliteConnection;
use instance::Instance;
use mod_::DbMod;
use specta::{NamedType, TypeMap};

use crate::{
    game::Game,
    // mod_::Mod,
    source::{Source, Sources, SupportedSource},
};

pub mod conv;
pub mod game;
pub mod instance;
pub mod macros;
pub mod migrate;
pub mod mod_;
pub mod schema;
pub mod source;

pub type Conn = SqliteConnection;

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = Instance::named_data_type(&mut map, &[]);
    map.insert(Instance::SID, ty);

    let ty = Game::named_data_type(&mut map, &[]);
    map.insert(Game::SID, ty);

    let ty = Source::named_data_type(&mut map, &[]);
    map.insert(Source::SID, ty);

    let ty = Sources::named_data_type(&mut map, &[]);
    map.insert(Sources::SID, ty);

    let ty = SupportedSource::named_data_type(&mut map, &[]);
    map.insert(SupportedSource::SID, ty);

    let ty = DbMod::named_data_type(&mut map, &[]);
    map.insert(DbMod::SID, ty);

    map
}
