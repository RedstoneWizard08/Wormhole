#![allow(clippy::needless_return)]

use std::env;

use clap::{Parser, CommandFactory};
use commands::{ckan, curseforge, instance, spacedock};
use tokio::main;

use cli::{Cli, Commands};

pub mod auth;
pub mod cli;
pub mod commands;

#[main]
pub async fn main() {
    if env::var("GITHUB_TOKEN").is_err() {
        env::set_var("GITHUB_TOKEN", "");
    }

    let cli = Cli::parse();
    let verbose = cli.verbose;

    if verbose {
        println!("Command: {:?}", cli.command);
    }

    let mut ok: bool = false;

    if let Some(command) = cli.command {
        match command {
            Commands::SpaceDock { command } => {
                if let Some(command) = command {
                    ok = spacedock::match_command(command).await;
                }
            }

            Commands::CKAN { command } => {
                if let Some(command) = command {
                    ok = ckan::match_command(command).await;
                }
            }

            Commands::CurseForge { command } => {
                if let Some(command) = command {
                    ok = curseforge::match_command(command).await;
                }
            }

            Commands::Instance { command } => {
                if let Some(command) = command {
                    ok = instance::match_command(command).await;
                }
            }
        };
    }

    if !ok {
        let mut cmd = Cli::command();

        cmd.print_help().unwrap();
    }
}
