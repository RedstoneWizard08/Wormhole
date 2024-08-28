//! The module for Forge (LexForge)
//!
//! LexForge version format:
//! - [mc_version]-[forge_version]
//!
//! Versions must be queried from Forge's maven, with the artifact `net.minecraftforge:forge`

use anyhow::Result;
use whcore::async_trait::async_trait;

use crate::{
    loader::LoaderData,
    maven::{artifact::Artifact, MavenRepo},
};

pub const MAVEN_REPO: MavenRepo = MavenRepo::new("https://maven.minecraftforge.net");

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
pub struct Forge;

#[async_trait]
impl LoaderData for Forge {
    async fn all_versions(&self) -> Result<Vec<Artifact>> {
        MAVEN_REPO.get_versions("net.minecraftforge:forge").await
    }

    async fn versions_for(&self, game_version: impl AsRef<str> + Send) -> Result<Vec<Artifact>> {
        let game_version = game_version.as_ref();

        Ok(MAVEN_REPO
            .get_versions("net.minecraftforge:forge")
            .await?
            .iter()
            .filter(|v| v.version.clone().unwrap().split("-").next().unwrap() == game_version)
            .cloned()
            .collect())
    }

    async fn version_jar_url(&self, artifact: impl Into<Artifact> + Send) -> Result<String> {
        Ok(MAVEN_REPO.get_artifact_url(artifact))
    }
}
