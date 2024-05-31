//! The CLI for the Web UI.

use crate::log::{from_log_level, init_file_logger};
use crate::server::run_server;
use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use init::boot;
use lazy_static::lazy_static;
use midlog::add_route_filter;
use std::path::PathBuf;
use std::sync::RwLock;
use whcore::async_trait::async_trait;
use whcore::traits::Runnable;

lazy_static! {
    static ref CONFIG: RwLock<Option<Cli>> = RwLock::new(None);
}

/// The CLI for the Web UI.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The host to listen on.
    #[arg(short = 'H', long = "host", default_value = "0.0.0.0", env = "HOST")]
    pub host: String,

    /// The port to listen on.
    #[arg(short = 'p', long = "post", default_value_t = 4000, env = "PORT")]
    pub port: u16,

    /// The username for the database.
    #[arg(short = 'u', long = "db-path", env = "DB_PATH")]
    pub db_path: Option<PathBuf>,

    /// Enables verbose mode.
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// Enable safe-IP mode.
    /// Will hide all client IPs from logs.
    #[arg(short = 's', long = "safe", default_value_t = false, env = "SAFE_MODE")]
    pub safe: bool,
}

#[async_trait]
impl Runnable for Cli {
    async fn run(&self) -> Result<()> {
        init_file_logger(
            "./logs/server.log",
            from_log_level(self.verbose.log_level_filter()),
        )?;

        *CONFIG.write().unwrap() = Some(self.clone());

        info!("Setting up the database...");

        let pool = boot(&self.db_path).await?;

        info!("Creating route filters...");

        add_route_filter("/node_modules/");
        add_route_filter("/.svelte-kit/");
        add_route_filter("/@vite/client");
        add_route_filter("/@id/");

        run_server(pool, self.clone()).await
    }
}
