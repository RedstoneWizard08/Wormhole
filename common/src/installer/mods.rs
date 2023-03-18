use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use rand::{Rng, distributions::Alphanumeric, thread_rng};
use crate::{instances::KSPGame, mods::spacedock::SpaceDockAPI, util::copy_dir_all};

pub struct ModInstaller {
    pub install_path: PathBuf,
}

impl ModInstaller {
    pub fn new(install_path: PathBuf) -> Self {
        return Self { install_path };
    }

    pub async fn install_from_spacedock(&self, id: i32) {
        let api = SpaceDockAPI::new();
        let url = api.get_mod_download(id).await;
        let tmp_name = thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>();

        let response = reqwest::get(url)
            .await
            .expect("Could not download the mod!");

        let body = response.bytes().await.expect("Could not read the mod!");
        let out_path = &self.install_path.join(format!(".{}.mod.zip", tmp_name));

        let mut out_file = File::create(out_path)
            .expect("Could not create the mod file!");

        io::copy(&mut body.as_ref(), &mut out_file).expect("Could not copy the mod to the file!");

        let mod_tmp_path = &self.install_path.join(format!(".{}.mod_tmp", tmp_name));

        if mod_tmp_path.exists() {
            fs::remove_dir_all(mod_tmp_path).expect("Could not delete the mod tmp folder!");
        }

        fs::create_dir_all(mod_tmp_path).expect("Could not create the mod tmp folder!");

        zip_extensions::read::zip_extract(
            &PathBuf::from(out_path),
            &PathBuf::from(mod_tmp_path),
        )
        .expect("Could not extract the mod!");

        fs::remove_file(out_path)
            .expect("Could not delete the mod file!");

        let mod_info = api.get_mod(id).await;

        if let Some(game_id) = mod_info.game_id {
            if let Some(game) = KSPGame::from_id(game_id) {
                if game.eq(&KSPGame::KSP2) {
                    let bep_in_ex_dir = mod_tmp_path.join("BepInEx");

                    if bep_in_ex_dir.exists() {
                        copy_dir_all(bep_in_ex_dir, self.install_path.join("BepInEx"))
                            .expect("Could not move the BepInEx folder!");
                    } else {
                        copy_dir_all(
                            mod_tmp_path,
                            self.install_path.join("BepInEx").join("plugins"),
                        )
                        .expect("Could not move the BepInEx folder!");
                    }
                } else {
                    let mod_contents = fs::read_dir(mod_tmp_path)
                        .expect("Could not read the mod tmp folder!");
                    
                    let files = mod_contents.filter_map(|entry| entry.ok()).collect::<Vec<_>>();
                    let files_strs = files.iter().map(|entry| entry.file_name().into_string().unwrap()).collect::<Vec<_>>();

                    if files_strs.contains(&"GameData".to_string()) {
                        copy_dir_all(mod_tmp_path, self.install_path.clone())
                            .expect("Could not move the GameData folder!");
                    } else {
                        copy_dir_all(mod_tmp_path, self.install_path.join("GameData"))
                            .expect("Could not move the GameData folder!");
                    }
                }
            }
        }

        fs::remove_dir_all(mod_tmp_path).expect("Could not delete the mod tmp folder!");
    }
}
