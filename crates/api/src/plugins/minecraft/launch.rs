use std::path::PathBuf;

use anyhow::Result;
use mcmeta::cmd::{cmd::build_launch_command, modded::ModLoader, options::LaunchOptions};
use msa::state::MsaState;
use tokio::process::{Child, Command};

pub async fn launch_minecraft(
    java: &PathBuf,
    mem: u64,
    root: &PathBuf,
    loader: &ModLoader,
    auth: &MsaState,
) -> Result<Child> {
    let cmd = build_launch_command(java, root, loader, LaunchOptions::new(auth, mem)).await?;

    Ok(Command::new(cmd.first().unwrap()).args(&cmd[1..]).spawn()?)
}
