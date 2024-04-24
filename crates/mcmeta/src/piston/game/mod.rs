pub mod args;
pub mod assets;
pub mod cond;
pub mod download;
pub mod inherit;
pub mod java;
pub mod library;
pub mod logging;
pub mod manifest;
pub mod platform;
pub mod rules;

use anyhow::Result;

use self::manifest::GameManifest;

pub async fn get_game_manifest(url: String) -> Result<GameManifest> {
    Ok(reqwest::get(url).await?.json().await?)
}
