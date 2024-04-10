use base64::{engine::general_purpose::STANDARD, Engine};
use query::{ckan::Ckan, source::Resolver, spacedock::SpaceDock};

use crate::plugin::Plugin;

pub const ICON_BYTES: &[u8] = include_bytes!("../assets/ksp2/icon.png");
pub const BANNER_BYTES: &[u8] = include_bytes!("../assets/ksp2/banner.png");

// The expected size of KSP2's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP2_ROOT]/KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP2 Installed Files
pub const KSP2_STEAM_API_SIZE: u64 = 295336;

pub struct Kerbal2Plugin;

#[async_trait]
impl Plugin for Kerbal2Plugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> String {
        "Kerbal Space Program 2".into()
    }

    fn game(&self) -> i32 {
        22407
    }

    fn icon(&self) -> String {
        format!("data:image/png;base64,{}", STANDARD.encode(ICON_BYTES))
    }

    fn banner(&self) -> String {
        format!("data:image/png;base64,{}", STANDARD.encode(BANNER_BYTES))
    }

    fn display(&self) -> String {
        "Kerbal Space Program 2".into()
    }

    fn fallback(&self) -> Option<&'static str> {
        Some("BepInEx/plugins")
    }

    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(SpaceDock::new().await),
            Box::new(Ckan::new().await),
        ]
    }
}
