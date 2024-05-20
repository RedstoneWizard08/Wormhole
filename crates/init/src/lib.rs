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
