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

use anyhow::Result;

use crate::download::{download_file, DownloadCallbackFn};

use super::coord::MavenCoordinate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenArtifact {
    pub name: String,

    #[serde(rename = "url")]
    pub repo: String,
}

impl MavenArtifact {
    pub fn coordinate(&self) -> MavenCoordinate {
        (&self.name).into()
    }
}

pub async fn download_maven_artifacts(
    root: impl Into<PathBuf>,
    artifacts: Vec<MavenArtifact>,
    callback: Option<DownloadCallbackFn>,
) -> Result<()> {
    let root = root.into();

    for mut artifact in artifacts {
        // Fabric's LaunchWrapper config doesn't have the proper
        // maven repo for this. It tries to use one on Fabric's
        // own maven, but it doesn't exist. Why? I don't know.
        // This is probably one of the most lazy-ass patches
        // I've had to do here, but there isn't much else I can do.
        if artifact.name == "net.minecraft:launchwrapper:1.12" {
            artifact.repo = "https://libraries.minecraft.net".into();
        }

        let coord = artifact.coordinate();

        download_file(
            &root,
            coord.url(artifact.repo),
            coord.path(),
            Option::<String>::None,
            &callback,
        )
        .await?;
    }

    Ok(())
}
