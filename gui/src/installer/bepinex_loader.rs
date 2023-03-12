use std::{fs, path::PathBuf};

use tauri::Window;

use crate::progress::Downloader;

pub struct BepInExLoaderInstallManager {
    pub ksp2_install_path: PathBuf,
    pub zip_url: String,
}

impl BepInExLoaderInstallManager {
    pub fn new(ksp2_install_path: PathBuf) -> Self {
        return BepInExLoaderInstallManager {
            ksp2_install_path,

            // For now, this is hard-coded because SpaceWarp uses BepInEx 5,
            // which is now in LTS mode. I would fetch the latest release
            // from GitHub, but I don't want to accidentally download
            // BepInEx 6 instead.
            zip_url: "https://github.com/BepInEx/BepInEx/releases/download/v5.4.21/BepInEx_x64_5.4.21.0.zip".to_string(),
        };
    }

    pub async fn download(&mut self, window: Window) -> Result<(), String> {
        if !self.ksp2_install_path.is_dir() {
            return Err("KSP2 install path is not a directory!".to_string());
        }

        let files_in_dir = self.ksp2_install_path.read_dir().unwrap();

        for file in files_in_dir {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();

            if file_name.contains("doorstop_config.ini")
                || file_name.contains(".doorstop_version")
                || file_name.contains("winhttp.dll")
                || file_name.contains("version.dll")
                || file_name.contains("BepInEx")
            {
                return Err("BepInEx or another mod loader is already installed!".to_string());
            }
        }

        let download_url = self.zip_url.clone();

        println!("Downloading from URL: {}", download_url);

        let out_file = self.ksp2_install_path.join(".bepinex_release.zip");

        Downloader::download(download_url, out_file, window).await;

        zip_extensions::read::zip_extract(
            &PathBuf::from(&self.ksp2_install_path.join(".bepinex_release.zip")),
            &PathBuf::from(&self.ksp2_install_path),
        )
            .expect("Could not extract the BepInEx release!");

        fs::remove_file(self.ksp2_install_path.join(".bepinex_release.zip"))
            .expect("Could not delete the BepInEx release file!");

        return Ok(());
    }
}
