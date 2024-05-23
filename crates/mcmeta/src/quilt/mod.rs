pub mod intermediary;

use anyhow::Result;

use crate::{
    launchwrapper::config::LaunchWrapperConfig,
    maven::{
        artifact::MavenArtifact, coord::MavenCoordinate, get_metadata, metadata::MavenMetadata,
    },
    piston::game::inherit::InheritedGameManifest,
};

pub const QUILT_MAVEN: &str = "https://maven.quiltmc.org/repository/release";

pub async fn get_quilt_versions() -> Result<MavenMetadata> {
    Ok(get_metadata(QUILT_MAVEN, "org.quiltmc:quilt-loader").await?)
}

pub fn get_quilt_loader(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("org.quiltmc:quilt-loader:{}", ver),
        repo: QUILT_MAVEN.into(),
    }
}

pub fn get_quilt_launchwrapper_artifact(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("org.quiltmc:quilt-loader:{}@json", ver),
        repo: QUILT_MAVEN.into(),
    }
}

pub async fn get_quilt_launchwrapper(version: impl AsRef<str>) -> Result<LaunchWrapperConfig> {
    let ver = version.as_ref();
    let coord = MavenCoordinate::from(format!("org.quiltmc:quilt-loader:{}@json", ver));
    let url = coord.url(QUILT_MAVEN);

    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_quilt_profile(
    mc_version: impl AsRef<str>,
    quilt_version: impl AsRef<str>,
) -> Result<InheritedGameManifest> {
    Ok(reqwest::get(format!(
        "https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json",
        mc_version.as_ref(),
        quilt_version.as_ref()
    ))
    .await?
    .json()
    .await?)
}
