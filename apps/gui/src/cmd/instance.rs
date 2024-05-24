use api::register::PLUGINS;
use chrono::{DateTime, Utc};
use data::{
    diesel::{
        delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
    },
    instance::Instance,
    schema::instances,
};
use mcmeta::cmd::modded::GetLoader;
use whcore::Boolify;

use crate::AppState;

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_instances(game_id: i32, pool: AppState<'_>) -> Result<Vec<Instance>, bool> {
    let mut db = pool.get().bool()?;

    let items = instances::table
        .select(Instance::as_select())
        .filter(instances::game_id.eq(game_id))
        .load(&mut db)
        .bool()?;

    Ok(items)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn delete_instance(instance_id: i32, pool: AppState<'_>) -> Result<(), bool> {
    let mut db = pool.get().bool()?;

    delete(instances::table.filter(instances::id.eq(instance_id)))
        .execute(&mut db)
        .bool()?;

    Ok(())
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn create_instance(
    name: String,
    game_id: i32,
    pool: AppState<'_>,
) -> Result<Instance, bool> {
    let lock = PLUGINS.lock().await;
    let plugin = lock.get(&game_id).bool()?;
    let dirs = plugin.dirs();
    let now = DateTime::<Utc>::default().timestamp_millis();

    info!("Creating instance...");

    let mut it = insert_into(instances::table)
        .values(Instance {
            id: None,
            cache_dir: dirs.cache.to_str().unwrap().to_string(),
            game_id,
            created: now,
            updated: now,
            data_dir: dirs
                .data
                .join("instances")
                .join(&name)
                .to_str()
                .unwrap()
                .to_string(),
            install_dir: plugin.find().bool()?.to_str().unwrap().to_string(),
            description: String::new(),
            name,
            loader: None,
        })
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?;

    info!("Installing loader...");

    it.loader = Some(serde_json::to_string(&it.loader().await.bool()?).bool()?);

    info!("Updating database...");

    let it = update(instances::table)
        .filter(instances::id.eq(it.id))
        .set(instances::loader.eq(it.loader))
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?;

    info!("Installing instance...");

    plugin.install_instance(&it).await.bool()?;

    Ok(it)
}

// Realistically, we won't be updating anything but the description for now.
// If this changes in the future, I'll expand this.
#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn update_instance(
    instance_id: i32,
    desc: String,
    pool: AppState<'_>,
) -> Result<Instance, bool> {
    Ok(update(instances::table)
        .filter(instances::id.eq(instance_id))
        .set(instances::description.eq(desc))
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn add_instance(instance: Instance, pool: AppState<'_>) -> Result<Instance, bool> {
    let mut it = insert_into(instances::table)
        .values(instance)
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?;

    it.loader = Some(serde_json::to_string(&it.loader().await.bool()?).bool()?);

    let it = update(instances::table)
        .filter(instances::id.eq(it.id))
        .set(instances::loader.eq(it.loader))
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?;

    PLUGINS
        .lock()
        .await
        .get(&it.game_id)
        .bool()?
        .install_instance(&it)
        .await
        .bool()?;

    Ok(it)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
pub async fn get_instance(instance_id: i32, pool: AppState<'_>) -> Result<Instance, bool> {
    Ok(instances::table
        .select(Instance::as_select())
        .filter(instances::id.eq(instance_id))
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}
