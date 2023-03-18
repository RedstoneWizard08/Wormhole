use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use crate::{mods::spacedock::SpaceDockAPI, util::copy_dir_all};

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

        let response = reqwest::get(url)
            .await
            .expect("Could not download the mod!");

        let body = response.bytes().await.expect("Could not read the mod!");

        let mut out_file = File::create(self.install_path.join(".mod.zip"))
            .expect("Could not create the mod file!");

        io::copy(&mut body.as_ref(), &mut out_file).expect("Could not copy the mod to the file!");
        
        let mod_tmp_path = &self.install_path.join(".mod_tmp");

        if mod_tmp_path.exists() {
            fs::remove_dir_all(mod_tmp_path).expect("Could not delete the mod tmp folder!");
        }

        fs::create_dir_all(mod_tmp_path).expect("Could not create the mod tmp folder!");

        zip_extensions::read::zip_extract(
            &PathBuf::from(&self.install_path.join(".mod.zip")),
            &PathBuf::from(mod_tmp_path),
        )
        .expect("Could not extract the mod!");

        fs::remove_file(self.install_path.join(".mod.zip"))
            .expect("Could not delete the mod file!");

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

        fs::remove_dir_all(mod_tmp_path).expect("Could not delete the mod tmp folder!");
    }
}
