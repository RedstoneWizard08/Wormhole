#![allow(unused)]

use clap::{CommandFactory, Parser};
use std::env;
use tokio::main;
use wormhole_cli::cli::{Cli, Commands};

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
        todo!()
    }

    if !ok {
        let mut cmd = Cli::command();

        cmd.print_help().unwrap();
    }
}
