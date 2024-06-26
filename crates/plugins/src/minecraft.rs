//! The Minecraft support module.

use std::path::PathBuf;

use anyhow::Result;
use api::plugin::Plugin;
use base64::{engine::general_purpose::STANDARD, Engine};
use data::Instance;
use mcmeta::{cmd::modded::ModLoader, manager::MinecraftManager};
use msa::state::MsaState;
use query::{curse::CurseForge, modrinth::Modrinth, source::Resolver};
use tokio::process::Child;
use whcore::manager::CoreManager;

const ICON_BYTES: &[u8] = include_bytes!("../assets/minecraft/icon.svg");
const BANNER_BYTES: &[u8] = include_bytes!("../assets/minecraft/banner.jpg");

/// The plugin for Minecraft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinecraftPlugin;

#[async_trait]
impl Plugin for MinecraftPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> &'static str {
        "MC".into()
    }

    fn game(&self) -> i32 {
        432
    }

    async fn loader(&self, instance: Instance) -> Result<ModLoader> {
        info!("Fetching mod loader...");

        let loader = ModLoader::quilt_latest().await?;

        info!("Creating manager...");

        // We have to do this so that we can install the loader too
        let manager = MinecraftManager::load_or_create(instance.data_dir(), &loader).await?;

        Ok(manager.loader)
    }

    async fn create_resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(Modrinth::new().await),
            Box::new(CurseForge::new().await),
        ]
    }

    async fn install_loader(&self, instance: &Instance, loader: &ModLoader) -> Result<()> {
        info!("Creating manager...");

        let mut manager = MinecraftManager::load_or_create(instance.data_dir(), loader).await?;

        info!("Installing loader...");

        manager.install_loader(loader, &None).await?;
        manager.save()
    }

    fn display(&self) -> String {
        "Minecraft".into()
    }

    fn icon(&self) -> String {
        format!("data:image/svg+xml;base64,{}", STANDARD.encode(ICON_BYTES))
    }

    fn banner(&self) -> String {
        format!("data:image/jpeg;base64,{}", STANDARD.encode(BANNER_BYTES))
    }

    fn find(&self) -> Option<PathBuf> {
        Some(CoreManager::get().game_data_dir("minecraft"))
    }

    fn name(&self) -> &'static str {
        "minecraft"
    }

    fn fallback(&self) -> Option<&'static str> {
        Some("mods")
    }

    async fn launch(&self, instance: Instance) -> Result<Child> {
        info!("Fetching mod loader...");

        let loader = ModLoader::quilt_latest().await?;

        info!("Creating manager...");

        let manager = MinecraftManager::load_or_create(instance.data_dir(), &loader).await?;

        info!("Launching...");

        manager.launch(&MsaState::get(), &instance).await
    }

    async fn install_instance(&self, inst: &Instance) -> Result<()> {
        info!("Fetching mod loader...");

        let loader = ModLoader::vanilla_latest().await?;

        info!("Creating manager and installing loader...");

        MinecraftManager::load_or_create(inst.data_dir(), &loader).await?;

        Ok(())
    }
}
