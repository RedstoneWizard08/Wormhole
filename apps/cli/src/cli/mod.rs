pub mod pack;

use std::env;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use pack::PackCommands;
use whcore::{async_trait::async_trait, traits::Runnable};

/// Wormhole's CLI
#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "\x1b[1;36mWormhole's CLI\x1b[0m", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Commands for modpack management
    Pack {
        #[command(subcommand)]
        command: PackCommands,
    },
}

#[async_trait]
impl Runnable for Commands {
    async fn run(&self) -> Result<()> {
        match self {
            Commands::Pack { command } => command.run().await,
        }
    }
}

#[async_trait]
impl Runnable for Cli {
    async fn run(&self) -> Result<()> {
        if env::var("GITHUB_TOKEN").is_err() {
            env::set_var("GITHUB_TOKEN", "");
        }

        let cli = Cli::parse();
        let verbose = cli.verbose;

        if verbose {
            println!("Command: {:?}", cli.command);
        }

        if let Some(command) = cli.command {
            let res = command.run().await;

            if let Err(err) = res {
                let mut cmd = Cli::command();

                cmd.print_help()?;

                println!("\n\x1b[1;31mError:\x1b[0m {}", err);
            }
        } else {
            let mut cmd = Cli::command();

            cmd.print_help()?;
        }

        Ok(())
    }
}
