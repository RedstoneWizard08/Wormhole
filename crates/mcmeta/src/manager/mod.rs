pub mod dirs;
pub mod launch;

use std::{fs, path::PathBuf};

use crate::{cmd::modded::ModLoader, download::DownloadCallbackFn};
use anyhow::Result;
use data::instance::Instance;
use java::install::install_java;
use msa::state::MsaState;
use tokio::process::Child;
use whcore::manager::CoreManager;

use self::{dirs::MinecraftDirs, launch::launch_minecraft};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftManager {
    pub dir: PathBuf,
    pub java: PathBuf,
    pub loader: ModLoader,

    #[serde(skip)]
    pub dirs: MinecraftDirs,
}

impl MinecraftManager {
    pub async fn load(dir: PathBuf) -> Result<Self> {
        let me: MinecraftManager =
            serde_json::from_str(&fs::read_to_string(dir.join("instance-metadata.json"))?)?;
        let java = me.loader.get_java_version().await?;

        info!("Installing Java...");

        install_java(&me.dirs.java.join(java.to_string()), java).await?;

        info!("Installing loader...");

        me.loader
            .install(
                &me.dirs.libs,
                &me.dirs.natives,
                &me.dirs.temp,
                &me.dirs.assets(me.loader.mc_version()),
                &None,
            )
            .await?;

        Ok(me)
    }

    pub async fn load_or_create(dir: PathBuf, loader: &ModLoader) -> Result<Self> {
        if dir.join("instance-metadata.json").exists() {
            Self::load(dir).await
        } else {
            info!("Creating a new manager...");

            let dirs = MinecraftDirs::collect();
            let java = loader.get_java_version().await?;

            info!("Installing Java...");

            let java = install_java(&dirs.java.join(java.to_string()), java).await?;

            info!("Creating manager struct...");

            let me = Self {
                dir,
                java,
                dirs: dirs.clone(),
                loader: loader.clone(),
            };

            info!("Installing loader...");

            loader
                .install(
                    &dirs.libs,
                    &dirs.natives,
                    &dirs.temp,
                    &dirs.assets(loader.mc_version()),
                    &None,
                )
                .await?;

            info!("Saving...");

            me.save()?;

            info!("Done!");

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
