use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::get_data_dir;

pub mod db;

pub fn boot() -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    db::connect(get_data_dir().join("data.db"))
}
