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

    if let Some(scmd) = cli.command {
        match scmd {
            Commands::Mod { command } => {
                if let Some(scmd) = command {
                    match scmd {
                        ModCommands::Install { id, instance_id } => {
                            install_mod(id, verbose).await;
                        },

                        _ => (),
                    };
                }
            },

            _ => (),
        };
    }
}
