#![allow(clippy::needless_return)]

pub mod cli;
pub mod commands;

use clap::Parser;
use cli::{Cli, Commands, ModCommands};
use commands::mods::install::install_mod;
use tokio::main;

#[main]
pub async fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    if verbose {
        println!("Command: {:?}", cli.command);
    }

    if let Some(Commands::Mod {
        command:
            Some(ModCommands::Install {
                id,
                instance_id: _iid,
            }),
    }) = cli.command
    {
        install_mod(id, verbose).await;
    }
}
