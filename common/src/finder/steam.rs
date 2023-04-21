use std::{fs, path::PathBuf};

use keyvalues_parser::Vdf;

#[cfg(target_os = "linux")]
use super::resolver::linux;
#[cfg(target_os = "macos")]
use super::resolver::macos;
#[cfg(target_os = "windows")]
use super::resolver::windows;

#[derive(Default, Clone, Debug)]
pub struct LibraryFolders {
    pub paths: Vec<PathBuf>,
    pub discovered: bool,
}

impl LibraryFolders {
    pub fn new() -> Self {
        return Self {
            paths: vec![],
            discovered: false,
        };
    }

    #[cfg(target_os = "macos")]
    pub fn get_library_folders_file(&self) -> PathBuf {
        return macos::get_steam_library_folders_file();
    }

    #[cfg(target_os = "linux")]
    pub fn get_library_folders_file(&self) -> PathBuf {
        return linux::get_steam_library_folders_file();
    }

    #[cfg(target_os = "windows")]
    pub fn get_library_folders_file(&self) -> PathBuf {
        return windows::get_steam_library_folders_file();
    }

    pub fn discover(&mut self) -> Option<&mut Self> {
        let libraryfolders_vdf_path = self.get_library_folders_file();

        if libraryfolders_vdf_path.is_file() {
            let vdf_text = fs::read_to_string(&libraryfolders_vdf_path).ok()?;
            let value = Vdf::parse(&vdf_text).ok()?.value;
            let obj = value.get_obj()?;

            let library_folders: Vec<_> = obj
                .iter()
                .filter(|(key, values)| key.parse::<u32>().is_ok() && values.len() == 1)
                .filter_map(|(_, values)| {
                    let library_folder_string = values
                        .get(0)?
                        .get_obj()?
                        .get("path")?
                        .get(0)?
                        .get_str()?
                        .to_string();

                    let library_folder = PathBuf::from(library_folder_string).join("steamapps");

                    return Some(library_folder);
                })
                .collect();

            self.paths = library_folders;
        }

        self.discovered = true;

        return Some(self);
    }
}

pub struct SteamInstallFinder {
    pub library_folders: LibraryFolders,
}

impl Default for SteamInstallFinder {
    fn default() -> Self {
        let mut me = Self {
            library_folders: LibraryFolders::new(),
        };

        me.library_folders
            .discover()
            .expect("Failed to discover Steam library folders!");

        return me;
    }
}

impl SteamInstallFinder {
    pub fn find_ksp2_dir(&mut self) -> Option<PathBuf> {
        for library_folder in self.library_folders.clone().paths {
            let ksp2_dir = library_folder.join("common").join("Kerbal Space Program 2");

            if ksp2_dir.is_dir() {
                let dir_contents = fs::read_dir(&ksp2_dir).unwrap().flatten();

                for file in dir_contents {
                    let file_path = file.path();

                    if file_path.is_file()
                        && file_path
                            .as_os_str()
                            .to_str()
                            .unwrap()
                            .contains("KSP2_x64.exe")
                    {
                        return Some(ksp2_dir);
                    }
                }
            }
        }

        return None;
    }

    pub fn find_ksp1_dir(&mut self) -> Option<PathBuf> {
        for library_folder in self.library_folders.clone().paths {
            let ksp_dir = library_folder.join("common").join("Kerbal Space Program");

            if ksp_dir.is_dir() {
                let dir_contents = fs::read_dir(&ksp_dir).unwrap().flatten();

                for file in dir_contents {
                    let file_path = file.path();

                    if file_path.is_file()
                        && file_path
                            .as_os_str()
                            .to_str()
                            .unwrap()
                            .contains("KSP_x64.exe")
                    {
                        return Some(ksp_dir);
                    }
                }
            }
        }

        return None;
    }
}
