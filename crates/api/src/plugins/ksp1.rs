use base64::{engine::general_purpose::STANDARD, Engine};
use query::{ckan::Ckan, source::Resolver, spacedock::SpaceDock};

use crate::plugin::Plugin;

pub const ICON_BYTES: &[u8] = include_bytes!("../assets/ksp1/icon.png");
pub const BANNER_BYTES: &[u8] = include_bytes!("../assets/ksp1/banner.png");

// The expected size of KSP1's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP1_ROOT]/KSP_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP1 Installed Files
pub const KSP1_STEAM_API_SIZE: u64 = 249120;

pub struct Kerbal1Plugin;

#[async_trait]
impl Plugin for Kerbal1Plugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> String {
        "Kerbal Space Program".into()
    }

    fn game(&self) -> i32 {
        3102
    }

    fn icon(&self) -> String {
        format!("data:image/png;base64,{}", STANDARD.encode(ICON_BYTES))
    }

    fn banner(&self) -> String {
        format!("data:image/png;base64,{}", STANDARD.encode(BANNER_BYTES))
    }

    fn display(&self) -> String {
        "Kerbal Space Program".into()
    }

    fn fallback(&self) -> Option<&str> {
        Some("GameData")
    }

    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(SpaceDock::new().await),
            Box::new(Ckan::new().await),
        ]
    }
}
