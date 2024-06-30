//! Loader commands for the GUI.

use std::sync::Arc;

use api::register::PLUGINS;
use data::{
    prisma::{instance, PrismaClient},
    Instance,
};
use mcmeta::{
    cmd::modded::{ModLoader, ModLoaderType},
    fabric::get_fabric_versions,
    forge::{get_forge_versions, parse_forge_version},
    neoforge::{get_neoforge_versions, parse_neoforge_version},
    piston::manifest::get_manifest,
    quilt::get_quilt_versions,
};

use anyhow::Result;
use whcore::traits::Resultify;

/// Get a loader's latest version.
///
/// Arguments:
/// - `loader` - The loader type.
pub async fn get_latest_loader(loader: ModLoaderType) -> Result<ModLoader> {
    Ok(match loader {
        ModLoaderType::Forge => ModLoader::forge_latest().await?,
        ModLoaderType::NeoForge => ModLoader::neo_latest().await?,
        ModLoaderType::Fabric => ModLoader::fabric_latest().await?,
        ModLoaderType::Quilt => ModLoader::quilt_latest().await?,
        ModLoaderType::Vanilla => ModLoader::vanilla_latest().await?,
        ModLoaderType::None => ModLoader::None,
    })
}

/// Get a loader's versions.
///
/// Arguments:
/// - `loader` - The loader type.
pub async fn get_loaders(loader: ModLoaderType) -> Result<Vec<ModLoader>> {
    Ok(match loader {
        ModLoaderType::Forge => get_forge_versions()
            .await?
            .versioning
            .versions
            .iter()
            .map(|v| {
                let v = parse_forge_version(v);
                ModLoader::Forge(v.0, v.1)
            })
            .collect(),

        ModLoaderType::NeoForge => get_neoforge_versions()
            .await?
            .0
            .iter()
            .map(|v| {
                let v = parse_neoforge_version(v);
                ModLoader::NeoForge(v.0, v.1)
            })
            .collect(),

        ModLoaderType::Fabric => get_fabric_versions()
            .await?
            .versioning
            .versions
            .iter()
            .map(|v| ModLoader::Fabric("".into(), v.clone()))
            .collect(),

        ModLoaderType::Quilt => get_quilt_versions()
            .await?
            .versioning
            .versions
            .iter()
            .map(|v| ModLoader::Quilt("".into(), v.clone()))
            .collect(),

        ModLoaderType::Vanilla => get_manifest()
            .await?
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
pub async fn install_loader(
    loader: ModLoader,
    instance: Instance,
    db: Arc<PrismaClient>,
) -> Result<Instance> {
    let mut instance = instance;
    let lock = PLUGINS.lock().await;
    let plugin = lock.get(&instance.game_id).resultify()?;

    instance.loader = Some(serde_json::to_string(&loader)?);
    plugin.install_loader(&instance, &loader).await?;

    Ok(db
        .instance()
        .update(
            instance::id::equals(instance.id),
            vec![instance::loader::set(instance.loader)],
        )
        .exec()
        .await?)
}
