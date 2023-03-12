use std::path::PathBuf;
use std::{fs};

pub fn check_directories() {
    let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir_path.push("Wormhole");

    if !dir_path.exists() {
        let _ = fs::create_dir_all(&dir_path);
    }

    if !&dir_path.join("Instances").exists() {
        let _ = fs::create_dir_all(&dir_path.join("Instances"));
    }

    if !&dir_path.join("Mods").exists() {
        let _ = fs::create_dir_all(&dir_path.join("Mods"));
    }
}

pub fn check_files() {
    let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir_path.push("Wormhole");

    if !&dir_path.join("Instances").join("instances.json").exists() {
        let _ = fs::write(&dir_path.join("Instances").join("instances.json"), "[]");
    }

    if !&dir_path.join("Mods").join("mods.json").exists() {
        let _ = fs::write(&dir_path.join("Mods").join("mods.json"), "[]");
    }
}