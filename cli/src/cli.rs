use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone, Copy)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Commands {
    Mod {
        #[command(subcommand)]
        command: Option<ModCommands>,
    },

    Instance {
        #[command(subcommand)]
        command: Option<InstanceCommands>,
    },
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum ModCommands {
    Install { id: i32, instance_id: Option<i32> },

    Info { id: i32, instance_id: Option<i32> },

    Remove { id: i32, instance_id: Option<i32> },
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum InstanceCommands {
    List {},
    Create {},
    Delete {},
    Info {},
}
