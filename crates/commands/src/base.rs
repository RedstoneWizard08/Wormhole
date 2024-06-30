//! Base commands for the GUI.

use std::sync::Arc;

use anyhow::{anyhow, Result};
use api::{plugin::PluginInfo, register::PLUGINS};
use data::prisma::{source, PrismaClient};
use whcore::{dirs::Dirs, manager::CoreManager, traits::Resultify};

/// Gets the list of registered plugins as [`PluginInfo`].
pub async fn get_plugins() -> Result<Vec<PluginInfo>> {
    let mut res = Vec::new();
    let lock = PLUGINS.lock().await;

    for plugin in lock.values() {
        res.push(plugin.as_info().await.resultify()?);
    }

    Ok(res)
}

/// Gets Wormhole's base directories.
/// See: [`Dirs`]
pub async fn get_dirs() -> Result<Dirs> {
    Ok(CoreManager::get().dirs())
}

/// Gets the string source ID from a source (mapping) ID.
pub async fn get_source_id(sid: i32, db: Arc<PrismaClient>) -> Result<String> {
    Ok(db
        .source()
        .find_first(vec![source::id::equals(sid)])
        .exec()
        .await?
        .ok_or(anyhow!("Cannot find that source!"))?
        .name)
}
