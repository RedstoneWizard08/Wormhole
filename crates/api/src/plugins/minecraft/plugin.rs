use std::path::PathBuf;

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use data::instance::Instance;
use mcmeta::cmd::modded::ModLoader;
use msa::state::MsaState;
use query::{curse::CurseForge, modrinth::Modrinth, source::Resolver};
use tokio::process::Child;
use whcore::manager::CoreManager;

use crate::plugin::Plugin;

use super::manager::MinecraftManager;

pub const ICON_BYTES: &[u8] = include_bytes!("../../assets/minecraft/icon.svg");
pub const BANNER_BYTES: &[u8] = include_bytes!("../../assets/minecraft/banner.jpg");

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

    fn icon(&self) -> String {
        format!("data:image/svg+xml;base64,{}", STANDARD.encode(ICON_BYTES))
    }

    fn banner(&self) -> String {
        format!("data:image/jpeg;base64,{}", STANDARD.encode(BANNER_BYTES))
    }

    fn display(&self) -> String {
        "Minecraft".into()
    }

    fn fallback(&self) -> Option<&'static str> {
        Some("mods")
    }

    fn find(&self) -> Option<PathBuf> {
        Some(CoreManager::get().game_data_dir("minecraft"))
    }

    fn name(&self) -> &'static str {
        "minecraft"
    }

    async fn create_resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(Modrinth::new().await),
            Box::new(CurseForge::new().await),
        ]
    }

    async fn launch(&self, instance: Instance) -> Result<Child> {
        info!("Fetching mod loader...");

        let loader = ModLoader::vanilla_latest().await?;

        info!("Creating manager...");

        let manager = MinecraftManager::load_or_create(instance.data_dir(), &loader).await?;

        info!("Launching...");

        manager.launch(&MsaState::get(), &instance).await
    }
}
