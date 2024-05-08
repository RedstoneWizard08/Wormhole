use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;

use crate::{
    download::DownloadCallbackFn,
    forge::processor::{build_processor_command, should_run_processor},
    piston::download::{download_lib_refs, download_libs},
};

use super::{
    extract::extract_neoforge_installer, get_neoforge_manifest,
    profile::get_neoforge_install_profile, NEOFORGE_MAVEN,
};

pub async fn install_neoforge(
    java: PathBuf,
    lib_dir: PathBuf,
    tmp_dir: PathBuf,
    version: impl AsRef<str>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let version = version.as_ref();
    let manifest = get_neoforge_manifest(&version).await?;
    let profile = get_neoforge_install_profile(&version).await?;

    extract_neoforge_installer(&tmp_dir, &version).await?;
    download_libs(&lib_dir, manifest.resolve().await?, &callback).await?;
    download_lib_refs(&lib_dir, profile.libraries.clone(), &callback).await?;

    for proc in profile.processors.clone() {
        if !should_run_processor(proc.sides.clone()) {
            continue;
        }

        let mut cmd =
            build_processor_command(&profile, &java, &lib_dir, &tmp_dir, proc, NEOFORGE_MAVEN)
                .await?;

        Command::new(cmd.remove(0))
            .args(cmd)
            .envs(&env::vars().collect::<HashMap<_, _>>())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
    }

    Ok(())
}
