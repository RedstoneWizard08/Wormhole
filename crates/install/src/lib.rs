use anyhow::Result;
use extract::extract_file;
use instance::Instance;
use magic::detect_file_type;
use std::{fs, path::PathBuf};

pub mod extract;
pub mod magic;

#[macro_use]
extern crate anyhow;

pub fn install_mod(file: PathBuf, instance: Instance, fallback_dir: Option<&str>) -> Result<()> {
    let data = fs::read(&file)?;
    let kind = detect_file_type(data)?;

    extract_file(file, kind, instance, fallback_dir)?;

    Ok(())
}
