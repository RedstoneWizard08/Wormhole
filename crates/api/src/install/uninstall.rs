//! The uninstall API.
//! This module really hates the installer API, on an emotional level.

use std::{fs, path::PathBuf, sync::Arc};

use anyhow::Result;
use data::{
    prisma::{r#mod, PrismaClient},
    Instance, Mod,
};

/// Uninstall a mod.
pub async fn uninstall_mod(db: Arc<PrismaClient>, item: Mod, instance: Instance) -> Result<()> {
    let paths = serde_json::from_str::<Vec<PathBuf>>(&item.installed_files)?;

    for item in paths {
        let path = instance.data_dir().join(item);

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    db.r#mod().delete(r#mod::id::equals(item.id)).exec().await?;

    Ok(())
}
