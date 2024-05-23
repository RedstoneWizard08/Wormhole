use std::ops::DerefMut;

use data::{
    diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper},
    instance::Instance,
    mod_::DbMod,
    schema::mods,
};
use query::mod_::{Mod, ModVersion};
use whcore::Boolify;

use crate::AppState;

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn install_mod(
    game_id: i32,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    pool: AppState<'_>,
) -> Result<(), bool> {
    use whcore::Boolify;

    let it = api::register::PLUGINS.lock().await;
    let plugin = it.get(&game_id).bool()?;

    plugin
        .install(pool.get().bool()?.deref_mut(), item, version, instance)
        .await
        .ok_or(false)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn uninstall_mod(
    game_id: i32,
    item: Mod,
    instance: Instance,
    pool: AppState<'_>,
) -> Result<(), bool> {
    use whcore::Boolify;

    let it = api::register::PLUGINS.lock().await;
    let plugin = it.get(&game_id).bool()?;

    plugin
        .uninstall(pool.get().bool()?.deref_mut(), item, instance)
        .await
        .ok_or(false)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_mods(instance_id: i32, pool: AppState<'_>) -> Result<Vec<DbMod>, bool> {
    Ok(mods::table
        .select(DbMod::as_select())
        .filter(mods::instance_id.eq(instance_id))
        .load(&mut pool.get().bool()?)
        .bool()?)
}
