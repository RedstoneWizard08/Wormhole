use anyhow::Result;
use data::{conv::DbIntoArg, instance::Instance, Conn};
use install::{extract::extract_file, magic::detect_file_type};
use query::mod_::{Mod, ModVersion};
use tokio_stream::StreamExt;
use whcore::progress::ProgressCallback;

use crate::plugin::Plugin;

pub async fn install_mod(
    db: &mut Conn,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    plugin: Box<&dyn Plugin>,
    callback: Option<ProgressCallback>,
) -> Result<()> {
    let src = plugin.get_source(item.source).await.unwrap();
    let version = version.unwrap_or(src.get_latest_version(item.id.clone(), &instance).await?);
    let file = version.file_name();

    let url = src
        .get_download_url(item.id.clone(), &instance, Some(version.clone().id))
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

    // TODO: Get the path the mod was installed to and provide it

    item.db_into_arg(db, (version, instance, files))?;

    Ok(())
}
