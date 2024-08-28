//! The module for Fabric

use anyhow::Result;
use meta::FabricMetaVersion;
use whcore::async_trait::async_trait;

use crate::{
    loader::LoaderData,
    maven::{artifact::Artifact, MavenRepo},
};

pub mod meta;

pub const MAVEN_REPO: MavenRepo = MavenRepo::new("https://maven.fabricmc.net");
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
            .collect())
    }

    async fn versions_for(&self, _game_version: impl AsRef<str> + Send) -> Result<Vec<Artifact>> {
        self.all_versions().await
    }

    async fn version_jar_url(&self, artifact: impl Into<Artifact> + Send) -> Result<String> {
        Ok(MAVEN_REPO.get_artifact_url(artifact))
    }
}
