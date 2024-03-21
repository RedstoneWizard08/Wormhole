use anyhow::Result;
use whcore::get_data_dir;

pub mod db;

pub fn boot() -> Result<()> {
    db::init_database(get_data_dir().join("data.db"))?;

    Ok(())
}
