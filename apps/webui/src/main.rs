use anyhow::Result;
use clap::Parser;
use webui::cli::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}
