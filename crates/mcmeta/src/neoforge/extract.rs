use std::{io::Cursor, path::PathBuf};

use anyhow::Result;
use zip::ZipArchive;

use super::{get_neoforge_installer, NEOFORGE_MAVEN};

pub async fn extract_neoforge_installer(tmp_dir: &PathBuf, version: impl AsRef<str>) -> Result<()> {
    let url = get_neoforge_installer(version)
        .coordinate()
        .url(NEOFORGE_MAVEN);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;

    zip.extract(tmp_dir)?;

    Ok(())
}
