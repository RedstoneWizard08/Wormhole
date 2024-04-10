#![allow(unused)]

use std::env;

use clap::{CommandFactory, Parser};
use tokio::main;

use cli::{Cli, Commands};

pub mod auth;
pub mod cli;

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
