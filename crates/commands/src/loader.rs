//! Loader commands for the GUI.

use api::register::PLUGINS;
use data::{
    diesel::{update, ExpressionMethods, RunQueryDsl, SelectableHelper},
    instance::Instance,
    schema::instances,
};
use mcmeta::{
    cmd::modded::{ModLoader, ModLoaderType},
    fabric::get_fabric_versions,
    forge::{get_forge_versions, parse_forge_version},
    neoforge::{get_neoforge_versions, parse_neoforge_version},
    piston::manifest::get_manifest,
    quilt::get_quilt_versions,
};
use whcore::Stringify;

use crate::{AppState, Result};

/// Get a loader's latest version.
///
/// Arguments:
/// - `loader` - The loader type.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_latest_loader(loader: ModLoaderType, _pool: AppState<'_>) -> Result<ModLoader> {
    Ok(match loader {
        ModLoaderType::Forge => ModLoader::forge_latest().await.stringify()?,
        ModLoaderType::NeoForge => ModLoader::neo_latest().await.stringify()?,
        ModLoaderType::Fabric => ModLoader::fabric_latest().await.stringify()?,
        ModLoaderType::Quilt => ModLoader::quilt_latest().await.stringify()?,
        ModLoaderType::Vanilla => ModLoader::vanilla_latest().await.stringify()?,
        ModLoaderType::None => ModLoader::None,
    })
}

/// Get a loader's versions.
///
/// Arguments:
/// - `loader` - The loader type.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_loaders(loader: ModLoaderType, _pool: AppState<'_>) -> Result<Vec<ModLoader>> {
    Ok(match loader {
        ModLoaderType::Forge => get_forge_versions()
            .await
            .stringify()?
            .versioning
            .versions
            .iter()
            .map(|v| {
                let v = parse_forge_version(v);
                ModLoader::Forge(v.0, v.1)
            })
            .collect(),

        ModLoaderType::NeoForge => get_neoforge_versions()
            .await
            .stringify()?
            .0
            .iter()
            .map(|v| {
                let v = parse_neoforge_version(v);
                ModLoader::NeoForge(v.0, v.1)
            })
            .collect(),

        ModLoaderType::Fabric => get_fabric_versions()
            .await
            .stringify()?
            .versioning
            .versions
            .iter()
            .map(|v| ModLoader::Fabric("".into(), v.clone()))
            .collect(),

        ModLoaderType::Quilt => get_quilt_versions()
            .await
            .stringify()?
            .versioning
            .versions
            .iter()
            .map(|v| ModLoader::Quilt("".into(), v.clone()))
            .collect(),

        ModLoaderType::Vanilla => get_manifest()
            .await
            .stringify()?
            .versions
            .iter()
            .map(|v| ModLoader::Vanilla(v.id.clone()))
            .collect(),

        _ => Vec::new(),
    })
}

/// Installs a loader to an instance.
///
/// Arguments:
/// - `loader` - The loader to install.
/// - `instance` - The instance to install the loader to.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn install_loader(
    loader: ModLoader,
    instance: Instance,
    pool: AppState<'_>,
) -> Result<Instance> {
    let mut instance = instance;
    let lock = PLUGINS.lock().await;
    let plugin = lock.get(&instance.game_id).stringify()?;

    instance.loader = Some(serde_json::to_string(&loader).stringify()?);
    plugin
        .install_loader(&instance, &loader)
        .await
        .stringify()?;

    Ok(update(instances::table)
        .filter(instances::id.eq(instance.id))
        .set(instances::loader.eq(instance.loader))
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().stringify()?)
        .stringify()?)
}
