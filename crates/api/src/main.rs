use std::env::args;

use anyhow::Result;
use wormhole_api::load_wasi_plugin;

pub fn main() -> Result<()> {
    let path = args().nth(1).unwrap();
    
    load_wasi_plugin(&path)?;

    Ok(())
}
