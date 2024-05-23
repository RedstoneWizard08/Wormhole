use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use specta::ts::{formatter::prettier, BigIntExportBehavior, ExportConfig};
use tauri::Wry;
use tauri_specta::ts;
use whcore::{async_trait::async_trait, traits::Runnable};

pub const BIG_INT: BigIntExportBehavior = BigIntExportBehavior::Number;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "src/api/bindings/app.ts")]
    pub path: PathBuf,
}

#[async_trait]
impl Runnable for Cli {
    async fn run(&self) -> Result<()> {
        println!("Exporting app bindings...");

        ts::builder()
            .commands(wormhole_gui::cmds())
            .events(wormhole_gui::events::<Wry>())
            .path(&self.path)
            .config(ExportConfig::default().formatter(prettier).bigint(BIG_INT))
            .export()?;

        Ok(())
    }
}
