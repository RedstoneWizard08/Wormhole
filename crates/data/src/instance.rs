// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::path::PathBuf;

use crate::schema::instances;
use diesel::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Queryable,
    Identifiable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Type,
)]
#[diesel(table_name = instances)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Instance {
    pub id: Option<i32>,
    pub name: String,
    pub game_id: i32,
    pub data_dir: String,
    pub cache_dir: String,
    pub install_dir: String,
    pub description: String,
    pub created: i64,
    pub updated: i64,
}

impl Instance {
    pub fn data_dir(&self) -> PathBuf {
        PathBuf::from(self.data_dir.clone())
    }

    pub fn cache_dir(&self) -> PathBuf {
        PathBuf::from(self.cache_dir.clone())
    }

    pub fn install_dir(&self) -> PathBuf {
        PathBuf::from(self.install_dir.clone())
    }
}
