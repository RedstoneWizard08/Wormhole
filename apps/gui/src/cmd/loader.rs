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
use whcore::Boolify;

use crate::AppState;

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_latest_loader(
    loader: ModLoaderType,
    _pool: AppState<'_>,
) -> Result<ModLoader, bool> {
    Ok(match loader {
        ModLoaderType::Forge => ModLoader::forge_latest().await.bool()?,
        ModLoaderType::NeoForge => ModLoader::neo_latest().await.bool()?,
        ModLoaderType::Fabric => ModLoader::fabric_latest().await.bool()?,
        ModLoaderType::Quilt => ModLoader::quilt_latest().await.bool()?,
        ModLoaderType::Vanilla => ModLoader::vanilla_latest().await.bool()?,
        ModLoaderType::None => ModLoader::None,
    })
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_loaders(
    loader: ModLoaderType,
    _pool: AppState<'_>,
) -> Result<Vec<ModLoader>, bool> {
    Ok(match loader {
        ModLoaderType::Forge => get_forge_versions()
            .await
            .bool()?
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
            .bool()?
            .0
            .iter()
            .map(|v| {
                let v = parse_neoforge_version(v);
                ModLoader::NeoForge(v.0, v.1)
            })
            .collect(),

        ModLoaderType::Fabric => get_fabric_versions()
            .await
            .bool()?
            .versioning
            .versions
            .iter()
            .map(|v| ModLoader::Fabric("".into(), v.clone()))
            .collect(),

        ModLoaderType::Quilt => get_quilt_versions()
            .await
            .bool()?
            .versioning
            .versions
            .iter()
            .map(|v| ModLoader::Quilt("".into(), v.clone()))
            .collect(),

        ModLoaderType::Vanilla => get_manifest()
            .await
            .bool()?
            .versions
            .iter()
            .map(|v| ModLoader::Vanilla(v.id.clone()))
            .collect(),

        _ => Vec::new(),
    })
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn install_loader(
    loader: ModLoader,
    instance: Instance,
    pool: AppState<'_>,
) -> Result<Instance, bool> {
    let mut instance = instance;
    let lock = PLUGINS.lock().await;
    let plugin = lock.get(&instance.game_id).bool()?;

    instance.loader = Some(serde_json::to_string(&loader).bool()?);
    plugin.install_loader(&instance, &loader).await.bool()?;

    Ok(update(instances::table)
        .filter(instances::id.eq(instance.id))
        .set(instances::loader.eq(instance.loader))
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}