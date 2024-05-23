use api::{plugin::PluginInfo, register::PLUGINS};
use data::source::SourceMapping;
use whcore::{dirs::Dirs, manager::CoreManager};

use crate::AppState;

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

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_dirs(_pool: AppState<'_>) -> Result<Dirs, bool> {
    Ok(CoreManager::get().dirs())
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_source_id(sid: i32, _pool: AppState<'_>) -> Result<String, bool> {
    Ok(SourceMapping::from(sid).as_str().to_string())
}
