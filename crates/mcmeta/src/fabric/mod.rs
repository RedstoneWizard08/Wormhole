//! The module for Fabric

use std::cell::LazyCell;

use anyhow::Result;
use meta::FabricMetaVersion;
use whcore::async_trait::async_trait;

use crate::{
    installer::manifest::InstallerManifest,
    loader::LoaderData,
    maven::{artifact::Artifact, MavenRepo},
    vanilla::manifest::VersionManifest,
};

pub mod meta;

pub const MAVEN_REPO: LazyCell<MavenRepo> =
    LazyCell::new(|| MavenRepo::new("https://maven.fabricmc.net".into()));
pub const META_API: &str = "https://meta.fabricmc.net/v2/versions/loader";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
pub struct Fabric;

#[async_trait]
impl LoaderData for Fabric {
    async fn all_versions(&self) -> Result<Vec<Artifact>> {
        Ok(reqwest::get(META_API)
            .await?
            .json::<Vec<FabricMetaVersion>>()
            .await?
            .iter()
            .map(|v| v.maven.clone().into())
            .collect::<Vec<_>>())
    }

    async fn versions_for(&self, _game_version: impl AsRef<str> + Send) -> Result<Vec<Artifact>> {
        self.all_versions().await
    }

    async fn version_jar_url(&self, artifact: impl Into<Artifact> + Send) -> Result<String> {
        Ok(MAVEN_REPO.get_artifact_url(artifact))
    }

    async fn get_version_manifest(
        &self,
        version: impl Into<Artifact> + Send,
        game_version: impl AsRef<str> + Send,
    ) -> Result<VersionManifest> {
        let url = format!(
            "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
            game_version.as_ref(),
            version.into().version.unwrap()
        );

        let data = reqwest::get(url).await?.text().await?;
        let data = serde_json::from_str::<InstallerManifest>(&data)?;

        Ok(data
            .resolve(game_version.as_ref().to_string(), MAVEN_REPO.clone())
            .await?)
    }
}
