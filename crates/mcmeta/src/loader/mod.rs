use anyhow::Result;
use whcore::async_trait::async_trait;

use crate::{maven::artifact::Artifact, vanilla::manifest::VersionManifest};

#[async_trait]
pub trait LoaderData {
    async fn all_versions(&self) -> Result<Vec<Artifact>>;
    async fn versions_for(&self, game_version: impl AsRef<str> + Send) -> Result<Vec<Artifact>>;
    async fn version_jar_url(&self, version: impl Into<Artifact> + Send) -> Result<String>;

    async fn get_version_manifest(
        &self,
        version: impl Into<Artifact> + Send,
        game_version: impl AsRef<str> + Send,
    ) -> Result<VersionManifest>;
}
