// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::path::PathBuf;

use whcore::manager::CoreManager;

#[derive(Debug, Clone)]
pub struct MinecraftDirs {
    pub libs: PathBuf,
    pub natives: PathBuf,
    pub temp: PathBuf,
    pub java: PathBuf,
    pub assets_base: PathBuf,
}

impl MinecraftDirs {
    pub fn collect() -> Self {
        Self {
            libs: CoreManager::get()
                .game_data_dir("minecraft")
                .join("libraries"),
            natives: CoreManager::get()
                .game_data_dir("minecraft")
                .join("natives"),
            temp: CoreManager::get().game_data_dir("minecraft").join("temp"),
            java: CoreManager::get().game_data_dir("java"),
            assets_base: CoreManager::get().game_data_dir("minecraft").join("assets"),
        }
    }

    pub fn assets(&self, version: impl AsRef<str>) -> PathBuf {
        self.assets_base.join(version.as_ref())
    }
}

impl Default for MinecraftDirs {
    fn default() -> Self {
        Self::collect()
    }
}
