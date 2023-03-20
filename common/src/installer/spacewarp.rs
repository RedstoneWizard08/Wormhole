use crate::{
    downloader::Downloader, installer::bepinex::BepInExInstallManager,
    releases::get_latest_release_zips,
};

use std::{fs, path::PathBuf};

pub struct SpaceWarpInstallManager {
    pub ksp2_install_path: PathBuf,
    pub zip_url: Option<String>,
}

impl SpaceWarpInstallManager {
    pub fn new(ksp2_install_path: PathBuf) -> Self {
        return SpaceWarpInstallManager {
            ksp2_install_path,
            zip_url: None,
        };
    }

    pub async fn resolve(&mut self) -> Result<(), String> {
        let latest_release = get_latest_release_zips().await;

        if let Some(release) = latest_release {
            self.zip_url = Some(release);
        } else {
            return Err("No SpaceWarp release found!".to_string());
        }

        return Ok(());
    }

    pub async fn download<W>(
        &mut self,
        on_progress: fn(u64, usize, W) -> (),
        on_finish: fn(u64, W) -> (),
        window: W,
    ) -> Result<(), String>
    where
        W: Clone,
    {
        if !self.ksp2_install_path.is_dir() {
            return Err("KSP2 install path is not a directory!".to_string());
        }

        if self.zip_url.clone().is_none() {
            self.resolve().await?;

            if self.zip_url.clone().is_none() {
                return Err("No valid SpaceWarp release found!".to_string());
            }
        }

        let files_in_dir = self.ksp2_install_path.read_dir().unwrap();

        let mut bepinex_installed = false;

        for file in files_in_dir {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();

            if file_name.contains("BepInEx") {
                bepinex_installed = true;
            }
        }

        if !bepinex_installed {
            let mut bepinex = BepInExInstallManager::new(self.ksp2_install_path.clone());

            bepinex
                .download(on_progress, on_finish, window.clone())
                .await?;
        }

        let download_url = self
            .zip_url
            .clone()
            .expect("No valid SpaceWarp release found!");

        println!("Downloading from URL: {}", download_url);

        let out_file = self.ksp2_install_path.join(".spacewarp_release.zip");

        Downloader::download(download_url, out_file, on_progress, on_finish, window).await;

        zip_extensions::read::zip_extract(
            &PathBuf::from(&self.ksp2_install_path.join(".spacewarp_release.zip")),
            &PathBuf::from(&self.ksp2_install_path),
        )
        .expect("Could not extract the SpaceWarp release!");

        fs::remove_file(self.ksp2_install_path.join(".spacewarp_release.zip"))
            .expect("Could not delete the SpaceWarp release file!");

        return Ok(());
    }

    pub fn uninstall(&mut self) {
        fs::remove_dir_all(self.ksp2_install_path.join("BepInEx"))
            .expect("Could not delete the BepInEx directory!");

        fs::remove_file(self.ksp2_install_path.join("winhttp.dll"))
            .expect("Could not delete the winhttp.dll file!");

        fs::remove_file(self.ksp2_install_path.join("doorstop_config.ini"))
            .expect("Could not delete the doorstop_config.ini file!");
    }
}
