use base64::{engine::general_purpose::STANDARD, Engine};
use query::{curse::CurseForge, modrinth::Modrinth, source::Resolver};

use crate::plugin::Plugin;

pub const ICON_BYTES: &[u8] = include_bytes!("../assets/minecraft/icon.svg");
pub const BANNER_BYTES: &[u8] = include_bytes!("../assets/minecraft/banner.jpg");

pub struct MinecraftPlugin;

#[async_trait]
impl Plugin for MinecraftPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> String {
        "Minecraft".into()
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

    fn fallback(&self) -> Option<&str> {
        Some("mods")
    }

    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(CurseForge::new().await),
            Box::new(Modrinth::new().await),
        ]
    }
}
