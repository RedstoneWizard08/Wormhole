use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{util::get_data_dir, finder::{find_ksp2_install_dir, find_ksp1_install_dir}};

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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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
    pub install_path: PathBuf,
    pub description: Option<String>,
    pub time_played: Option<String>,
}

impl KSPGame {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            3102 => Some(KSPGame::KSP1),
            22407 => Some(KSPGame::KSP2),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        return match self {
            KSPGame::KSP1 => 3102,
            KSPGame::KSP2 => 22407,
        };
    }
}

impl InstanceInfo {
    pub fn defaults() -> Vec<Self> {
        let v = vec![
            InstanceInfo {
                id: 0,
                name: "KSP2 Default Instance".to_string(),
                game: KSPGame::KSP2,
                install_path: find_ksp2_install_dir(),
                mods: Vec::new(),
                description: None,
                time_played: None,
            },

            InstanceInfo {
                id: 1,
                name: "KSP1 Default Instance".to_string(),
                game: KSPGame::KSP1,
                install_path: find_ksp1_install_dir(),
                mods: Vec::new(),
                description: None,
                time_played: None,
            },
        ];

        return v;
    }

    pub fn fetch() -> Vec<Self> {
        let instances;
        let instances_path = get_data_dir().join("instances.json");

        if instances_path.exists() {
            let file = std::fs::File::open(instances_path).unwrap();
            let reader = std::io::BufReader::new(file);

            instances = serde_json::from_reader(reader).unwrap();
        } else {
            instances = InstanceInfo::defaults();

            InstanceInfo::save(&instances);
        }

        return instances;
    }

    pub fn save(instances: &Vec<Self>) {
        let instances_path = get_data_dir().join("instances.json");
        let file = std::fs::File::create(instances_path).unwrap();
        let writer = std::io::BufWriter::new(file);

        serde_json::to_writer_pretty(writer, instances).unwrap();
    }
}
