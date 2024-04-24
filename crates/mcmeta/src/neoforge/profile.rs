use std::io::{Cursor, Read};

use anyhow::Result;
use zip::ZipArchive;

use crate::forge::install::InstallProfile;

use super::{get_neoforge_installer, NEOFORGE_MAVEN};

pub async fn get_neoforge_install_profile(version: impl AsRef<str>) -> Result<InstallProfile> {
    let url = get_neoforge_installer(version)
        .coordinate()
        .url(NEOFORGE_MAVEN);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name("install_profile.json")?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;

    Ok(serde_json::from_str(&data)?)
}
