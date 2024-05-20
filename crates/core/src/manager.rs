use std::{fs, path::PathBuf};

use dirs::data_local_dir;
use sysinfo::System;

use crate::dirs::Dirs;

pub const WORMHOLE_DIR_NAME: &str = "Wormhole";
pub static mut CORE_MANAGER: CoreManager = CoreManager::new();

#[derive(Debug, Clone, Copy)]
pub struct CoreManager {
    mem: u64,
}

impl CoreManager {
    pub const fn new() -> Self {
        Self { mem: 0 }
    }

    pub fn init(&self) {
        self.create_dirs();
    }

    pub fn dirs(&self) -> Dirs {
        Dirs {
            root: self.dir(),
            data: self.data_dir(),
            cache: self.cache_dir(),
            temp: self.temp_dir(),
        }
    }

    pub fn create_dirs(&self) {
        self.create_dir(self.dir());
        self.create_dir(self.data_dir());
        self.create_dir(self.cache_dir());
        self.create_dir(self.temp_dir());

        self.create_dir(self.cache_dir().join("ckandex"));
    }

    pub fn create_dir(&self, path: PathBuf) {
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
    }

    pub fn dir(&self) -> PathBuf {
        data_local_dir().unwrap().join(WORMHOLE_DIR_NAME)
    }

    pub fn data_dir(&self) -> PathBuf {
        self.dir().join("data")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.dir().join("cache")
    }

    pub fn temp_dir(&self) -> PathBuf {
        self.dir().join("temp")
    }

    pub fn game_data_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.data_dir().join(game.as_ref())
    }

    pub fn game_cache_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.cache_dir().join(game.as_ref())
    }

    pub fn game_temp_dir(&self, game: impl AsRef<str>) -> PathBuf {
        self.temp_dir().join(game.as_ref())
    }

    fn mem_(&mut self) -> u64 {
        if self.mem == 0 {
            self.mem = (System::new_all().total_memory() / 8).max(4096).min(8192);
        }

        self.mem
    }

    fn set_mem_(&mut self, mem: u64) -> u64 {
        self.mem = mem;
        self.mem
    }

    pub fn mem() -> u64 {
        unsafe { CORE_MANAGER.mem_() }
    }

    pub fn set_mem(mem: u64) -> u64 {
        unsafe { CORE_MANAGER.set_mem_(mem) }
    }

    pub fn get() -> Self {
        unsafe { CORE_MANAGER }
    }
}
