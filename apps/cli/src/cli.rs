use clap::{Parser, Subcommand};
use wormhole_common::instances::KSPGame;

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
    #[command(name = "spacedock")]
    SpaceDock {
        #[command(subcommand)]
        command: Option<ModCommands>,
    },

    CKAN {
        #[command(subcommand)]
        command: Option<ModCommands>,
    },

    #[command(name = "curseforge")]
    CurseForge {
        #[command(subcommand)]
        command: Option<ModCommands>,
    },

    Instance {
        #[command(subcommand)]
        command: Option<InstanceCommands>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ModCommands {
    Info {
        #[arg(value_enum)]
        game_id: KSPGame,
        id: String,
    },

    Install {
        #[arg(value_enum)]
        game_id: KSPGame,
        id: String,
        instance_id: i32,
    },

    Remove {
        #[arg(value_enum)]
        game_id: KSPGame,
        id: String,
        instance_id: i32,
    },

    Browse {
        #[arg(value_enum)]
        game_id: KSPGame,

        name_filter: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceCommands {
    List {},
    Create {},
    Delete {},
    Info {},
}
