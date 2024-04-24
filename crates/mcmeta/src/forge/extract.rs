use std::{io::Cursor, path::PathBuf};

use anyhow::Result;
use zip::ZipArchive;

use super::{get_forge_installer, FORGE_MAVEN};

pub async fn extract_forge_installer(tmp_dir: &PathBuf, version: impl AsRef<str>) -> Result<()> {
    let url = get_forge_installer(version).coordinate().url(FORGE_MAVEN);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;

    zip.extract(tmp_dir)?;

    Ok(())
}
