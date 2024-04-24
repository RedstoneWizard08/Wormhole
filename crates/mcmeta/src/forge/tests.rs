use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;
use which::which;

use crate::{
    forge::{
        extract::extract_forge_installer,
        mappings::get_forge_install_profile,
        processor::{build_processor_command, should_run_processor},
        FORGE_MAVEN,
    },
    piston::download::{download_lib_refs, download_libs},
    test_util::library_download_callback,
};

use super::get_forge_manifest;

#[tokio::test]
pub async fn test_forge_manifest_works() -> Result<()> {
    let it = get_forge_manifest("1.20.2-48.1.0").await?;

    assert_eq!(it.id, "1.20.2-forge-48.1.0");

    Ok(())
}

#[tokio::test]
pub async fn test_forge_processor_cmd_works() -> Result<()> {
    let version = "1.20.2-48.1.0";
    let lib_dir = PathBuf::from("./test/libraries");
    let tmp_dir = PathBuf::from("./test/tmp");

    extract_forge_installer(&tmp_dir, &version).await?;

    let manifest = get_forge_manifest(&version).await?;

    download_libs(
        &lib_dir,
        manifest.resolve().await?,
        Some(Box::new(library_download_callback)),
    )
    .await?;

    let profile = get_forge_install_profile(&version).await?;
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
            build_processor_command(&profile, &java, &lib_dir, &tmp_dir, proc, FORGE_MAVEN).await?;

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
