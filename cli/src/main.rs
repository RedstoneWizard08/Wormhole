#![allow(clippy::needless_return)]

use std::env;

use clap::Parser;
use tokio::main;

use cli::{Cli, Commands, ModCommands};
use commands::mods::install::install_mod;
use wormhole_common::github::{DeviceFlow, WORMHOLE_CLIENT_ID};

pub mod cli;
pub mod commands;

#[main]
pub async fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    if verbose {
        println!("Command: {:?}", cli.command);
    }

    let mut flow = DeviceFlow::start(WORMHOLE_CLIENT_ID.to_string()).await.unwrap();

    println!("Please go to: {} and enter code: {}", flow.clone().verification_uri.unwrap(), flow.clone().user_code.unwrap());

    let polled = flow.poll().await.unwrap();
    let token = polled.token;

    env::set_var("GITHUB_TOKEN", token);

    if let Some(Commands::Mod {
        command: Some(ModCommands::Install { id, instance_id }),
    }) = cli.command
    {
        install_mod(id, instance_id, verbose).await;
    }
}
