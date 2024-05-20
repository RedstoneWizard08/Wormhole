// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub mod intermediary;

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

pub const QUILT_MAVEN: &str = "https://maven.quiltmc.org/repository/release";

pub async fn get_quilt_versions() -> Result<MavenMetadata> {
    Ok(get_metadata(QUILT_MAVEN, "org.quiltmc:quilt-loader").await?)
}

pub fn get_quilt_loader(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("org.quiltmc:quilt-loader:{}", ver),
        repo: QUILT_MAVEN.into(),
    }
}

pub fn get_quilt_launchwrapper_artifact(version: impl AsRef<str>) -> MavenArtifact {
    let ver = version.as_ref();

    MavenArtifact {
        name: format!("org.quiltmc:quilt-loader:{}@json", ver),
        repo: QUILT_MAVEN.into(),
    }
}

pub async fn get_quilt_launchwrapper(version: impl AsRef<str>) -> Result<LaunchWrapperConfig> {
    let ver = version.as_ref();
    let coord = MavenCoordinate::from(format!("org.quiltmc:quilt-loader:{}@json", ver));
    let url = coord.url(QUILT_MAVEN);

    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_quilt_profile(
    mc_version: impl AsRef<str>,
    quilt_version: impl AsRef<str>,
) -> Result<InheritedGameManifest> {
    Ok(reqwest::get(format!(
        "https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json",
        mc_version.as_ref(),
        quilt_version.as_ref()
    ))
    .await?
    .json()
    .await?)
}
