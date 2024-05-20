// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::path::PathBuf;

use crate::{
    piston::{
        assets::get_asset_index,
        download::{download_assets, download_libs},
        game::{get_game_manifest, inherit::InheritedGameManifest},
        manifest::get_manifest,
    },
    test_util::{asset_download_callback, library_download_callback},
};
use anyhow::Result;

#[tokio::test]
#[serial_test::serial]
pub async fn test_parse_versions() -> Result<()> {
    let all = get_manifest().await?;

    let _ = get_game_manifest(all.find("1.20.4").unwrap().url).await?;
    let _ = get_game_manifest(all.find("rd-132211").unwrap().url).await?;

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
pub async fn test_lib_dl() -> Result<()> {
    let all = get_manifest().await?;
    let root = PathBuf::from("./test/libraries");

    download_libs(
        &root,
        &get_game_manifest(all.find("1.20.4").unwrap().url).await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    download_libs(
        root,
        &get_game_manifest(all.find("rd-132211").unwrap().url).await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
pub async fn test_assets_dl() -> Result<()> {
    let all = get_manifest().await?;
    let root = PathBuf::from("./test/assets");

    download_assets(
        &root,
        get_asset_index(
            get_game_manifest(all.find("1.20.4").unwrap().url)
                .await?
                .asset_index
                .url,
        )
        .await?,
        &Some(Box::new(asset_download_callback)),
    )
    .await?;

    download_assets(
        root,
        get_asset_index(
            get_game_manifest(all.find("rd-132211").unwrap().url)
                .await?
                .asset_index
                .url,
        )
        .await?,
        &Some(Box::new(asset_download_callback)),
    )
    .await?;

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
pub async fn test_inherited_manifest() -> Result<()> {
    let lib_root = PathBuf::from("./test/libraries");

    download_libs(
        &lib_root,
        &serde_json::from_str::<InheritedGameManifest>(include_str!(
            "./test_data/inherited_manifest.json"
        ))?
        .resolve()
        .await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    download_libs(
        &lib_root,
        &serde_json::from_str::<InheritedGameManifest>(include_str!(
            "./test_data/inherited_manifest_neo.json"
        ))?
        .resolve()
        .await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    download_libs(
        &lib_root,
        &serde_json::from_str::<InheritedGameManifest>(include_str!(
            "./test_data/inherited_manifest_neo_new.json"
        ))?
        .resolve()
        .await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    Ok(())
}
