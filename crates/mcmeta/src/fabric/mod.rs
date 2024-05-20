// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

#[cfg(test)]
pub mod tests;

use anyhow::Result;

use crate::{
    launchwrapper::config::LaunchWrapperConfig,
    maven::{
        artifact::MavenArtifact, coord::MavenCoordinate, get_metadata, metadata::MavenMetadata,
    },
    piston::game::inherit::InheritedGameManifest,
};

pub const FABRIC_MAVEN: &str = "https://maven.fabricmc.net";

pub async fn get_fabric_versions() -> Result<MavenMetadata> {
    Ok(get_metadata(FABRIC_MAVEN, "net.fabricmc:fabric-loader").await?)
}

pub fn get_fabric_loader(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("net.fabricmc:fabric-loader:{}", ver),
        repo: FABRIC_MAVEN.into(),
    }
}

pub fn get_fabric_launchwrapper_artifact(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("net.fabricmc:fabric-loader:{}:launchwrapper@json", ver),
        repo: FABRIC_MAVEN.into(),
    }
}

pub async fn get_fabric_launchwrapper(version: impl AsRef<str>) -> Result<LaunchWrapperConfig> {
    let ver = version.as_ref();
    let coord = MavenCoordinate::from(format!(
        "net.fabricmc:fabric-loader:{}:launchwrapper@json",
        ver
    ));
    let url = coord.url(FABRIC_MAVEN);

    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_fabric_profile(
    mc_version: impl AsRef<str>,
    fabric_version: impl AsRef<str>,
) -> Result<InheritedGameManifest> {
    Ok(reqwest::get(format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
        mc_version.as_ref(),
        fabric_version.as_ref()
    ))
    .await?
    .json()
    .await?)
}
