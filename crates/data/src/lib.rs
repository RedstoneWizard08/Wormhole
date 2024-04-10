pub extern crate diesel;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

use diesel::sqlite::SqliteConnection;

pub mod conv;
pub mod game;
pub mod instance;
pub mod macros;
pub mod mod_;
pub mod schema;
pub mod source;

pub type Conn = SqliteConnection;
