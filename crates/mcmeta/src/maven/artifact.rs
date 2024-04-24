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
