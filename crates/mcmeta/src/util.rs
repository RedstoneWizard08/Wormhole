use anyhow::Result;
use bytes::Bytes;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub fn get_file_from_zip(data: Bytes, file: impl AsRef<str>) -> Result<String> {
    let data = Cursor::new(data);
    let mut archive = ZipArchive::new(data)?;
    let mut file = archive.by_name(file.as_ref())?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}
