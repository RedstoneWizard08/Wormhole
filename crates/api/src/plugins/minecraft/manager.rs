use std::{fs, path::PathBuf};

use anyhow::Result;
use data::instance::Instance;
use java::install::install_java;
use mcmeta::{cmd::modded::ModLoader, download::DownloadCallbackFn};
use msa::state::MsaState;
use tokio::process::Child;
use whcore::manager::CoreManager;

use super::{dirs::MinecraftDirs, launch::launch_minecraft};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftManager {
    pub dir: PathBuf,
    pub java: PathBuf,
    pub loader: ModLoader,

    #[serde(skip)]
    pub dirs: MinecraftDirs,
}

impl MinecraftManager {
    pub fn load(dir: PathBuf) -> Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(
            dir.join("instance-metadata.json"),
        )?)?)
    }

    pub async fn load_or_create(dir: PathBuf, loader: &ModLoader) -> Result<Self> {
        if dir.join("instance-metadata.json").exists() {
            Self::load(dir)
        } else {
            let dirs = MinecraftDirs::collect();
            let java = loader.get_java_version().await?;

            let me = Self {
                dir,
                java: install_java(&dirs.java.join(java.to_string()), java).await?,
                dirs,
                loader: loader.clone(),
            };

            me.save()?;

            Ok(me)
        }
    }

    pub fn save(&self) -> Result<()> {
        if !self.dir.exists() {
            fs::create_dir_all(&self.dir)?;
        }

        fs::write(
            self.dir.join("instance-metadata.json"),
            serde_json::to_string(self)?,
        )?;

        Ok(())
    }

    pub async fn install_loader(
        &mut self,
        loader: &ModLoader,
        callback: &Option<DownloadCallbackFn>,
    ) -> Result<()> {
        loader
            .install(
                &self.dirs.libs,
                &self.dirs.natives,
                &self.dirs.temp,
                &self.dirs.assets(loader.mc_version()),
                callback,
            )
            .await?;

        self.loader = loader.clone();
        self.save()?;

        Ok(())
    }

    pub async fn launch(&self, auth: &MsaState, inst: &Instance) -> Result<Child> {
        launch_minecraft(
            &self.java,
            CoreManager::mem(),
            &inst.data_dir(),
            &self.loader,
            auth,
        )
        .await
    }
}
