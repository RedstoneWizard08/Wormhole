use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Mods {
    mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Mod {
    name: String,
    date_installed: String,
    size: u64,
    install_path: String,
}

pub fn check_directories() {
    let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir_path.push("Wormhole");

    if !dir_path.exists() {
        let _ = fs::create_dir_all(&dir_path);
    }

    if !&dir_path.join("instances").exists() {
        let _ = fs::create_dir_all(dir_path.join("instances"));
    }

    if !&dir_path.join("mods").exists() {
        let _ = fs::create_dir_all(dir_path.join("mods"));
    }

    if !&dir_path.join("cache").exists() {
        let _ = fs::create_dir_all(dir_path.join("cache"));
    }
}

pub fn check_files() {
    let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir_path.push("Wormhole");

    if !&dir_path.join("instances").join("instances.json").exists() {
        let _ = fs::write(dir_path.join("instances").join("instances.json"), "[]");
    }

    const MODS_TEMPLATE: &str = r#"
    {
      "mods": [
        {
          "name": "Mod 1",
          "date_installed": "2022-03-10T10:30:00Z",
          "size": 1000000,
          "install_path": "C:\\Users\\username\\AppData\\Roaming\\Wormhole\\mods\\mod1"
        },
        {
          "name": "Mod 2",
          "date_installed": "2022-03-09T15:20:00Z",
          "size": 500000,
          "install_path": "C:\\Users\\username\\AppData\\Roaming\\Wormhole\\mods\\mod2"
        },
        {
          "name": "Mod 3",
          "date_installed": "2022-03-08T11:45:00Z",
          "size": 2000000,
          "install_path": "C:\\Users\\username\\AppData\\Roaming\\Wormhole\\mods\\mod3"
        }
      ]
    }
    "#;

    if !&dir_path.join("mods").join("mods.json").exists() {
        let _ = fs::write(dir_path.join("mods").join("mods.json"), MODS_TEMPLATE);
    }
}

pub fn directory_integrity_check() {
    check_directories();
    check_files();
}

pub fn read_mods_file() -> Mods {
    let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir_path.push("Wormhole");
    dir_path.push("mods");
    dir_path.push("mods.json");
    let mods_file_contents = fs::read_to_string(dir_path);
    let mods: Mods = serde_json::from_str(&mods_file_contents.unwrap()).unwrap();
    return mods;
}

// write to mods file
pub fn write_mods_file(mods: Mods) {
    let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir_path.push("Wormhole");
    dir_path.push("mods");
    dir_path.push("mods.json");
    let _mods_file_contents = fs::write(dir_path, serde_json::to_string(&mods).unwrap());
}
