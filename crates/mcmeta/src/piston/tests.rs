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
