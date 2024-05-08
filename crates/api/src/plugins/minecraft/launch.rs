use std::path::PathBuf;

use anyhow::Result;
use mcmeta::cmd::{cmd::build_launch_command, modded::ModLoader, options::LaunchOptions};
use msa::state::MsaState;

pub async fn launch_minecraft(
    java: PathBuf,
    mem: u32,
    root: PathBuf,
    loader: &ModLoader,
    auth: &MsaState,
) -> Result<()> {
    let cmd = build_launch_command(java, root, loader, LaunchOptions::new(auth, mem)).await?;

    Ok(())
}
