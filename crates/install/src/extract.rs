use std::{
    fs::{self, remove_dir_all, File},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use crate::magic::{detect_file_type, FileType};
use anyhow::Result;

use data::instance::Instance;

use flate2::read::GzDecoder;
use tar::Archive;
use walkdir::WalkDir;
use whcore::util::random_string;
use xz::read::XzDecoder;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub fn extract_file(
    data: Vec<u8>,
    name: impl AsRef<str>,
    kind: FileType,
    instance: Instance,
    fallback: Option<&str>,
) -> Result<Vec<PathBuf>> {
    let name = name.as_ref();

    match kind {
        FileType::Jar => {
            if !instance.data_dir().join("mods").exists() {
                fs::create_dir_all(instance.data_dir().join("mods"))?;
            }

            let output = instance.data_dir().join("mods").join(name);

            fs::write(&output, data)?;

            Ok(vec![output])
        }

        FileType::ResourcePack => {
            if !instance.data_dir().join("resourcepacks").exists() {
                fs::create_dir_all(instance.data_dir().join("resourcepacks"))?;
            }

            let output = instance.data_dir().join("resourcepacks").join(name);

            fs::write(&output, data)?;

            Ok(vec![output])
        }

        it => reprocess_zip(data, name, it, instance, fallback),
    }
}

pub fn extract_zip(arc: &mut ZipArchive<Cursor<Vec<u8>>>, root: PathBuf) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for idx in 0..arc.len() {
        let mut item = arc.by_index(idx)?;

        if !item.is_file() {
            continue;
        }

        let mut data = Vec::new();

        item.read_to_end(&mut data)?;

        let path = root.join(item.enclosed_name().unwrap());
        let parent = path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(&parent)?;
            paths.push(parent.into());
        }

        fs::write(&path, data)?;
        paths.push(path);
    }

    Ok(paths)
}

pub fn reprocess_zip(
    data: Vec<u8>,
    name: impl AsRef<str>,
    kind: FileType,
    instance: Instance,
    fallback: Option<&str>,
) -> Result<Vec<PathBuf>> {
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
                extract_zip(&mut archive, instance.data_dir())
            } else if names.contains(&"plugins") || names.contains(&"patchers") {
                extract_zip(&mut archive, instance.data_dir().join("BepInEx"))
            } else if names.iter().any(|v| v.ends_with(".dll")) {
                extract_zip(
                    &mut archive,
                    instance.data_dir().join("BepInEx").join("plugins"),
                )
            } else {
                extract_zip(
                    &mut archive,
                    instance.data_dir().join(fallback.unwrap_or("")),
                )
            }
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
