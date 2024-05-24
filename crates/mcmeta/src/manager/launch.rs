use std::path::PathBuf;

use crate::cmd::{cmd::build_launch_command, modded::ModLoader, options::LaunchOptions};
use anyhow::Result;
use msa::state::MsaState;
use tokio::process::{Child, Command};

pub async fn launch_minecraft(
    java: &PathBuf,
    mem: u64,
    root: &PathBuf,
    loader: &ModLoader,
    auth: &MsaState,
) -> Result<Child> {
    let f_auth = MsaState::fake(auth);
    let cmd = build_launch_command(java, root, loader, LaunchOptions::new(auth, mem)).await?;
    let cmd_l = build_launch_command(java, root, loader, LaunchOptions::new(&f_auth, mem)).await?;

    info!("Launching process: {}", cmd_l.join(" "));

    Ok(Command::new(cmd.first().unwrap())
        .current_dir(root)
        .args(&cmd[1..])
        .spawn()?)
}
