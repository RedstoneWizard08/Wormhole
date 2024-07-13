//! Database seeders for plugins

use std::sync::Arc;

use anyhow::{anyhow, Result};
use data::prisma::{game, PrismaClient};

use crate::{plugin::PluginInfo, register::PLUGINS};

/// Seed a plugin in the database
pub async fn seed_plugin(db: &Arc<PrismaClient>, info: PluginInfo) -> Result<()> {
    db.game()
        .upsert(
            game::name::equals(info.id.into()),
            game::create(info.id.into(), vec![game::id::set(info.game)]),
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

/// Seed all registered plugins in the database.
pub async fn seed_plugins(db: Arc<PrismaClient>) -> Result<()> {
    for (_, plugin) in PLUGINS.lock().await.iter() {
        let info = plugin
            .as_info()
            .await
            .ok_or(anyhow!("Plugin as no info!"))?;

        seed_plugin(&db, info).await?;
    }

    Ok(())
}
