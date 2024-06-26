use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;

use crate::{
    download::DownloadCallbackFn,
    piston::{
        download::{download_lib_refs, download_libs},
        game::library::LibraryRef,
    },
};

use super::{
    extract::extract_forge_installer,
    get_forge_manifest,
    mappings::get_forge_install_profile,
    processor::{build_processor_command, should_run_processor},
    FORGE_MAVEN,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallProfile {
    pub spec: u8,
    pub profile: String,
    pub version: String,
    pub path: Option<String>,
    pub minecraft: String,
    pub server_jar_path: String,
    pub data: HashMap<String, Sided<String>>,
    pub processors: Vec<Processor>,
    pub libraries: Vec<LibraryRef>,
    pub icon: String,
    pub json: String,
    pub logo: String,
    pub mirror_list: String,
    pub welcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Processor {
    pub sides: Option<Vec<String>>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sided<T> {
    pub client: T,
    pub server: T,
}

pub async fn install_forge(
    java: &PathBuf,
    lib_dir: &PathBuf,
    tmp_dir: &PathBuf,
    version: impl AsRef<str>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let version = version.as_ref();
    let manifest = get_forge_manifest(&version).await?;
    let profile = get_forge_install_profile(&version).await?;

    extract_forge_installer(&tmp_dir, &version).await?;
    download_libs(&lib_dir, &manifest.resolve().await?, &callback).await?;
    download_lib_refs(&lib_dir, profile.libraries.clone(), &callback).await?;

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
