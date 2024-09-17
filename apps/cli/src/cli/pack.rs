use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use inquire::Text;
use pack::models::{
    index::PackIndex,
    pack::{IndexInfo, ListFormat, Pack, PackMeta, Side},
};
use whcore::{async_trait::async_trait, Runnable};

#[derive(Subcommand, Debug, Clone)]
pub enum PackCommands {
    /// Initialize a new modpack project.
    Init,

    /// Refresh the index lock.
    Refresh,

    /// Add a mod to the pack.
    Add {
        /// The mod source.
        #[arg(short, long)]
        source: Option<String>,

        /// The side this mod is on.
        #[arg(short, long, value_enum)]
        side: Option<Side>,

        /// The project. Can be `[id|slug]=[version]`
        project: String,
    },

    /// Export the pack.
    Export {
        /// The format to export.
        #[arg(short, long)]
        format: Option<String>,

        /// The side to export.
        #[arg(short, long, value_enum)]
        side: Option<Side>,
    },

    /// List mods in the pack.
    List {
        /// The file to save to.
        #[arg(short, long)]
        save: Option<PathBuf>,

        /// The format to save as.
        #[arg(short, long)]
        format: Option<ListFormat>,
    },

    /// Update mods in the pack.
    Update {
        /// Update all mods.
        #[arg(short, long)]
        all: Option<bool>,

        /// The mod to update.
        project: Option<String>,
    },
}

#[async_trait]
impl Runnable for PackCommands {
    async fn run(&self) -> Result<()> {
        match self {
            Self::Init => {
                let mut data = Pack {
                    meta: PackMeta {
                        name: Text::new("What should your pack be named?").prompt()?,
                        version: Text::new("What version should your pack be?")
                            .with_default("0.1.0")
                            .prompt()?,
                        authors: vec![Text::new("Who is the author?")
                            .with_default("Me!")
                            .prompt()?],
                    },
                    index: IndexInfo::default(),
                    export: Vec::new(),
                    loaders: Vec::new(),
                    versions: HashMap::new(),
                };

                fs::write("index.lock", PackIndex::default().encode()?)?;

                data.refresh(None)?;

                fs::write("pack.toml", toml::to_string_pretty(&data)?)?;
            }

            Self::Refresh => {
                Pack::load(None)?.refresh(None)?;
            }

            _ => todo!(),
        }

        Ok(())
    }
}
