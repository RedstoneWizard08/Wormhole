use anyhow::Result;
use clap::Parser;
use whcore::traits::Runnable;
use wormhole_cli::cli::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}
