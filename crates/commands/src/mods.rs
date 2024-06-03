//! Mod commands for the GUI.

use std::ops::DerefMut;

use data::{
    diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper},
    instance::Instance,
    mod_::DbMod,
    schema::mods,
};
use query::mod_::{Mod, ModVersion};
use whcore::errors::Stringify;

use crate::{AppState, Result};

/// Install a mod.
///
/// Arguments:
/// - `game_id` - The instance's game ID.
/// - `item` - The mod to install.
/// - `version` - The specific version of the mod to install. This can be `null` or `None`.
/// - `instance` - The instance to install to.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn install_mod(
    game_id: i32,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    pool: AppState<'_>,
) -> Result<()> {
    let it = api::register::PLUGINS.lock().await;
    let plugin = it.get(&game_id).stringify()?;

    plugin
        .install(pool.get().stringify()?.deref_mut(), item, version, instance)
        .await
        .ok_or("Tried to unwrap a None value!".into())
}

/// Uninstall a mod.
///
/// Arguments:
/// - `game_id` - The instance's game ID.
/// - `item` - The mod to uninstall.
/// - `instance` - The instance to uninstall from.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn uninstall_mod(
    game_id: i32,
    item: DbMod,
    instance: Instance,
    pool: AppState<'_>,
) -> Result<()> {
    let it = api::register::PLUGINS.lock().await;
    let plugin = it.get(&game_id).stringify()?;

    plugin
        .uninstall(pool.get().stringify()?.deref_mut(), item, instance)
        .await
        .ok_or("Tried to unwrap a None value!".into())
}

/// Get a list of mods installed on an instance.
///
/// Arguments:
/// - `instance_id` - The instance's ID in the database.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_mods(instance_id: i32, pool: AppState<'_>) -> Result<Vec<DbMod>> {
    Ok(mods::table
        .select(DbMod::as_select())
        .filter(mods::instance_id.eq(instance_id))
        .load(&mut pool.get().stringify()?)
        .stringify()?)
}
