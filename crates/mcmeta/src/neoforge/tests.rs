use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;
use which::which;

use crate::{
    forge::processor::{build_processor_command, should_run_processor},
    neoforge::{
        extract::extract_neoforge_installer, profile::get_neoforge_install_profile, NEOFORGE_MAVEN,
    },
    piston::download::{download_lib_refs, download_libs},
    test_util::library_download_callback,
};

use super::get_neoforge_manifest;

#[tokio::test]
pub async fn test_neoforge_manifest_works() -> Result<()> {
    let it = get_neoforge_manifest("20.4.232").await?;

    assert_eq!(it.id, "neoforge-20.4.232");

    Ok(())
}

#[tokio::test]
pub async fn test_neoforge_processor_cmd_works() -> Result<()> {
    let version = "20.4.232";
    let lib_dir = PathBuf::from("./test/libraries");
    let tmp_dir = PathBuf::from("./test/tmp");

    extract_neoforge_installer(&tmp_dir, &version).await?;

    let manifest = get_neoforge_manifest(&version).await?;

    download_libs(
        &lib_dir,
        manifest.resolve().await?,
        Some(Box::new(library_download_callback)),
    )
    .await?;

    let profile = get_neoforge_install_profile(&version).await?;
    let java = which("java")?;

    download_lib_refs(
        &lib_dir,
        profile.libraries.clone(),
        Some(Box::new(library_download_callback)),
    )
    .await?;

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
