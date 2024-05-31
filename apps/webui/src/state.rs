//! The state module.

use anyhow::Result;
use data::{
    diesel::r2d2::{ConnectionManager, Pool},
    Conn,
};

use crate::cli::Cli;

/// The state for the Web UI.
#[derive(Clone)]
pub struct AppState {
    /// The CLI options.
    pub options: Cli,

    /// The database pool.
    pub pool: Pool<ConnectionManager<Conn>>,
}

impl AppState {
    /// Create a new state object.
    pub async fn new(options: Cli, pool: Pool<ConnectionManager<Conn>>) -> Result<Self> {
        Ok(Self { options, pool })
    }
}
