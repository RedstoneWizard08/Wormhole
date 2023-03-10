use serde::{Deserialize, Serialize};

// The expected size of KSP1's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP1_ROOT]/KSP_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP1 Installed Files
pub const KSP1_STEAM_API_SIZE: i32 = 249120;

// The expected size of KSP2's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP2_ROOT]/KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP2 Installed Files
pub const KSP2_STEAM_API_SIZE: i32 = 295336;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KSPGame {
    KSP1 = 3102,
    KSP2 = 22407,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceMod {
    pub id: i32,
    pub name: String,
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceInfo {
    pub id: i32,
    pub name: String,
    pub game: KSPGame,
    pub mods: Vec<InstanceMod>,
    pub install_path: String,
    pub description: Option<String>,
    pub time_played: Option<String>,
}

impl InstanceInfo {
    pub fn defaults() -> Vec<Self> {
        let v = vec![
            InstanceInfo {
                id: 0,
                name: "KSP2 Default Instance".to_string(),
                game: KSPGame::KSP2,
                install_path: "/home/steam/.steam/root/steamapps/common/Kerbal Space Program 2"
                    .to_string(),
                mods: Vec::new(),
                description: None,
                time_played: None,
            },
            InstanceInfo {
                id: 1,
                name: "KSP1 Default Instance".to_string(),
                game: KSPGame::KSP1,
                install_path: "/home/steam/.steam/root/steamapps/common/Kerbal Space Program"
                    .to_string(),
                mods: Vec::new(),
                description: None,
                time_played: None,
            },
        ];

        return v;
    }
}
