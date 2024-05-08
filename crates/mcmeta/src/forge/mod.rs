pub mod extract;
pub mod install;
pub mod mappings;
pub mod processor;
pub mod util;

#[cfg(test)]
pub mod tests;

use std::io::{Cursor, Read};

use anyhow::Result;
use zip::ZipArchive;

use crate::{
    maven::{artifact::MavenArtifact, get_metadata, metadata::MavenMetadata},
    piston::game::inherit::InheritedGameManifest,
};

pub const FORGE_MAVEN: &str = "https://maven.minecraftforge.net";

/// Takes in a Forge version string and returns
/// a tuple of the Minecraft and loader versions.
pub fn parse_forge_version(ver: impl AsRef<str>) -> (String, String) {
    let mut ver = ver.as_ref().split('-');
    let mc = ver.next().unwrap().to_string();
    let forge = ver.next().unwrap().to_string();

    (mc, forge)
}

pub async fn get_forge_versions() -> Result<MavenMetadata> {
    Ok(get_metadata(FORGE_MAVEN, "net.minecraftforge:forge").await?)
}

pub fn get_forge_installer(version: impl AsRef<str>) -> MavenArtifact {
    MavenArtifact {
        name: format!("net.minecraftforge:forge:{}:installer", version.as_ref()),
        repo: FORGE_MAVEN.into(),
    }
}

pub async fn get_forge_manifest(version: impl AsRef<str>) -> Result<InheritedGameManifest> {
    let url = get_forge_installer(version).coordinate().url(FORGE_MAVEN);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name("version.json")?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;

    Ok(serde_json::from_str(&data)?)
}
