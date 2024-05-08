use std::{
    env::consts::OS,
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};

use anyhow::Result;
use flate2::read::GzDecoder;
use tar::Archive;
use zip::ZipArchive;

use crate::{get_release_url, Arch, OperatingSystem};

pub async fn install_java(dir: &PathBuf, version: i32) -> Result<PathBuf> {
    let ext = if OS == "windows" { ".exe" } else { "" };
    let bin = dir.join("bin").join(format!("java{}", ext));

    if bin.exists() {
        return Ok(bin);
    }

    let url = get_release_url(version, OperatingSystem::detect(), Arch::detect()).await?;
    let data = reqwest::get(url).await?.bytes().await?;
    let magic = &data[0..2];

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    if magic == [0x1f, 0x8b] {
        // It's a tarball
        let cursor = Cursor::new(data);
        let tar = GzDecoder::new(cursor);
        let mut archive = Archive::new(tar);

        archive
            .entries()?
            .filter_map(|e| e.ok())
            .try_for_each(|mut entry| -> Result<()> {
                let path = entry.path()?;
                let prefix = path.to_str().unwrap().split("/").nth(0).unwrap();
                let path = path.strip_prefix(prefix)?.to_owned();

                entry.unpack(dir.join(&path))?;

                Ok(())
            })?;
    } else {
        let cursor = Cursor::new(data);
        let mut zip = ZipArchive::new(cursor)?;

        for idx in 0..zip.len() {
            let mut file = zip.by_index(idx)?;

            if file.is_dir() {
                continue;
            }

            let path = file.name().to_string();
            let prefix = path.split("/").nth(0).unwrap();
            let path = path.strip_prefix(prefix).unwrap().trim_start_matches('/');
            let path = dir.join(path);
            let parent = path.parent().unwrap();
            let mut buf = Vec::new();

            file.read_to_end(&mut buf)?;

            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }

            fs::write(path, buf)?;
        }
    }

    Ok(bin)
}
