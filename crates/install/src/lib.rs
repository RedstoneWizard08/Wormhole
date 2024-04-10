use anyhow::Result;
use api::plugin::Plugin;
use data::{conv::DbIntoArg, instance::Instance, Conn};
use extract::extract_file;
use magic::detect_file_type;
use query::mod_::{Mod, ModVersion};
use std::io::Write;
use tempfile::NamedTempFile;

pub mod extract;
pub mod magic;

#[macro_use]
extern crate anyhow;

pub async fn install_mod<T: Plugin>(
    db: &mut Conn,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    plugin: T,
) -> Result<()> {
    let src = plugin.get_source(item.source).await.unwrap();
    let version = version.unwrap_or(src.get_latest_version(item.id.clone()).await?);

    let data = reqwest::get(
        src.get_download_url(item.id.clone(), Some(version.clone().id))
            .await?,
    )
    .await?;

    let data = data.bytes().await?.to_vec();
    let kind = detect_file_type(&data)?;
    let mut file = NamedTempFile::new()?;

    let _ = file.write(&data.into_boxed_slice())?;

    extract_file(
        db,
        file.path().into(),
        kind,
        instance.clone(),
        plugin.fallback(),
    )?;
    item.db_into_arg(db, (version, instance))?;

    Ok(())
}
