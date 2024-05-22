use std::io::Cursor;

use anyhow::Result;
use zip::ZipArchive;

pub const ZIP_MAGIC: [u8; 2] = [0x50, 0x4B];
pub const RAR_MAGIC: [u8; 6] = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07];
pub const GZIP_MAGIC: [u8; 2] = [0x1F, 0x8B];
pub const XZ_MAGIC: [u8; 6] = [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00];
pub const TAR_MAGIC: [u8; 5] = [0x75, 0x73, 0x74, 0x61, 0x72];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FileType {
    Zip,
    Rar,
    Gzip,
    Tar,
    Xz,
    Jar,
    ResourcePack,

    #[default]
    Unknown,
}

pub fn detect_file_type(file: impl AsRef<[u8]>, name: impl AsRef<str>) -> Result<FileType> {
    let file = file.as_ref();

    if check_jar_file(file, name) {
        Ok(FileType::Jar)
    } else if check_resourcepack_file(file)? {
        Ok(FileType::ResourcePack)
    } else if file[0..2] == ZIP_MAGIC {
        Ok(FileType::Zip)
    } else if file[0..6] == RAR_MAGIC {
        Ok(FileType::Rar)
    } else if file[0..2] == GZIP_MAGIC {
        Ok(FileType::Gzip)
    } else if file[0..6] == XZ_MAGIC {
        Ok(FileType::Xz)
    } else if file[0..5] == TAR_MAGIC {
        Ok(FileType::Tar)
    } else {
        Err(anyhow!("Unknown file type!"))
    }
}

pub fn check_jar_file(file: impl AsRef<[u8]>, name: impl AsRef<str>) -> bool {
    let file = file.as_ref();

    file[0..2] == ZIP_MAGIC && name.as_ref().to_lowercase().ends_with(".jar")
}

pub fn check_resourcepack_file(file: impl AsRef<[u8]>) -> Result<bool> {
    let file = file.as_ref();

    if file[0..2] == ZIP_MAGIC {
        let zip = ZipArchive::new(Cursor::new(file.to_vec()))?;

        let names = zip
            .file_names()
            .map(|v| v.to_lowercase())
            .collect::<Vec<_>>();

        Ok(names.contains(&"pack.mcmeta".to_string()))
    } else {
        Ok(false)
    }
}
