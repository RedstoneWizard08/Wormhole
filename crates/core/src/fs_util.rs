use std::{
    fs::{copy, create_dir_all, read_dir, rename},
    io::Result,
    path::Path,
};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    create_dir_all(&dst)?;

    if !src.as_ref().is_dir() {
        copy(&src, dst.as_ref().join(src.as_ref().file_name().unwrap()))?;

        return Ok(());
    }

    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn rename_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    create_dir_all(&dst)?;

    if !src.as_ref().is_dir() {
        rename(&src, dst.as_ref().join(src.as_ref().file_name().unwrap()))?;

        return Ok(());
    }

    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            rename_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            rename(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
