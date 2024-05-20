use anyhow::Result;
use const_format::formatcp;
use std::fs;

pub const CONFIG_PATH: &str = formatcp!("{}/tauri.conf.json.in", env!("CARGO_MANIFEST_DIR"));
pub const CONFIG_PATH_OUT: &str = formatcp!("{}/tauri.conf.json", env!("CARGO_MANIFEST_DIR"));

pub const ROOT: &str = formatcp!("{}/../..", env!("CARGO_MANIFEST_DIR"));
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let mut data = fs::read_to_string(CONFIG_PATH)?;

    data = data.replace("<version>", VERSION);
    data = data.replace("<root>", &ROOT.replace("\\", "/")); // windows sucks

    fs::write(CONFIG_PATH_OUT, data)?;
    tauri_build::build();

    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");

    Ok(())
}
