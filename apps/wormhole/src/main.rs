use std::env;

use anyhow::Result;
use clap::Parser;
use whcore::traits::Runnable;
use wormhole::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    if env::var("GITHUB_TOKEN").is_err() {
        env::set_var("GITHUB_TOKEN", "");
    }

    Cli::parse().run().await
}
