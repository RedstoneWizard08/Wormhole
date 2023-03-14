use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use crate::mods::spacedock::SpaceDockAPI;

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

        zip_extensions::read::zip_extract(
            &PathBuf::from(&self.install_path.join(".mod.zip")),
            &PathBuf::from(&self.install_path),
        )
        .expect("Could not extract the mod!");

        fs::remove_file(self.install_path.join(".mod.zip"))
            .expect("Could not delete the mod file!");
    }
}
