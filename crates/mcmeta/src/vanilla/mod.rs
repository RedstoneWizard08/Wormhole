//! The vanilla module.

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use manifest::{
    libraries::{Library, LibraryDownloads, LibraryFile},
    VersionManifest,
};
use meta::{PistonMetaManifest, PistonMetaVersion};
use whcore::async_trait::async_trait;

use crate::{loader::LoaderData, maven::artifact::Artifact};

pub mod manifest;
pub mod meta;

pub const META_API: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
pub struct Vanilla;

impl Vanilla {
    pub async fn get_manifest(&self) -> Result<PistonMetaManifest> {
        Ok(reqwest::get(META_API)
            .await?
            .json::<PistonMetaManifest>()
            .await?)
    }

    pub async fn get_versions(&self) -> Result<Vec<PistonMetaVersion>> {
        Ok(self.get_manifest().await?.versions)
    }

    pub async fn get_latest_release(&self) -> Result<String> {
        Ok(self.get_manifest().await?.latest.release)
    }

    pub async fn get_latest_snapshot(&self) -> Result<String> {
        Ok(self.get_manifest().await?.latest.snapshot)
    }

    pub async fn get_manifest_for(&self, version: impl AsRef<str>) -> Result<VersionManifest> {
        let version = version.as_ref();

        let mut mf = reqwest::get(
            self.get_versions()
                .await?
                .iter()
                .find(|v| v.id == version)
                .ok_or(anyhow!(
                    "Could not find a manifest URL for version {}!",
                    version
                ))?
                .url
                .clone(),
        )
        .await?
        .json::<VersionManifest>()
        .await?;

        mf.libraries.push(Library {
            downloads: LibraryDownloads {
                artifact: Some(LibraryFile {
                    path: format!("net/minecraft/client/{0}/client-{0}.jar", version),
                    sha1: Some(mf.downloads.client.sha1.clone()),
                    size: Some(mf.downloads.client.size),
                    url: mf.downloads.client.url.clone(),
                }),
                classifiers: HashMap::new(),
            },
            name: format!("net.minecraft:client:{}", version).into(),
            rules: Vec::new(),
        });

        Ok(mf)
    }
}

#[async_trait]
impl LoaderData for Vanilla {
    async fn all_versions(&self) -> Result<Vec<Artifact>> {
        Ok(self
            .get_versions()
            .await?
            .iter()
            .map(|v| format!("net.minecraft:minecraft:{}", v.id).into())
            .collect())
    }

    async fn versions_for(&self, game_version: impl AsRef<str> + Send) -> Result<Vec<Artifact>> {
        Ok(vec![format!(
            "net.minecraft:minecraft:{}",
            game_version.as_ref()
        )
        .into()])
    }

    async fn version_jar_url(&self, artifact: impl Into<Artifact> + Send) -> Result<String> {
        Ok(self
            .get_manifest_for(artifact.into().version.unwrap())
            .await?
            .downloads
            .client
            .url)
    }

    async fn get_version_manifest(
        &self,
        _version: impl Into<Artifact> + Send,
        game_version: impl AsRef<str> + Send,
    ) -> Result<VersionManifest> {
        self.get_manifest_for(game_version).await
    }
}
