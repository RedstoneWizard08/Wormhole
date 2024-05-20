use std::{
    fs::{self, remove_dir_all, rename, write, File},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use crate::magic::{detect_file_type, FileType};
use anyhow::Result;

use data::{
    instance::Instance,
    schema::instances::dsl::{id as iid, instances},
    Conn,
};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use flate2::read::GzDecoder;
use rar::Archive as Rar;
use tar::Archive;
use walkdir::WalkDir;
use whcore::{random_string, rename_dir_all};
use xz::read::XzDecoder;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub fn extract_file(
    db: &mut Conn,
    path: PathBuf,
    kind: FileType,
    instance: Instance,
    fallback: Option<&str>,
) -> Result<()> {
    let meta = instances
        .filter(iid.eq(instance.id.unwrap()))
        .select(Instance::as_select())
        .get_result(db)?;

    match kind {
        FileType::Jar => Ok(rename(
            path.clone(),
            meta.data_dir()
                .join("mods")
                .join(path.file_name().unwrap().to_str().unwrap()),
        )?),

        FileType::ResourcePack => Ok(rename(
            path.clone(),
            meta.data_dir()
                .join("resourcepacks")
                .join(path.file_name().unwrap().to_str().unwrap()),
        )?),

        it => reprocess_zip(path, it, instance, meta, fallback),
    }
}

pub fn reprocess_zip(
    path: PathBuf,
    kind: FileType,
    instance: Instance,
    meta: Instance,
    fallback: Option<&str>,
) -> Result<()> {
    match kind {
        FileType::Gzip => {
            let (p, k, i) = extract_gzip(path, instance, &meta)?;

            reprocess_zip(p, k, i, meta, fallback)
        }

        FileType::Xz => {
            let (p, k, i) = extract_xz(path, instance, &meta)?;

            reprocess_zip(p, k, i, meta, fallback)
        }

        FileType::Rar => {
            let (p, i) = extract_rar(path, instance, &meta)?;

            reprocess_zip(p, FileType::Zip, i, meta, fallback)
        }

        FileType::Tar => {
            let (p, i) = extract_tar(path, instance, &meta)?;

            reprocess_zip(p, FileType::Zip, i, meta, fallback)
        }

        FileType::Zip => {
            let tmp_dir = meta
                .cache_dir()
                .join(format!("zip_process_{}.d", random_string(8)));

            let mut archive = ZipArchive::new(Cursor::new(fs::read(path)?))?;

            archive.extract(&tmp_dir)?;

            // Some names are case-sensitive, so we don't lowercase them.
            let names = archive.file_names().collect::<Vec<_>>();

            let dir_names = fs::read_dir(&tmp_dir)?
                .filter_map(|v| v.ok())
                .map(|v| v.path())
                .collect::<Vec<_>>();

            if names.contains(&"BepInEx") {
                for name in dir_names {
                    rename_dir_all(name, &meta.data_dir)?;
                }
            } else if names.contains(&"plugins") || names.contains(&"patchers") {
                for name in dir_names {
                    rename_dir_all(name, &meta.data_dir().join("BepInEx"))?;
                }
            } else if names.iter().any(|v| v.ends_with(".dll")) {
                for name in dir_names {
                    rename_dir_all(
                        name,
                        &meta.data_dir().join(fallback.unwrap_or("BepInEx/plugins")),
                    )?;
                }
            }

            Ok(())
        }

        _ => Err(anyhow!("Could not reprocess the specified type!")),
    }
}

pub fn extract_gzip(
    path: PathBuf,
    instance: Instance,
    meta: &Instance,
) -> Result<(PathBuf, FileType, Instance)> {
    let new_path = meta
        .cache_dir()
        .join(format!("gzip_convert_{}.bin", random_string(8)));

    let data = fs::read(path)?;
    let mut extracted = Vec::new();

    GzDecoder::new(Cursor::new(&data)).read_to_end(&mut extracted)?;
    write(&new_path, extracted)?;

    let kind = detect_file_type(data)?;

    Ok((new_path, kind, instance))
}

pub fn extract_xz(
    path: PathBuf,
    instance: Instance,
    meta: &Instance,
) -> Result<(PathBuf, FileType, Instance)> {
    let new_path = meta
        .cache_dir()
        .join(format!("xz_convert_{}.bin", random_string(8)));

    let data = fs::read(path)?;
    let mut extracted = Vec::new();

    XzDecoder::new(Cursor::new(&data)).read_to_end(&mut extracted)?;
    write(&new_path, extracted)?;

    let kind = detect_file_type(data)?;

    Ok((new_path, kind, instance))
}

pub fn extract_rar(
    path: PathBuf,
    instance: Instance,
    meta: &Instance,
) -> Result<(PathBuf, Instance)> {
    let new_path = meta
        .cache_dir()
        .join(format!("rar_convert_{}.zip", random_string(8)));

    let tmp_dir = meta
        .cache_dir()
        .join(format!("rar_convert_{}.d", random_string(8)));

    Rar::extract_all(path.to_str().unwrap(), tmp_dir.to_str().unwrap(), None)
        .map_err(|v| anyhow!(v))?;

    zip_directory(&new_path, &tmp_dir)?;
    remove_dir_all(tmp_dir)?;

    Ok((new_path, instance))
}

pub fn extract_tar(
    path: PathBuf,
    instance: Instance,
    meta: &Instance,
) -> Result<(PathBuf, Instance)> {
    let new_path = meta
        .cache_dir()
        .join(format!("tar_convert_{}.zip", random_string(8)));

    let tmp_dir = meta
        .cache_dir()
        .join(format!("tar_convert_{}.d", random_string(8)));

    Archive::new(File::open(path)?).unpack(&tmp_dir)?;
    zip_directory(&new_path, &tmp_dir)?;
    remove_dir_all(tmp_dir)?;

    Ok((new_path, instance))
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
