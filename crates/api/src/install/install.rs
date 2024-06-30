//! The module responsible for actually installing mods.

use std::sync::Arc;

use anyhow::Result;
use data::{
    prisma::{instance, r#mod, source, PrismaClient},
    Instance, Mod,
};
use install::{extract::extract_file, magic::detect_file_type};
use query::mod_::ModVersion;
use tokio_stream::StreamExt;
use whcore::progress::ProgressCallback;

use crate::plugin::Plugin;

/// Install a mod.
pub async fn install_mod(
    db: Arc<PrismaClient>,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    plugin: Box<&dyn Plugin>,
    callback: Option<ProgressCallback>,
) -> Result<()> {
    let loader = plugin.loader(instance.clone()).await?;
    let src = plugin.get_source(item.source_id).await.unwrap();
    let version = version.unwrap_or(src.get_latest_version(&loader, item.r#mod.clone()).await?);
    let file = version.file_name();

    let url = src
        .get_download_url(&loader, item.r#mod.clone(), Some(version.clone().id))
        .await?;

    let res = reqwest::get(url).await?;

    let total = res.content_length().unwrap_or(0);
    let mut data = Vec::new();
    let mut count = 0;
    let mut stream = res.bytes_stream();

    while let Some(Ok(chunk)) = stream.next().await {
        count += chunk.len() as u64;
        data.extend(chunk);

        if let Some(callback) = &callback {
            callback(total, count, file.clone()).await;
        }
    }

    if let Some(callback) = &callback {
        callback(total, total, file.clone()).await;
    }

    let kind = detect_file_type(&data, &file)?;
    let files = extract_file(data, file, kind, instance.clone(), plugin.fallback())?;

    let installed_files = serde_json::to_string(
        &files
            .iter()
            .map(|v| {
                v.strip_prefix(instance.data_dir())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>(),
    )?;

    let it = r#mod::create(
        item.r#mod,
        item.name,
        version.file_name(),
        version
            .size
            .clone()
            .map(|v| v.parse().unwrap())
            .unwrap_or(0),
        installed_files,
        source::id::equals(item.source_id),
        instance::id::equals(instance.id),
        vec![r#mod::hash::set(version.hash.clone())],
    );

    db.r#mod().upsert(
        r#mod::file::equals(version.file_name()),
        it.clone(),
        it.to_params(),
    );

    Ok(())
}
