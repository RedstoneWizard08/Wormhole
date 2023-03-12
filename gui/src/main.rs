#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_return)]

use std::{path::PathBuf, process::Command};

use tauri::Window;

use installer::bepinex::BepInExInstallManager;

use wormhole_common::{
    finder::find_install_dir,
    instances::InstanceInfo,
    mods::{
        schema::browse::{BrowseResult, ModInfo},
        spacedock::SpaceDockAPI,
    },
    installer::mods::ModInstaller,
};
use wormhole_common::boot::integrity::{directory_integrity_check, Mods, read_mods_file};

pub mod installer;
pub mod progress;

#[tauri::command]
fn get_install_dir() -> PathBuf {
    return find_install_dir();
}

#[tauri::command]
fn get_install_type() -> bool {
    let dir = find_install_dir();
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
async fn download_bepinex(window: Window) -> String {
    let mut install_manager = BepInExInstallManager::new(get_install_dir());

    install_manager.resolve().await.unwrap();
    install_manager.download(window).await.unwrap();

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
    let dir = find_install_dir();
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
    return InstanceInfo::defaults()
        .iter()
        .find(|i| i.id == instance_id)
        .cloned();
}

#[tauri::command]
async fn get_mod(mod_id: i32) -> ModInfo {
    return SpaceDockAPI::new().get_mod(mod_id).await;
}

#[tauri::command]
async fn get_mods(game_id: i32, count: i32, page: i32) -> BrowseResult {
    return SpaceDockAPI::new()
        .get_mods_for_game(game_id, page, count)
        .await;
}

#[tauri::command]
async fn install_mod(mod_id: i32) {
    let installer = ModInstaller::new(find_install_dir());

    installer.install_from_spacedock(mod_id).await;
}

#[allow(clippy::needless_range_loop)]
async fn levenshtein_distance(a: &str, b: &str) -> usize {
    let m = a.chars().count();
    let n = b.chars().count();
    let mut matrix = vec![vec![0; n + 1]; m + 1];

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    for i in 0..=m {
        matrix[i][0] = i;
    }
    for j in 0..=n {
        matrix[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[m][n]
}

#[tauri::command]
async fn get_distance(mod_name: &str, query: &str) -> Result<usize, String> {
    Ok(levenshtein_distance(query, mod_name).await)
}

#[tauri::command]
async fn backend_boot() {
    directory_integrity_check();
}

#[tauri::command]
fn read_mod_json() -> Mods {
    return read_mods_file();
}

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_bepinex,
            uninstall_bepinex,
            get_install_dir,
            get_install_type,
            launch,
            get_instance_info,
            get_instances,
            get_mod,
            get_mods,
            install_mod,
            get_distance,
            backend_boot,
            read_mod_json
        ])
        .run(tauri::generate_context!())
        .expect("Error while starting Wormhole!");
}
// BIE KSP2 = 3255
