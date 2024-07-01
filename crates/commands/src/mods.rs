//! Mod commands for the GUI.

use std::sync::Arc;

use data::{
    prisma::{r#mod, PrismaClient},
    Instance, Mod,
};
use query::mod_::ModVersion;
use whcore::traits::Resultify;

use anyhow::Result;

/// Install a mod.
///
/// Arguments:
/// - `game_id` - The instance's game ID.
/// - `item` - The mod to install.
/// - `version` - The specific version of the mod to install. This can be `null` or `None`.
/// - `instance` - The instance to install to.
pub async fn install_mod(
    game_id: i32,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    db: Arc<PrismaClient>,
) -> Result<()> {
    let it = api::register::PLUGINS.lock().await;
    let plugin = it.get(&game_id).resultify()?;

    plugin.install(db, item, version, instance).await
}

/// Uninstall a mod.
///
/// Arguments:
/// - `game_id` - The instance's game ID.
/// - `item` - The mod to uninstall.
/// - `instance` - The instance to uninstall from.
pub async fn uninstall_mod(
    game_id: i32,
    item: Mod,
    instance: Instance,
    db: Arc<PrismaClient>,
) -> Result<()> {
    let it = api::register::PLUGINS.lock().await;
    let plugin = it.get(&game_id).resultify()?;

    plugin.uninstall_mod(db, item, instance).await
}

/// Get a list of mods installed on an instance.
///
/// Arguments:
/// - `instance_id` - The instance's ID in the database.
pub async fn get_mods(instance_id: i32, db: Arc<PrismaClient>) -> Result<Vec<Mod>> {
    Ok(db
        .r#mod()
        .find_many(vec![r#mod::instance_id::equals(instance_id)])
        .exec()
        .await?)
}
