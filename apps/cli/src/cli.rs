use std::env;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use whcore::{async_trait::async_trait, traits::Runnable};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Instance {
        #[command(subcommand)]
        command: Option<InstanceCommands>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ModCommands {
    Info {
        #[arg(value_enum)]
        id: String,
    },

    Install {
        #[arg(value_enum)]
        id: String,
        instance_id: i32,
    },

    Remove {
        #[arg(value_enum)]
        id: String,
        instance_id: i32,
    },

    Browse {
        #[arg(value_enum)]
        query: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceCommands {
    List {},
    Create {},
    Delete {},
    Info {},
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

        let /* mut */ ok = false;

        if let Some(_command) = cli.command {
            todo!()
        }

        if !ok {
            let mut cmd = Cli::command();

            cmd.print_help().unwrap();
        }

        Ok(())
    }
}
