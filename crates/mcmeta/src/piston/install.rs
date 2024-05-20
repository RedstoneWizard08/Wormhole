// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
