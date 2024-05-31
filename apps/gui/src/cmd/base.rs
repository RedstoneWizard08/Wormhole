//! Base commands for the GUI.

use api::{plugin::PluginInfo, register::PLUGINS};
use data::source::SourceMapping;
use whcore::{dirs::Dirs, manager::CoreManager};

use crate::AppState;

/// Gets the list of registered plugins as [`PluginInfo`].
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_plugins(_pool: AppState<'_>) -> Result<Vec<PluginInfo>, bool> {
    let mut res = Vec::new();
    let lock = PLUGINS.lock().await;

    for plugin in lock.values() {
        res.push(plugin.as_info().await.ok_or(false)?);
    }

    Ok(res)
}

/// Gets Wormhole's base directories.
/// See: [`Dirs`]
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_dirs(_pool: AppState<'_>) -> Result<Dirs, bool> {
    Ok(CoreManager::get().dirs())
}

/// Gets the string source ID from a source (mapping) ID.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_source_id(sid: i32, _pool: AppState<'_>) -> Result<String, bool> {
    Ok(SourceMapping::from(sid).as_str().to_string())
}
