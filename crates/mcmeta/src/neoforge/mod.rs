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
use zip::ZipArchive;

use crate::{
    maven::{artifact::MavenArtifact, get_metadata},
    piston::game::inherit::InheritedGameManifest,
};

pub const NEOFORGE_MAVEN: &str = "https://maven.neoforged.net";

pub mod extract;
pub mod install;
pub mod profile;
pub mod util;

#[cfg(test)]
pub mod tests;

/// Takes in a neoforge version and returns
/// a tuple with the Minecraft version (1st) and
/// the NeoForge verison (2nd).
pub fn parse_neoforge_version(ver: String) -> (String, String) {
    if ver.starts_with("1.") {
        (
            ver.split("-").nth(0).unwrap().to_string(),
            ver.split("-").nth(1).unwrap().to_string(),
        )
    } else if ver.starts_with("47.") {
        ("1.20.1".to_string(), ver)
    } else {
        let mut spl = ver.split(".");
        let minor = spl.next().unwrap();
        let patch = spl.next().unwrap();

        (format!("1.{}.{}", minor, patch), ver)
    }
}

pub async fn get_neoforge_versions() -> Result<(Vec<String>, String)> {
    let forge = get_metadata(NEOFORGE_MAVEN, "net.neoforged:neoforge").await?;
    let neo = get_metadata(NEOFORGE_MAVEN, "net.neoforged:neoforge").await?;
    let mut all = Vec::new();

    all.extend(forge.versioning.versions);
    all.extend(neo.versioning.versions);

    Ok((all, neo.versioning.latest))
}

pub fn get_neoforge_installer(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    if ver.starts_with("1.") {
        MavenArtifact {
            name: format!("net.neoforged:forge:{}:installer", ver),
            repo: NEOFORGE_MAVEN.into(),
        }
    } else {
        MavenArtifact {
            name: format!("net.neoforged:neoforge:{}:installer", ver),
            repo: NEOFORGE_MAVEN.into(),
        }
    }
}

pub async fn get_neoforge_manifest(version: impl AsRef<str>) -> Result<InheritedGameManifest> {
    let url = get_neoforge_installer(version)
        .coordinate()
        .url(NEOFORGE_MAVEN);

    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name("version.json")?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;

    Ok(serde_json::from_str(&data)?)
}
