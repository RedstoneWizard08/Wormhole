//! The module for NeoForge
//!
//! NeoForge version format:
//! - [mc_minor].[mc_patch].[neo_build]

pub mod meta;
pub mod version;

use anyhow::Result;
use meta::ReposiliteVersions;
use version::NeoForgeVersion;
use whcore::async_trait::async_trait;

use crate::{loader::LoaderData, maven::{artifact::Artifact, MavenRepo}};

pub const MAVEN_REPO: MavenRepo = MavenRepo::new("https://maven.neoforged.net/releases");

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
}
