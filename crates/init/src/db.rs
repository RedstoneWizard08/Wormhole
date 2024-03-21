use anyhow::{anyhow, Result};
use diesel::{Connection, SqliteConnection};
use std::path::PathBuf;

pub fn init_database(path: PathBuf) -> Result<SqliteConnection> {
    let path = path.to_str().unwrap();
    let url = format!("sqlite://{}", path);

    SqliteConnection::establish(&url).map_err(|v| anyhow!(v))
}
