use anyhow::Result;
use clap::{Parser, Subcommand};
use whcore::{async_trait::async_trait, traits::Runnable};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[async_trait]
impl Runnable for Cli {
    async fn run(&self) -> Result<()> {
        self.command.clone().unwrap_or_default().run().await
    }
}

#[derive(Debug, Clone, Subcommand, Default)]
pub enum Commands {
    /// TypeScript bindgen CLI tool.
    Bindgen(bindings::Cli),

    /// Web UI management CLI.
    Server(webui::cli::Cli),

    /// Wormhole CLI.
    Cli(wormhole_cli::cli::Cli),

    /// Wormhole GUI app.
    #[default]
    Gui,
}

#[async_trait]
impl Runnable for Commands {
    async fn run(&self) -> Result<()> {
        match self {
            Self::Bindgen(b) => b.run().await?,
            Self::Server(s) => s.run().await?,
            Self::Cli(c) => c.run().await?,
            Self::Gui => wormhole_gui::run().await?,
        };

        Ok(())
    }
}
