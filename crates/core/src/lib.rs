pub mod config;
pub mod dirs;
pub mod finder;
pub mod manager;
pub mod messaging;
pub mod state;
pub mod traits;

#[macro_use]
extern crate serde;

#[macro_use]
pub extern crate async_trait;

use dirs::Dirs;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use specta::{NamedType, TypeMap};
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

pub fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub trait Boolify<T> {
    fn bool(self) -> std::result::Result<T, bool>;
}

impl<T, E> Boolify<T> for std::result::Result<T, E> {
    fn bool(self) -> std::result::Result<T, bool> {
        self.map_err(|_| false)
    }
}

impl<T> Boolify<T> for Option<T> {
    fn bool(self) -> std::result::Result<T, bool> {
        self.ok_or(false)
    }
}

pub fn merge_type_maps(maps: Vec<TypeMap>) -> TypeMap {
    let mut map = TypeMap::default();

    for ty_map in maps {
        for (id, ty) in ty_map.iter() {
            map.insert(id, ty.clone());
        }
    }

    map
}

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = Dirs::named_data_type(&mut map, &[]);
    map.insert(Dirs::SID, ty);

    map
}
