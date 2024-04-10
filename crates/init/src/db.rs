use anyhow::{anyhow, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use std::path::PathBuf;

pub fn connect(path: PathBuf) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let path = path.to_str().unwrap();
    let url = format!("sqlite://{}", path);
    let mgr = ConnectionManager::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(mgr)
        .map_err(|v| anyhow!(v))
}
