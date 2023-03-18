#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_return)]

use std::{path::PathBuf, process::Command};

use tauri::Window;

use installer::bepinex::BepInExInstallManager;

use wormhole_common::boot::cache::update_cache;
use wormhole_common::boot::integrity::{directory_integrity_check, read_mods_file, Mods};
use wormhole_common::instances::KSPGame;
use wormhole_common::{
    finder::find_install_dir,
    installer::mods::ModInstaller,
    instances::InstanceInfo,
    mods::{
        schema::browse::{BrowseResult, ModInfo},
        spacedock::SpaceDockAPI,
    },
};

pub mod installer;
pub mod progress;

#[tauri::command]
fn get_install_dir(game_id: i32) -> PathBuf {
    return find_install_dir(KSPGame::from_id(game_id).unwrap());
}

#[tauri::command]
async fn download_bepinex(window: Window, game_id: i32) -> String {
    let mut install_manager = BepInExInstallManager::new(get_install_dir(game_id));

    install_manager.resolve().await.unwrap();
    install_manager.download(window).await.unwrap();

    return "Success".to_string();
}

#[tauri::command]
async fn uninstall_bepinex(game_id: i32) -> String {
    let mut install_manager = BepInExInstallManager::new(get_install_dir(game_id));

    install_manager.uninstall();

    return "Success".to_string();
}

#[tauri::command]
async fn launch(game_id: i32) {
    println!("Launching game: {:?}", game_id);
    
    let game = KSPGame::from_id(game_id).unwrap();
    let dir = find_install_dir(game.clone());

    let executable = match game {
        KSPGame::KSP2 => dir.join("KSP2_x64.exe"),
        KSPGame::KSP1 => dir.join("KSP_x64.exe"),
    };

    println!("Launching game: {:?}", executable);

    Command::new(executable)
        .spawn()
        .expect("Failed to launch the game!");
}

#[tauri::command]
async fn get_instances() -> Vec<InstanceInfo> {
    return InstanceInfo::defaults();
}

#[tauri::command]
async fn get_instance_info(instance_id: i32) -> Option<InstanceInfo> {
    let it = InstanceInfo::defaults()
        .iter()
        .find(|i| i.id == instance_id)
        .cloned();

    if let Some(info) = it {
        let mut infos = info;

        infos.install_path = get_install_dir(infos.game.as_i32()).to_str().unwrap().to_string();

        return Some(infos);
    }

    return it;
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
async fn install_mod(mod_id: i32, game_id: i32) {
    let installer = ModInstaller::new(find_install_dir(KSPGame::from_id(game_id).unwrap()));

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
    update_cache();
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
