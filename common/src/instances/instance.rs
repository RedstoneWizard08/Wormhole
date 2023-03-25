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
    pub fn blank() -> Instance {
        return Instance {
            id: -1,
            name: "".to_string(),
            game: KSPGame::KSP1,
            mods: Vec::new(),
            install_path: PathBuf::from(""),
            description: None,
            time_played: None,
        };
    }

    pub fn new_id() -> i32 {
        let mut id = Instance::load().len() as i32 - 1;
        let mut instance = Some(Instance::blank());

        while instance.is_some() {
            id += 1;
            instance = Instance::from_id(id);
        }

        return id;
    }

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
                        .join("KSP2_x64_Data/Plugins/x86_64/steam_api64.dll"),
                };

                if api_dll.exists() {
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

        Instance::set_default_instances();

        return instances;
    }

    pub fn load_no_set() -> Vec<Self> {
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

    pub fn set_default_instances() {
        let ksp1_instance_json_path = find_ksp1_install_dir().join("instance.json");
        let ksp2_instance_json_path = find_ksp2_install_dir().join("instance.json");

        let instances = Instance::load_no_set();

        if !ksp1_instance_json_path.exists() {
            let ksp1_default = instances.iter().find(|i| i.game == KSPGame::KSP1).unwrap();

            let ksp1_default_json = InstanceJson {
                id: ksp1_default.id,
            };

            let ksp1_instance_json = serde_json::to_string(&ksp1_default_json).unwrap();

            let mut file = File::create(ksp1_instance_json_path).unwrap();

            file.write_all(ksp1_instance_json.as_bytes()).unwrap();
        }

        if !ksp2_instance_json_path.exists() {
            let ksp2_default = instances.iter().find(|i| i.game == KSPGame::KSP2).unwrap();

            let ksp2_default_json = InstanceJson {
                id: ksp2_default.id,
            };

            let ksp2_instance_json = serde_json::to_string(&ksp2_default_json).unwrap();

            let mut file = File::create(ksp2_instance_json_path).unwrap();

            file.write_all(ksp2_instance_json.as_bytes()).unwrap();
        }
    }

    pub fn save_all(instances: &Vec<Self>) {
        let instances_path = get_data_dir().join("instances.json");
        let mut file = File::create(instances_path).unwrap();

        file.write_all(serde_json::to_string(&instances).unwrap().as_bytes())
            .unwrap();
    }

    pub fn save(&self) {
        let mut instances = Instance::load();
        let mut found = false;

        for (index, instance) in instances.clone().iter().enumerate() {
            if instance.id == self.id {
                let _ = replace(&mut instances[index], self.clone());

                found = true;
            }
        }

        if !found {
            instances.push(self.clone());
        }

        Instance::save_all(&instances);
    }

    pub fn remove(&self) {
        if self.is_active() {
            self.disable();
        }

        let mut instances = Instance::load();
        let mut instance_index = 999;

        for (index, instance) in instances.clone().iter().enumerate() {
            if instance.id == self.id {
                instance_index = index;
            }
        }

        if instance_index != 999 {
            instances.remove(instance_index);
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
                if path.contains("SpaceWarp") || path.contains("ConfigurationManager") {
                    continue;
                }

                let local_path = self.install_path.join(path.clone());

                let saved_path = get_data_dir()
                    .join("instances")
                    .join(self.id.to_string())
                    .join(path);

                copy_dir_all(saved_path, local_path).unwrap();
            }
        }

        Instance::set_active_instance(self);
    }

    pub fn disable(&self) {
        for instance_mod in self.mods.clone() {
            for path in instance_mod.paths {
                if path.contains("SpaceWarp") || path.contains("ConfigurationManager") {
                    continue;
                }

                let local_path = self.install_path.join(path.clone());

                let saved_path = get_data_dir()
                    .join("instances")
                    .join(self.id.to_string())
                    .join(path);

                if local_path.exists() {
                    copy_dir_all(local_path.clone(), saved_path).unwrap();
                    remove_dir_all(local_path).unwrap();
                }
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
