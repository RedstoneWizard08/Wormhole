use std::{env::consts::OS, path::PathBuf};

use anyhow::Result;
use java::install::install_java;
use whcore::{async_trait::async_trait, async_traits::AsyncDefault, manager::CORE_MANAGER};

use crate::{
    download::DownloadCallbackFn,
    fabric::{get_fabric_profile, get_fabric_versions},
    forge::{get_forge_manifest, get_forge_versions, install::install_forge, parse_forge_version},
    neoforge::{
        get_neoforge_manifest, get_neoforge_versions, install::install_neoforge,
        parse_neoforge_version,
    },
    piston::{
        download::download_libs,
        game::{get_game_manifest, manifest::GameManifest},
        install::install_minecraft,
        manifest::get_manifest,
    },
    quilt::{get_quilt_profile, get_quilt_versions},
};

use super::{cmd::build_launch_command, options::LaunchOptions};

/// The ModLoader type.
/// Each element contains the Minecraft version
/// and its version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ModLoader {
    Vanilla(String),
    Forge(String, String),
    NeoForge(String, String),
    Fabric(String, String),
    Quilt(String, String),
}

impl ModLoader {
    pub async fn vanilla_latest() -> Result<Self> {
        let ver = get_manifest().await?.latest.release;

        Ok(Self::Vanilla(ver))
    }

    pub async fn forge_latest() -> Result<Self> {
        let vers = get_forge_versions().await?;
        let ver = vers.first().unwrap();
        let (mc, ver) = parse_forge_version(ver.to_string());

        Ok(Self::Forge(mc, ver))
    }

    pub async fn fabric_latest() -> Result<Self> {
        let ver = get_fabric_versions().await?.versioning.latest;
        let mc = get_manifest().await?.latest.release;

        Ok(Self::Fabric(mc, ver))
    }

    pub async fn neo_latest() -> Result<Self> {
        let vers = get_neoforge_versions().await?;
        let ver = vers.first().unwrap();
        let (mc, ver) = parse_neoforge_version(ver.to_string());

        Ok(Self::NeoForge(mc, ver))
    }

    pub async fn quilt_latest() -> Result<Self> {
        let ver = get_quilt_versions().await?.versioning.latest;
        let mc = get_manifest().await?.latest.release;

        Ok(Self::Quilt(mc, ver))
    }

    pub fn mc_version(&self) -> String {
        match self.clone() {
            ModLoader::Vanilla(ver) => ver,
            ModLoader::Forge(mc, _) => mc,
            ModLoader::NeoForge(mc, _) => mc,
            ModLoader::Fabric(mc, _) => mc,
            ModLoader::Quilt(mc, _) => mc,
        }
    }

    pub fn loader_version(&self) -> String {
        match self.clone() {
            ModLoader::Vanilla(ver) => ver,
            ModLoader::Forge(_, ver) => ver,
            ModLoader::NeoForge(_, ver) => ver,
            ModLoader::Fabric(_, ver) => ver,
            ModLoader::Quilt(_, ver) => ver,
        }
    }

    pub async fn get_manifest(&self) -> Result<GameManifest> {
        Ok(match self.clone() {
            ModLoader::Vanilla(ver) => {
                get_game_manifest(get_manifest().await?.find(ver).unwrap().url).await?
            }

            ModLoader::Fabric(mc, ver) => get_fabric_profile(mc, ver).await?.resolve().await?,
            ModLoader::Forge(_, ver) => get_forge_manifest(ver).await?.resolve().await?,
            ModLoader::NeoForge(_, ver) => get_neoforge_manifest(ver).await?.resolve().await?,
            ModLoader::Quilt(mc, ver) => get_quilt_profile(mc, ver).await?.resolve().await?,
        })
    }

    pub async fn install_to(
        &self,
        dir: PathBuf,
        callback: &Option<DownloadCallbackFn>,
    ) -> Result<()> {
        self.install(
            dir.join("libraries"),
            dir.join("tmp"),
            dir.join("assets"),
            callback,
        )
        .await
    }

    pub async fn install(
        &self,
        lib_dir: PathBuf,
        tmp_dir: PathBuf,
        assets_dir: PathBuf,
        callback: &Option<DownloadCallbackFn>,
    ) -> Result<()> {
        let java = self.install_java().await?;

        install_minecraft(&lib_dir, &assets_dir, self.mc_version(), callback).await?;

        match self.clone() {
            ModLoader::Forge(_, ver) => {
                install_forge(java, lib_dir, tmp_dir, ver, callback).await?
            }

            ModLoader::NeoForge(_, ver) => {
                install_neoforge(java, lib_dir, tmp_dir, ver, callback).await?
            }

            _ => download_libs(lib_dir, self.get_manifest().await?, callback).await?,
        };

        Ok(())
    }

    pub async fn get_java_version(&self) -> Result<i32> {
        Ok(self.get_manifest().await?.java_version.major_version)
    }

    pub async fn install_java(&self) -> Result<PathBuf> {
        let java = self.get_java_version().await?;
        let dir = CORE_MANAGER.game_data_dir("java").join(java.to_string());

        install_java(&dir, java).await?;

        let ext = if OS == "windows" { ".exe" } else { "" };
        let bin = dir.join("bin").join(format!("java{}", ext));

        Ok(bin)
    }

    pub async fn cmd(&self, root: PathBuf, opts: LaunchOptions) -> Result<Vec<String>> {
        Ok(build_launch_command(self.install_java().await?, root, self, opts).await?)
    }
}

#[async_trait]
impl AsyncDefault for ModLoader {
    async fn default() -> Self {
        Self::vanilla_latest().await.unwrap()
    }
}
