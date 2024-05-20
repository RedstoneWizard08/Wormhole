// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub extern crate diesel;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

use diesel::sqlite::SqliteConnection;
use instance::Instance;
use specta::{NamedType, TypeMap};

use crate::{
    game::Game,
    mod_::Mod,
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

    let ty = Mod::named_data_type(&mut map, &[]);
    map.insert(Mod::SID, ty);

    map
}
