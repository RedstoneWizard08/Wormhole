use std::{
    fs::{File, remove_dir_all},
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    finder::{find_ksp1_install_dir, find_ksp2_install_dir},
    util::{get_data_dir, copy_dir_all},
};

// The expected size of KSP1's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP1_ROOT]/KSP_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP1 Installed Files
pub const KSP1_STEAM_API_SIZE: u64 = 249120;

// The expected size of KSP2's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP2_ROOT]/KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP2 Installed Files
pub const KSP2_STEAM_API_SIZE: u64 = 295336;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceJson {
    pub id: i32,
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

        return InstanceInfo::validate_instances(v);
    }

    pub fn validate_instances(instances: Vec<InstanceInfo>) -> Vec<InstanceInfo> {
        let mut final_instances = Vec::new();

        for instance in instances {
            if instance.install_path.exists() {
                let api_dll = match instance.game {
                    KSPGame::KSP1 => instance.install_path.join("KSP_x64_Data/Plugins/x86_64/steam_api64.dll"),
                    KSPGame::KSP2 => instance.install_path.join("KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`"),
                };

                let size = api_dll.metadata().unwrap().len();

                let needed_size = match instance.game {
                    KSPGame::KSP1 => KSP1_STEAM_API_SIZE,
                    KSPGame::KSP2 => KSP2_STEAM_API_SIZE,
                };

                if size == needed_size {
                    final_instances.push(instance);
                }
            }
        }

        return final_instances;
    }

    pub fn load() -> Vec<Self> {
        let instances;
        let instances_path = get_data_dir().join("instances.json");

        if instances_path.exists() {
            let mut file = File::open(instances_path).unwrap();
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();
            instances = serde_json::from_str(&content).unwrap();
        } else {
            instances = InstanceInfo::defaults();

            InstanceInfo::save(&instances);
        }

        return instances;
    }

    pub fn save(instances: &Vec<Self>) {
        let instances_path = get_data_dir().join("instances.json");
        let mut file = File::create(instances_path).unwrap();

        file.write(serde_json::to_string(&instances).unwrap().as_bytes())
            .unwrap();
    }

    pub fn from_id(id: i32) -> Option<InstanceInfo> {
        let instances = InstanceInfo::load();
        let instance = instances.iter().find(|i| i.id == id).cloned();

        return instance;
    }

    pub fn load_from_file(file_path: PathBuf) -> Option<InstanceInfo> {
        let mut file = File::open(file_path).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        let parsed = serde_json::from_str::<InstanceJson>(&content);

        if let Ok(instance_json) = parsed {
            return InstanceInfo::from_id(instance_json.id);
        }

        return None;
    }

    pub fn get_active_instance(game: KSPGame) -> Option<InstanceInfo> {
        let instance_info_file = (match game {
            KSPGame::KSP1 => find_ksp1_install_dir(),
            KSPGame::KSP2 => find_ksp2_install_dir(),
        })
        .join("instance.json");

        if instance_info_file.exists() {
            let instance = InstanceInfo::load_from_file(instance_info_file);

            return instance;
        }

        return None;
    }

    pub fn enable(&self) {
        let active = InstanceInfo::get_active_instance(self.game.clone());

        if let Some(instance) = active {
            instance.disable();
        }

        for instance_mod in self.mods.clone() {
            for path in instance_mod.paths {
                let local_path = self.install_path.join(path.clone());

                let saved_path = get_data_dir()
                    .join("instances")
                    .join(self.id.to_string())
                    .join(path);
                
                copy_dir_all(saved_path, local_path).unwrap();
            }
        }
    }

    pub fn disable(&self) {
        for instance_mod in self.mods.clone() {
            for path in instance_mod.paths {
                let local_path = self.install_path.join(path.clone());

                let saved_path = get_data_dir()
                    .join("instances")
                    .join(self.id.to_string())
                    .join(path);
                
                copy_dir_all(local_path.clone(), saved_path).unwrap();
                remove_dir_all(local_path).unwrap();
            }
        }
    }
}
