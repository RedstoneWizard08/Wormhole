// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::Result;
use keyvalues_parser::Vdf;
use std::{collections::HashMap, fs, path::PathBuf};

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
        Self {
            paths: vec![],
            discovered: false,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn get_library_folders_file(&self) -> PathBuf {
        macos::get_steam_library_folders_file()
    }

    #[cfg(target_os = "linux")]
    pub fn get_library_folders_file(&self) -> PathBuf {
        linux::get_steam_library_folders_file()
    }

    #[cfg(target_os = "windows")]
    pub fn get_library_folders_file(&self) -> PathBuf {
        windows::get_steam_library_folders_file()
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
                        .first()?
                        .get_obj()?
                        .get("path")?
                        .first()?
                        .get_str()?
                        .to_string();

                    let library_folder = PathBuf::from(library_folder_string).join("steamapps");

                    Some(library_folder)
                })
                .collect();

            self.paths = library_folders;
        }

        self.discovered = true;

        Some(self)
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

        me
    }
}

impl SteamInstallFinder {
    pub fn discover_games(&self) -> Result<HashMap<String, PathBuf>> {
        let mut games = HashMap::new();

        for folder in self.library_folders.clone().paths {
            let items = fs::read_dir(folder.join("common"))?;

            for item in items.flatten() {
                if item.path().is_dir() {
                    games.insert(item.file_name().into_string().unwrap(), item.path());
                }
            }
        }

        Ok(games)
    }
}
