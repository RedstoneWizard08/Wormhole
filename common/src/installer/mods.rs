use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use crate::{
    instances::{Instance, InstanceMod, KSPGame},
    mods::spacedock::SpaceDockAPI,
    util::{copy_dir_all, get_data_dir},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use zip::ZipArchive;

pub struct ModInstaller {
    pub install_path: PathBuf,
}

impl ModInstaller {
    pub fn new(install_path: PathBuf) -> Self {
        return Self { install_path };
    }

    pub async fn install_from_spacedock(&self, id: i32, instance_id: i32) {
        let api = SpaceDockAPI::new();
        let url = api.get_mod_download(id).await;

        let tmp_name = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        let response = reqwest::get(url)
            .await
            .expect("Could not download the mod!");

        let body = response.bytes().await.expect("Could not read the mod!");
        let out_path = &self.install_path.join(format!(".{}.mod.zip", tmp_name));

        let mut out_file = File::create(out_path).expect("Could not create the mod file!");

        io::copy(&mut body.as_ref(), &mut out_file).expect("Could not copy the mod to the file!");

        let mod_tmp_path = &self.install_path.join(format!(".{}.mod_tmp", tmp_name));

        if mod_tmp_path.exists() {
            fs::remove_dir_all(mod_tmp_path).expect("Could not delete the mod tmp folder!");
        }

        fs::create_dir_all(mod_tmp_path).expect("Could not create the mod tmp folder!");

        let zip_file = File::open(out_path).unwrap();
        let mut zip = ZipArchive::new(zip_file).unwrap();
        let zip_size = zip.len();

        let mut files: Vec<String> = Vec::new();

        for i in 0..zip_size {
            let file = zip.by_index(i).unwrap();
            let name = file.name().to_string();

            files.push(name.clone());
        }

        zip.extract(mod_tmp_path)
            .expect("Could not extract the mod!");

        fs::remove_file(out_path).expect("Could not delete the mod file!");

        let mod_info = api.get_mod(id).await;

        if let Some(game_id) = mod_info.game_id {
            if let Some(game) = KSPGame::from_id(game_id) {
                let instance = Instance::from_id(instance_id);

                if let Some(mut instance) = instance {
                    let mut instance_mod = InstanceMod {
                        id: mod_info.id.unwrap(),
                        name: mod_info.name.unwrap(),
                        paths: Vec::new(),
                    };

                    if game.eq(&KSPGame::KSP2) {
                        let bep_in_ex_dir = mod_tmp_path.join("BepInEx");

                        if bep_in_ex_dir.exists() {
                            if instance.is_active() {
                                copy_dir_all(
                                    bep_in_ex_dir.clone(),
                                    self.install_path.join("BepInEx"),
                                )
                                .expect("Could not move the BepInEx folder!");
                            } else {
                                copy_dir_all(
                                    bep_in_ex_dir.clone(),
                                    get_data_dir()
                                        .join("instances")
                                        .join(instance.id.to_string())
                                        .join("BepInEx"),
                                )
                                .expect("Could not move the BepInEx folder!");
                            }

                            for file in bep_in_ex_dir.read_dir().unwrap() {
                                let file = file.unwrap();

                                if file.file_name() == "plugins" || file.file_name() == "config" {
                                    for file2 in file.path().read_dir().unwrap() {
                                        let file2 = file2.unwrap();

                                        instance_mod.paths.push(
                                            "BepInEx/".to_string()
                                                + file.file_name().into_string().unwrap().as_str()
                                                + "/"
                                                + file2.file_name().into_string().unwrap().as_str(),
                                        );
                                    }
                                }

                                instance_mod.paths.push(
                                    "BepInEx/".to_string()
                                        + file.file_name().into_string().unwrap().as_str(),
                                );
                            }
                        } else {
                            if instance.is_active() {
                                copy_dir_all(
                                    mod_tmp_path,
                                    self.install_path.join("BepInEx").join("plugins"),
                                )
                                .expect("Could not move the BepInEx folder!");
                            } else {
                                copy_dir_all(
                                    bep_in_ex_dir.clone(),
                                    get_data_dir()
                                        .join("instances")
                                        .join(instance.id.to_string())
                                        .join("BepInEx")
                                        .join("plugins"),
                                )
                                .expect("Could not move the BepInEx folder!");
                            }

                            for file in bep_in_ex_dir.read_dir().unwrap() {
                                let file = file.unwrap();

                                instance_mod.paths.push(
                                    "BepInEx/plugins/".to_string()
                                        + file.file_name().into_string().unwrap().as_str(),
                                );
                            }
                        }
                    } else {
                        let mod_contents =
                            fs::read_dir(mod_tmp_path).expect("Could not read the mod tmp folder!");

                        let files = mod_contents
                            .filter_map(|entry| entry.ok())
                            .collect::<Vec<_>>();

                        let files_strs = files
                            .iter()
                            .map(|entry| entry.file_name().into_string().unwrap())
                            .collect::<Vec<_>>();

                        if files_strs.contains(&"GameData".to_string()) {
                            if instance.is_active() {
                                copy_dir_all(mod_tmp_path, self.install_path.clone())
                                    .expect("Could not move the GameData folder!");
                            } else {
                                copy_dir_all(
                                    mod_tmp_path,
                                    get_data_dir()
                                        .join("instances")
                                        .join(instance.id.to_string()),
                                )
                                .expect("Could not move the GameData folder!");
                            }

                            for file in mod_tmp_path.read_dir().unwrap() {
                                let file = file.unwrap();

                                if file.file_name() == "GameData" {
                                    for file2 in file.path().read_dir().unwrap() {
                                        let file2 = file2.unwrap();

                                        instance_mod.paths.push(
                                            "GameData/".to_string()
                                                + file2.file_name().into_string().unwrap().as_str(),
                                        );
                                    }
                                }

                                instance_mod
                                    .paths
                                    .push(file.file_name().into_string().unwrap());
                            }
                        } else {
                            if instance.is_active() {
                                copy_dir_all(mod_tmp_path, self.install_path.join("GameData"))
                                    .expect("Could not move the GameData folder!");
                            } else {
                                copy_dir_all(
                                    mod_tmp_path,
                                    get_data_dir()
                                        .join("instances")
                                        .join(instance.id.to_string())
                                        .join("GameData"),
                                )
                                .expect("Could not move the GameData folder!");
                            }

                            for file in mod_tmp_path.read_dir().unwrap() {
                                let file = file.unwrap();

                                instance_mod.paths.push(
                                    "GameData/".to_string()
                                        + file.file_name().into_string().unwrap().as_str(),
                                );
                            }
                        }
                    }

                    instance.mods.push(instance_mod);
                    instance.save();
                }
            }
        }

        fs::remove_dir_all(mod_tmp_path).expect("Could not delete the mod tmp folder!");
    }
}
