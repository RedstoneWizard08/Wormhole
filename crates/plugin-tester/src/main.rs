use std::env::args;

use anyhow::{anyhow, Result};
use api::load_wasi_plugin;

pub fn main() -> Result<()> {
    let path = args().nth(1).ok_or(anyhow!("Missing path to plugin."))?;

    println!("[INFO] Trying to load plugin from {}", path);

    load_wasi_plugin(&path)?;

    Ok(())
}
