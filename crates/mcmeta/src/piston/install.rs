use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::{
    download::{download_file, DownloadCallbackFn},
    maven::coord::MavenCoordinate,
};

use super::{
    assets::get_asset_index,
    download::{download_assets, download_libs, extract_natives},
    game::get_game_manifest,
    manifest::get_manifest,
};

pub async fn install_minecraft(
    lib_dir: &PathBuf,
    natives_dir: &PathBuf,
    assets_dir: &PathBuf,
    version: impl AsRef<str>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let all = get_manifest().await?;
    let manifest = get_game_manifest(all.find(version).unwrap().url).await?;
    let asset_index = get_asset_index(&manifest.asset_index.url).await?;

    if !assets_dir.join("indexes").exists() {
        fs::create_dir_all(assets_dir.join("indexes"))?;
    }

    fs::write(
        assets_dir.join(format!("indexes/{}.json", manifest.assets)),
        serde_json::to_string(&asset_index)?,
    )?;

    download_file(
        &lib_dir,
        &manifest.downloads.client.url,
        MavenCoordinate::from(format!("net.minecraft:client:{}", manifest.id)).path(),
        Some(&manifest.downloads.client.sha1),
        callback,
    )
    .await?;

    download_libs(&lib_dir, &manifest, callback).await?;
    extract_natives(natives_dir, &manifest, callback).await?;
    download_assets(&assets_dir, asset_index, callback).await?;

    Ok(())
}
