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

use anyhow::Result;
use api::plugins::register_defaults;
use data::migrate::migrate;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::manager::CoreManager;

pub mod db;

pub static mut INIT: bool = false;

pub fn boot(path: &Option<PathBuf>) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    unsafe {
        if INIT {
            return Err(anyhow::anyhow!("Already initialized"));
        }

        INIT = true;
    }

    CoreManager::get().init();
    
    let db = db::connect(
        path.clone()
            .unwrap_or(CoreManager::get().data_dir().join("data.db")),
    )?;

    migrate(&mut db.get()?)?;
    register_defaults();

    Ok(db)
}
