use anyhow::Result;
use data::{
    diesel::r2d2::{ConnectionManager, Pool},
    Conn,
};

use crate::cli::Cli;

#[derive(Clone)]
pub struct AppState {
    pub options: Cli,
    pub pool: Pool<ConnectionManager<Conn>>,
}

impl AppState {
    pub async fn new(options: Cli, pool: Pool<ConnectionManager<Conn>>) -> Result<Self> {
        Ok(Self { options, pool })
    }
}
