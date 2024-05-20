// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
        &manifest.resolve().await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    let profile = get_neoforge_install_profile(&version).await?;
    let java = which("java")?;

    download_lib_refs(
        &lib_dir,
        profile.libraries.clone(),
        &Some(Box::new(library_download_callback)),
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
