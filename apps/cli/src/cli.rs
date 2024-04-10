use clap::{Parser, Subcommand};

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
