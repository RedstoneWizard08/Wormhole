use std::path::PathBuf;

use anyhow::Result;
use data::migrate::migrate;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use msa::state::MsaState;
use whcore::manager::CoreManager;
use plugins::register_defaults;

pub mod db;

pub static mut INIT: bool = false;

pub async fn boot(path: &Option<PathBuf>) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
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
    register_defaults().await;

    tokio::spawn(async move {
        MsaState::init().await.unwrap();
    })
    .await?;

    Ok(db)
}
