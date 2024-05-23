use anyhow::Result;
use clap::{Parser, Subcommand};
use whcore::{async_trait::async_trait, traits::Runnable};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[async_trait]
impl Runnable for Cli {
    async fn run(&self) -> Result<()> {
        self.command.run().await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Bindgen(bindings::Cli),
    Server(webui::cli::Cli),
    Cli(wormhole_cli::cli::Cli),
}

#[async_trait]
impl Runnable for Commands {
    async fn run(&self) -> Result<()> {
        match self {
            Self::Bindgen(b) => b.run().await?,
            Self::Server(s) => s.run().await?,
            Self::Cli(c) => c.run().await?,
        };

        Ok(())
    }
}
