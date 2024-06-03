//! The KSP2 support module.

use std::path::PathBuf;

use base64::{engine::general_purpose::STANDARD, Engine};
use query::{ckan::Ckan, source::Resolver, spacedock::SpaceDock};
use whcore::finder::{finder::InstallFinder, pdlauncher::PrivateDivision, steam::Steam};

use super::common::unity::UnityPlugin;

const ICON_BYTES: &[u8] = include_bytes!("../assets/ksp2/icon.png");
const BANNER_BYTES: &[u8] = include_bytes!("../assets/ksp2/banner.png");

/// The expected size of KSP2's `steam_api64.dll` in bytes.
/// This helps to make sure that the game is not pirated.
/// File path: `[KSP2_ROOT]/KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`
/// Information from: SteamDB, DepotDownloader, KSP2 Installed Files
///
/// TODO: Actually use this information somewhere.
pub const KSP2_STEAM_API_SIZE: u64 = 295336;

/// The plugin for KSP2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kerbal2Plugin;

#[async_trait]
impl UnityPlugin for Kerbal2Plugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> &'static str {
        "KSP2".into()
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

    fn executable(&self) -> &'static str {
        "KSP2_x64.exe"
    }

    fn find(&self) -> Option<PathBuf> {
        Steam::new()
            .chain(self.display(), PrivateDivision::new())
            .unwrap()
    }

    fn name(&self) -> &'static str {
        "ksp2"
    }

    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(SpaceDock::new().await),
            Box::new(Ckan::new().await),
        ]
    }
}
