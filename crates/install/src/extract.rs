use std::{
    fs::{self, remove_dir_all, File},
    io::{Cursor, Read, Write},
    path::Path,
};

use crate::magic::{detect_file_type, FileType};
use anyhow::Result;

use data::instance::Instance;

use flate2::read::GzDecoder;
use tar::Archive;
use walkdir::WalkDir;
use whcore::random_string;
use xz::read::XzDecoder;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub fn extract_file(
    data: Vec<u8>,
    name: impl AsRef<str>,
    kind: FileType,
    instance: Instance,
    fallback: Option<&str>,
) -> Result<()> {
    let name = name.as_ref();

    match kind {
        FileType::Jar => {
            if !instance.data_dir().join("mods").exists() {
                fs::create_dir_all(instance.data_dir().join("mods"))?;
            }

            Ok(fs::write(
                instance.data_dir().join("mods").join(name),
                data,
            )?)
        }

        FileType::ResourcePack => {
            if !instance.data_dir().join("resourcepacks").exists() {
                fs::create_dir_all(instance.data_dir().join("resourcepacks"))?;
            }

            Ok(fs::write(
                instance.data_dir().join("resourcepacks").join(name),
                data,
            )?)
        }

        it => reprocess_zip(data, name, it, instance, fallback),
    }
}

pub fn reprocess_zip(
    data: Vec<u8>,
    name: impl AsRef<str>,
    kind: FileType,
    instance: Instance,
    fallback: Option<&str>,
) -> Result<()> {
    match kind {
        FileType::Gzip => {
            let (data, kind) = extract_gzip(data, &name)?;

            reprocess_zip(data, name, kind, instance, fallback)
        }

        FileType::Xz => {
            let (data, kind) = extract_xz(data, &name)?;

            reprocess_zip(data, name, kind, instance, fallback)
        }

        // TODO:
        // FileType::Rar => {
        //     let data = extract_rar(data, &instance)?;

        //     reprocess_zip(data, name, FileType::Zip, instance, fallback)
        // }
        FileType::Tar => {
            let data = extract_tar(data, &instance)?;

            reprocess_zip(data, name, FileType::Zip, instance, fallback)
        }

        FileType::Zip => {
            let tmp_dir = instance
                .cache_dir()
                .join(format!("zip_process_{}.d", random_string(8)));

            let mut archive = ZipArchive::new(Cursor::new(data))?;

            archive.extract(&tmp_dir)?;

            // Some names are case-sensitive, so we don't lowercase them.
            let names = archive.file_names().collect::<Vec<_>>();

            if names.contains(&"BepInEx") {
                archive.extract(instance.data_dir())?;
            } else if names.contains(&"plugins") || names.contains(&"patchers") {
                archive.extract(instance.data_dir().join("BepInEx"))?;
            } else if names.iter().any(|v| v.ends_with(".dll")) {
                archive.extract(instance.data_dir().join("BepInEx").join("plugins"))?;
            } else {
                archive.extract(instance.data_dir().join(fallback.unwrap_or("")))?;
            }

            Ok(())
        }

        _ => Err(anyhow!("Could not reprocess the specified type!")),
    }
}

pub fn extract_gzip(data: Vec<u8>, name: impl AsRef<str>) -> Result<(Vec<u8>, FileType)> {
    let mut extracted = Vec::new();

    GzDecoder::new(Cursor::new(&data)).read_to_end(&mut extracted)?;

    let kind = detect_file_type(data, name)?;

    Ok((extracted, kind))
}

pub fn extract_xz(data: Vec<u8>, name: impl AsRef<str>) -> Result<(Vec<u8>, FileType)> {
    let mut extracted = Vec::new();

    XzDecoder::new(Cursor::new(&data)).read_to_end(&mut extracted)?;

    let kind = detect_file_type(data, name)?;

    Ok((extracted, kind))
}

pub fn extract_tar(data: Vec<u8>, instance: &Instance) -> Result<Vec<u8>> {
    let new_path = instance
        .cache_dir()
        .join(format!("tar_convert_{}.zip", random_string(8)));

    let tmp_dir = instance
        .cache_dir()
        .join(format!("tar_convert_{}.d", random_string(8)));

    Archive::new(Cursor::new(data)).unpack(&tmp_dir)?;
    zip_directory(&new_path, &tmp_dir)?;
    remove_dir_all(tmp_dir)?;

    Ok(fs::read(new_path)?)
}

pub fn zip_directory(path: impl AsRef<Path>, dir: impl AsRef<Path>) -> Result<()> {
    let out_file = File::create(&path)?;
    let mut zip = ZipWriter::new(out_file);

    for entry in WalkDir::new(&dir).into_iter().filter_map(|v| v.ok()) {
        zip.start_file(
            entry
                .path()
                .to_str()
                .unwrap()
                .replace(dir.as_ref().to_str().unwrap(), ""),
            SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
        )?;

        zip.write_all(&fs::read(entry.path())?.into_boxed_slice())?;
    }

    zip.finish()?;

    Ok(())
}
