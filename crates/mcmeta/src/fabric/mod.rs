use anyhow::Result;

use crate::{
    launchwrapper::config::LaunchWrapperConfig,
    maven::{
        artifact::MavenArtifact, coord::MavenCoordinate, get_metadata, metadata::MavenMetadata,
    },
    piston::game::inherit::InheritedGameManifest,
};

pub const FABRIC_MAVEN: &str = "https://maven.fabricmc.net";

pub async fn get_fabric_versions() -> Result<MavenMetadata> {
    Ok(get_metadata(FABRIC_MAVEN, "net.fabricmc:fabric-loader").await?)
}

pub fn get_fabric_loader(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("net.fabricmc:fabric-loader:{}", ver),
        repo: FABRIC_MAVEN.into(),
    }
}

pub fn get_fabric_launchwrapper_artifact(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("net.fabricmc:fabric-loader:{}:launchwrapper@json", ver),
        repo: FABRIC_MAVEN.into(),
    }
}

pub async fn get_fabric_launchwrapper(version: impl AsRef<str>) -> Result<LaunchWrapperConfig> {
    let ver = version.as_ref();
    let coord = MavenCoordinate::from(format!(
        "net.fabricmc:fabric-loader:{}:launchwrapper@json",
        ver
    ));
    let url = coord.url(FABRIC_MAVEN);

    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_fabric_profile(
    mc_version: impl AsRef<str>,
    fabric_version: impl AsRef<str>,
) -> Result<InheritedGameManifest> {
    Ok(reqwest::get(format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
        mc_version.as_ref(),
        fabric_version.as_ref()
    ))
    .await?
    .json()
    .await?)
}
