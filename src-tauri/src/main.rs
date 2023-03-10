#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_return)]
#![feature(async_closure)]

use installer::{bepinex::BepInExInstallManager, doorstop::DoorstopInstallManager};
use instances::InstanceInfo;
use mods::{spacedock::SpaceDockAPI, schema::browse::{BrowseResult, ModInfo}};
use std::{path::PathBuf, process::Command};

pub mod finder;
pub mod installer;
pub mod models;
pub mod releases;
pub mod mods;
pub mod instances;

#[tauri::command]
fn get_install_dir() -> PathBuf {
    return finder::find_install_dir();
}

#[tauri::command]
fn get_install_type() -> bool {
    let dir = finder::find_install_dir();
    let files = dir.read_dir().expect("Failed to read install directory!");

    let mut is_bepinex = false;

    for file in files {
        if file
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .contains("BepInEx")
        {
            is_bepinex = true;
        }
    }

    return is_bepinex;
}

#[tauri::command]
async fn download_doorstop() -> String {
    let mut install_manager = DoorstopInstallManager::new(get_install_dir());

    install_manager.resolve().await.unwrap();
    install_manager.download().await.unwrap();

    return "Success".to_string();
}

#[tauri::command]
async fn download_bepinex() -> String {
    let mut install_manager = BepInExInstallManager::new(get_install_dir());

    install_manager.resolve().await.unwrap();
    install_manager.download().await.unwrap();

    return "Success".to_string();
}

#[tauri::command]
async fn uninstall_doorstop() -> String {
    let mut install_manager = DoorstopInstallManager::new(get_install_dir());

    install_manager.uninstall();

    return "Success".to_string();
}

#[tauri::command]
async fn uninstall_bepinex() -> String {
    let mut install_manager = BepInExInstallManager::new(get_install_dir());

    install_manager.uninstall();

    return "Success".to_string();
}

#[tauri::command]
async fn launch() {
    let dir = finder::find_install_dir();
    let executable = dir.join("KSP2_x64.exe");

    Command::new(executable)
        .spawn()
        .expect("Failed to launch KSP2!");
}

#[tauri::command]
async fn get_instances() -> Vec<InstanceInfo> {
    return InstanceInfo::defaults();
}

#[tauri::command]
async fn get_instance_info(instance_id: i32) -> Option<InstanceInfo> {
    return InstanceInfo::defaults().iter().find(|i| i.id == instance_id).cloned();
}

#[tauri::command]
async fn get_mod(mod_id: i32) -> ModInfo {
    return SpaceDockAPI::new().get_mod(mod_id).await;
}

#[tauri::command]
async fn get_mods(game_id: i32, count: i32, page: i32) -> BrowseResult {
    return SpaceDockAPI::new().get_mods_for_game(game_id, page, count).await;
}

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_doorstop,
            uninstall_doorstop,
            download_bepinex,
            uninstall_bepinex,
            get_install_dir,
            get_install_type,
            launch,
            get_instance_info,
            get_instances,
            get_mod,
            get_mods
        ])
        .run(tauri::generate_context!())
        .expect("Error while starting Wormhole!");
}
// BIE KSP2 = 3255