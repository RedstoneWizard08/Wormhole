use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use data::instance::Instance;
use msa::state::MsaState;
use query::{curse::CurseForge, modrinth::Modrinth, source::Resolver};
use tokio::process::Child;

use crate::plugin::Plugin;

use super::manager::MinecraftManager;

pub const ICON_BYTES: &[u8] = include_bytes!("../../assets/minecraft/icon.svg");
pub const BANNER_BYTES: &[u8] = include_bytes!("../../assets/minecraft/banner.jpg");

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

    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(CurseForge::new().await),
            Box::new(Modrinth::new().await),
        ]
    }

    async fn launch(&self, instance: Instance) -> Result<Child> {
        let manager = MinecraftManager::load(instance.data_dir())?;

        manager.launch(&MsaState::get(), &instance).await
    }
}
