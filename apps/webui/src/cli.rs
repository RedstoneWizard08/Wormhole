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

lazy_static! {
    pub static ref CONFIG: RwLock<Option<Cli>> = RwLock::new(None);
}

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

impl Cli {
    pub async fn run(&mut self) -> Result<()> {
        init_file_logger(
            "./logs/server.log",
            from_log_level(self.verbose.log_level_filter()),
        )?;

        *CONFIG.write().unwrap() = Some(self.clone());

        info!("Setting up the database...");

        let pool = boot(&self.db_path)?;

        info!("Creating route filters...");

        add_route_filter("/node_modules/");
        add_route_filter("/.svelte-kit/");
        add_route_filter("/@vite/client");
        add_route_filter("/@id/");

        run_server(pool, self.clone()).await
    }

    pub const fn init_script(&self) -> &str {
        "
            Object.defineProperty(window, '__TAURI_POST_MESSAGE__', {{
                value: (message) => {{
                    const request = new XMLHttpRequest();
                    
                    request.addEventListener('load', function () {{
                        let arg;
                        let success = this.status === 200;

                        try {{
                          arg = JSON.parse(this.response);
                        }} catch (e) {{
                          arg = e;
                          success = false;
                        }}

                        window[`_${{success ? message.callback : message.error}}`](arg);
                    }});

                    request.open('POST', '/_tauri/invoke/?window=' + window.__TAURI_METADATA__.__currentWindow.label, true);
                    request.setRequestHeader('Content-Type', 'application/json');
                    request.send(JSON.stringify(message));
                }},
            }});
        "
    }
}
