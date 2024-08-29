//! The module for NeoForge
//!
//! NeoForge version format:
//! - [mc_minor].[mc_patch].[neo_build]

pub mod meta;
pub mod version;

use std::cell::LazyCell;

use anyhow::Result;
use meta::ReposiliteVersions;
use version::NeoForgeVersion;
use whcore::async_trait::async_trait;

use crate::{
    installer::manifest::InstallerManifest,
    loader::LoaderData,
    maven::{artifact::Artifact, MavenRepo},
    util::get_file_from_zip,
    vanilla::manifest::VersionManifest,
};

pub const MAVEN_REPO: LazyCell<MavenRepo> =
    LazyCell::new(|| MavenRepo::new("https://maven.neoforged.net/releases".into()));

pub const VERSIONS_API: &str =
    "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
pub struct NeoForge;

#[async_trait]
impl LoaderData for NeoForge {
    async fn all_versions(&self) -> Result<Vec<Artifact>> {
        Ok(reqwest::get(VERSIONS_API)
            .await?
            .json::<ReposiliteVersions>()
            .await?
            .versions
            .iter()
            .map(|v| Artifact::new("net.neoforged:neoforge").set_version(v))
            .collect())
    }

    async fn versions_for(&self, game_version: impl AsRef<str> + Send) -> Result<Vec<Artifact>> {
        let game_version = game_version.as_ref();

        Ok(self
            .all_versions()
            .await?
            .iter()
            .filter(|v| NeoForgeVersion::new(v.version.clone().unwrap()).minecraft == game_version)
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
