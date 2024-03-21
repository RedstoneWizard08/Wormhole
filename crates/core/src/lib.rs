pub mod messaging;

use dirs::data_local_dir;
use std::path::PathBuf;

pub const WORMHOLE_DIR_NAME: &str = "Wormhole";

pub fn get_wormhole_dir() -> PathBuf {
    data_local_dir().unwrap().join(WORMHOLE_DIR_NAME)
}

pub fn get_data_dir() -> PathBuf {
    get_wormhole_dir().join("data")
}

pub fn get_cache_dir() -> PathBuf {
    get_wormhole_dir().join("cache")
}
