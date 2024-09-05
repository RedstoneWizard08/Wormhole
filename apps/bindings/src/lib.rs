use anyhow::Result;
use clap::Parser;
use commands::router::build_router;
use std::path::PathBuf;
use whcore::{async_trait::async_trait, traits::Runnable};

/// TypeScript bindgen CLI tool.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "packages/api/src/bindings.ts")]
    pub path: PathBuf,
}

#[async_trait]
impl Runnable for Cli {
    async fn run(&self) -> Result<()> {
        println!("Exporting app bindings...");

        build_router().export("/rpc", self.path.clone())?;

        Ok(())
    }
}
