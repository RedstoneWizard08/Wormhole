use std::path::PathBuf;

use anyhow::Result;
use data::migrate::migrate;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::get_data_dir;

pub mod db;

pub fn boot(path: Option<PathBuf>) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let db = db::connect(path.unwrap_or(get_data_dir().join("data.db")))?;

    migrate(&mut db.get()?)?;

    Ok(db)
}
