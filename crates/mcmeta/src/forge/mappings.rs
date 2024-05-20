// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::io::{Cursor, Read};

use anyhow::Result;
use serde_json::Value;
use zip::ZipArchive;

use crate::piston::{game::get_game_manifest, manifest::get_manifest};

use super::{
    get_forge_installer, install::InstallProfile, parse_forge_version, util::get_artifact_ref,
    FORGE_MAVEN,
};

pub async fn get_forge_install_profile(version: impl AsRef<str>) -> Result<InstallProfile> {
    let url = get_forge_installer(version).coordinate().url(FORGE_MAVEN);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name("install_profile.json")?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;

    Ok(serde_json::from_str(&data)?)
}

pub async fn get_mcp_tsrg(version: impl AsRef<str>) -> Result<String> {
    let profile = get_forge_install_profile(version).await?;
    let loc = profile.data.get("MAPPINGS").unwrap().clone().client;
    let url = get_artifact_ref(loc).coordinate().url(FORGE_MAVEN);
    let bytes = reqwest::get(&url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name("config.json")?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;
    let mcp_config: Value = serde_json::from_str(&data)?;

    let path = mcp_config
        .get("data")
        .unwrap()
        .get("mappings")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // I have to do this because it thinks I'm borring
    // the archive mutably more than once at a time.
    // I'm not. (At least I don't think I am.)
    drop(file);
    drop(zip);

    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name(&path)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;

    Ok(data)
}

pub async fn get_mojmaps(version: impl AsRef<str>) -> Result<String> {
    let (mc, _forge) = parse_forge_version(version);
    let manifest = get_game_manifest(get_manifest().await?.find(mc).unwrap().url).await?;
    let url = manifest.downloads.client_mappings.unwrap().url;

    Ok(reqwest::get(url).await?.text().await?)
}
