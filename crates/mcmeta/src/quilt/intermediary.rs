use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::{
    download::{download_file, DownloadCallbackFn},
    fabric::FABRIC_MAVEN,
    maven::artifact::MavenArtifact,
};

use super::QUILT_MAVEN;

pub const INTERMEDIARY_ENDPOINT: &str = "https://meta.quiltmc.org/v3/versions/intermediary";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediaryRef {
    pub maven: String,
    pub version: String,
}

impl IntermediaryRef {
    pub fn is_for_version(&self, ver: impl AsRef<str>) -> bool {
        self.version == ver.as_ref()
    }
}

pub async fn get_intermediaries() -> Result<Vec<IntermediaryRef>> {
    Ok(reqwest::get(INTERMEDIARY_ENDPOINT).await?.json().await?)
}

pub async fn get_intermediary(ver: impl AsRef<str>) -> Result<MavenArtifact> {
    let items = get_intermediaries().await?;

    let item = items
        .iter()
        .find(|v| v.is_for_version(ver.as_ref()))
        .ok_or(anyhow!(
            "Cannot find an intermediary for the version: {}",
            ver.as_ref()
        ))?;

    Ok(MavenArtifact {
        repo: if item.maven.starts_with("net.fabricmc") {
            FABRIC_MAVEN.into()
        } else {
            QUILT_MAVEN.into()
        },
        name: item.maven.clone(),
    })
}

pub async fn download_intermediary(
    lib_dir: &PathBuf,
    version: impl AsRef<str>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let item = get_intermediary(version).await?;

    download_file(
        &lib_dir,
        &item.coordinate().url(&item.repo),
        item.coordinate().path(),
        Option::<String>::None,
        callback,
    )
    .await?;

    Ok(())
}
