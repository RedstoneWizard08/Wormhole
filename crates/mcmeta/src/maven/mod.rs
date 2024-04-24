use anyhow::Result;

use self::metadata::MavenMetadata;

pub mod artifact;
pub mod coord;
pub mod metadata;

pub async fn get_metadata(
    repo: impl AsRef<str>,
    artifact: impl AsRef<str>,
) -> Result<MavenMetadata> {
    let data = reqwest::get(format!(
        "{}/{}/maven-metadata.xml",
        repo.as_ref(),
        artifact.as_ref().replace(".", "/").replace(":", "/")
    ))
    .await?
    .text()
    .await?;

    Ok(quick_xml::de::from_str(&data)?)
}
