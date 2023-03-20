#![allow(clippy::needless_return)]

use clap::Parser;
use tokio::main;

use cli::{Cli, Commands, ModCommands};
use commands::mods::install::install_mod;

pub mod cli;
pub mod commands;

#[main]
pub async fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    if verbose {
        println!("Command: {:?}", cli.command);
    }

    if let Some(Commands::Mod {
        command: Some(ModCommands::Install { id, instance_id }),
    }) = cli.command
    {
        install_mod(id, instance_id, verbose).await;
    }
}
