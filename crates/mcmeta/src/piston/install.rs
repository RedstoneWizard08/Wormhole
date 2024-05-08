use std::path::PathBuf;

use anyhow::Result;

use crate::download::DownloadCallbackFn;

use super::{
    assets::get_asset_index,
    download::{download_assets, download_libs},
    game::get_game_manifest,
    manifest::get_manifest,
};

pub async fn install_minecraft(
    lib_dir: &PathBuf,
    assets_dir: &PathBuf,
    version: impl AsRef<str>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let all = get_manifest().await?;
    let manifest = get_game_manifest(all.find(version).unwrap().url).await?;
    let asset_index = get_asset_index(&manifest.asset_index.url).await?;

    download_libs(&lib_dir, manifest, callback).await?;
    download_assets(&assets_dir, asset_index, callback).await?;

    Ok(())
}
