use std::path::PathBuf;

use anyhow::Result;
use api::plugins::register_defaults;
use data::migrate::migrate;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::manager::CORE_MANAGER;

pub mod db;

pub fn boot(path: &Option<PathBuf>) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let db = db::connect(path.clone().unwrap_or(CORE_MANAGER.data_dir().join("data.db")))?;

    migrate(&mut db.get()?)?;
    register_defaults();

    Ok(db)
}
