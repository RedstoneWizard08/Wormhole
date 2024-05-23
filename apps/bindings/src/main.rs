use anyhow::Result;
use bindings::Cli;
use clap::Parser;
use whcore::traits::Runnable;

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}
