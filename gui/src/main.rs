#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_return)]

use std::{path::PathBuf, process::Command};
use tauri::Window;

use wormhole_common::boot::cache::update_cache;
use wormhole_common::boot::integrity::{directory_integrity_check, read_mods_file, Mods};
use wormhole_common::installer::mods::ModInstaller;
use wormhole_common::instances::KSPGame;

use wormhole_common::{
    finder::find_install_dir,
    installer::spacedock::SpaceDockModInstaller,
    instances::Instance,
    mods::{
        schema::browse::{BrowseResult, ModInfo},
        spacedock::SpaceDockAPI,
    },
};

pub mod installer;

#[tauri::command]
fn get_install_dir(game_id: i32) -> PathBuf {
    return find_install_dir(KSPGame::from_id(game_id).unwrap());
}

#[tauri::command]
async fn install_spacewarp(window: Window) -> String {
    installer::install_spacewarp(window).await;

    return "Success".to_string();
}

#[tauri::command]
async fn uninstall_spacewarp() -> String {
    installer::uninstall_spacewarp();

    return "Success".to_string();
}

#[tauri::command]
async fn launch(instance_id: i32) {
    println!("[Pre-launch] Launching instance: {:?}", instance_id);

    let instance = Instance::from_id(instance_id).unwrap();

    let executable = match instance.game {
        KSPGame::KSP2 => instance.install_path.join("KSP2_x64.exe"),
        KSPGame::KSP1 => instance.install_path.join("KSP_x64.exe"),
    };

    if let Some(active) = Instance::get_active_instance(instance.clone().game) {
        if active.id != instance.id {
            instance.enable();
        }
    }

    println!("[Launch] Launching game executable: {:?}", executable);

    Command::new(executable)
        .spawn()
        .expect("Failed to launch the instance!");
}

#[tauri::command]
async fn get_instances() -> Vec<Instance> {
    return Instance::load();
}

#[tauri::command]
async fn get_instance_info(instance_id: i32) -> Option<Instance> {
    let it = Instance::from_id(instance_id);

    if let Some(info) = it {
        let mut infos = info;

        infos.install_path = get_install_dir(infos.game.as_i32());

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
async fn install_mod(mod_id: i32, instance_id: i32) {
    let instance = Instance::from_id(instance_id).unwrap();
    let installer = SpaceDockModInstaller::new(instance.install_path);

    installer.install(mod_id, instance_id).await;
}

#[tauri::command]
async fn update_description(instance_id: i32, description: String) {
    let instances = Instance::load();
    let mut infos = get_instance_info(instance_id).await.unwrap();

    infos.description = Some(description);

    let mut new_instances = Vec::new();

    for instance in instances {
        if instance.id == instance_id {
            new_instances.push(infos.clone());
        } else {
            new_instances.push(instance);
        }
    }

    Instance::save_all(&new_instances);
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

#[tauri::command]
fn get_active_instance(game_id: i32) -> Option<Instance> {
    let instance = Instance::get_active_instance(KSPGame::from_id(game_id).unwrap());

    return instance;
}

#[tauri::command]
fn set_active_instance(instance_id: i32) {
    let instance = Instance::from_id(instance_id);

    if let Some(instance) = instance {
        instance.enable();
    }
}

#[tauri::command]
fn add_instance(game_id: i32, name: String, install_path: String) {
    let id = Instance::new_id();

    let instance = Instance {
        id,
        name,
        game: KSPGame::from_id(game_id).unwrap(),
        description: None,
        mods: Vec::new(),
        install_path: PathBuf::from(install_path),
        time_played: None,
    };

    instance.save();
}

#[tauri::command]
fn delete_instance(instance_id: i32) {
    let instance = Instance::from_id(instance_id);

    if let Some(instance) = instance {
        instance.remove();
    }
}

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_spacewarp,
            uninstall_spacewarp,
            get_install_dir,
            launch,
            get_instance_info,
            get_instances,
            get_mod,
            get_mods,
            install_mod,
            get_distance,
            backend_boot,
            read_mod_json,
            update_description,
            get_active_instance,
            set_active_instance,
            add_instance,
            delete_instance
        ])
        .run(tauri::generate_context!())
        .expect("Error while starting Wormhole!");
}
// BIE KSP2 = 3255
