use anyhow::Result;
use clap::Parser;
use whcore::traits::Runnable;
use wormhole::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    Cli::parse().run().await
}
