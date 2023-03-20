use super::{InstanceJson, InstanceMod, KSPGame, KSP1_STEAM_API_SIZE, KSP2_STEAM_API_SIZE};

use crate::{
    finder::{find_ksp1_install_dir, find_ksp2_install_dir},
    util::{copy_dir_all, get_data_dir},
};

use serde::{Deserialize, Serialize};
use std::{
    fs::{remove_dir_all, File},
    io::{Read, Write},
    mem::replace,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub id: i32,
    pub name: String,
    pub game: KSPGame,
    pub mods: Vec<InstanceMod>,
    pub install_path: PathBuf,
    pub description: Option<String>,
    pub time_played: Option<String>,
}

impl Instance {
    pub fn defaults() -> Vec<Self> {
        let v = vec![
            Instance {
                id: 0,
                name: "KSP2 Default Instance".to_string(),
                game: KSPGame::KSP2,
                install_path: find_ksp2_install_dir(),
                mods: Vec::new(),
                description: None,
                time_played: None,
            },
            Instance {
                id: 1,
                name: "KSP1 Default Instance".to_string(),
                game: KSPGame::KSP1,
                install_path: find_ksp1_install_dir(),
                mods: Vec::new(),
                description: None,
                time_played: None,
            },
        ];

        return Instance::validate_instances(v);
    }

    pub fn validate_instances(instances: Vec<Instance>) -> Vec<Instance> {
        let mut final_instances = Vec::new();

        for instance in instances {
            if instance.install_path.exists() {
                let api_dll = match instance.game {
                    KSPGame::KSP1 => instance
                        .install_path
                        .join("KSP_x64_Data/Plugins/x86_64/steam_api64.dll"),
                    KSPGame::KSP2 => instance
                        .install_path
                        .join("KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`"),
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
            instances = Instance::defaults();

            Instance::save_all(&instances);
        }

        return instances;
    }

    pub fn save_all(instances: &Vec<Self>) {
        let instances_path = get_data_dir().join("instances.json");
        let mut file = File::create(instances_path).unwrap();

        file.write_all(serde_json::to_string(&instances).unwrap().as_bytes())
            .unwrap();
    }

    pub fn save(&self) {
        let mut instances = Instance::load();

        for (index, instance) in instances.clone().iter().enumerate() {
            if instance.id == self.id {
                let _ = replace(&mut instances[index], self.clone());
            }
        }

        Instance::save_all(&instances);
    }

    pub fn from_id(id: i32) -> Option<Instance> {
        let instances = Instance::load();
        let instance = instances.iter().find(|i| i.id == id).cloned();

        return instance;
    }

    pub fn load_from_file(file_path: PathBuf) -> Option<Instance> {
        let mut file = File::open(file_path).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        let parsed = serde_json::from_str::<InstanceJson>(&content);

        if let Ok(instance_json) = parsed {
            return Instance::from_id(instance_json.id);
        }

        return None;
    }

    pub fn get_active_instance(game: KSPGame) -> Option<Instance> {
        let instance_info_file = (match game {
            KSPGame::KSP1 => find_ksp1_install_dir(),
            KSPGame::KSP2 => find_ksp2_install_dir(),
        })
        .join("instance.json");

        if instance_info_file.exists() {
            let instance = Instance::load_from_file(instance_info_file);

            return instance;
        }

        return None;
    }

    pub fn set_active_instance(instance: &Instance) {
        let instance_info_file_path = (match instance.game {
            KSPGame::KSP1 => find_ksp1_install_dir(),
            KSPGame::KSP2 => find_ksp2_install_dir(),
        })
        .join("instance.json");

        let mut instance_info_file = File::create(instance_info_file_path).unwrap();

        instance_info_file
            .write_all(
                serde_json::to_string(&InstanceJson { id: instance.id })
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }

    pub fn enable(&self) {
        let active = Instance::get_active_instance(self.game.clone());

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

        Instance::set_active_instance(&self);
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

    pub fn is_active(&self) -> bool {
        let active = Instance::get_active_instance(self.game.clone());

        if let Some(active) = active {
            if active.id == self.id {
                return true;
            }
        }

        return false;
    }
}
