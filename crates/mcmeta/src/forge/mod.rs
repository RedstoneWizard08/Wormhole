//! The module for Forge (LexForge)
//!
//! LexForge version format:
//! - [mc_version]-[forge_version]
//!
//! Versions must be queried from Forge's maven, with the artifact `net.minecraftforge:forge`

use std::cell::LazyCell;

use anyhow::Result;
use whcore::async_trait::async_trait;

use crate::{
    installer::manifest::InstallerManifest,
    loader::LoaderData,
    maven::{artifact::Artifact, MavenRepo},
    util::get_file_from_zip,
    vanilla::manifest::VersionManifest,
};

pub const MAVEN_REPO: LazyCell<MavenRepo> =
    LazyCell::new(|| MavenRepo::new("https://maven.minecraftforge.net".into()));

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

    async fn get_version_manifest(
        &self,
        version: impl Into<Artifact> + Send,
        game_version: impl AsRef<str> + Send,
    ) -> Result<VersionManifest> {
        let artifact = version.into().set_classifier("installer");
        let url = self.version_jar_url(artifact).await?;
        let data = reqwest::get(url).await?.bytes().await?;
        let data = get_file_from_zip(data, "version.json")?;
        let manifest = serde_json::from_str::<InstallerManifest>(&data)?;

        Ok(manifest
            .resolve(game_version.as_ref().to_string(), MAVEN_REPO.clone())
            .await?)
    }
}
