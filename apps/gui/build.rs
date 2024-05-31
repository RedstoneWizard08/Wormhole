use anyhow::Result;
use const_format::formatcp;
use std::fs;

pub const CONFIG_PATH: &str = formatcp!("{}/tauri.conf.json.in", env!("CARGO_MANIFEST_DIR"));
pub const CONFIG_PATH_OUT: &str = formatcp!("{}/tauri.conf.json", env!("CARGO_MANIFEST_DIR"));

pub const ROOT: &str = formatcp!("{}/../..", env!("CARGO_MANIFEST_DIR"));
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(debug_assertions))]
fn build() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");

    std::process::Command::new("pnpm")
        .arg("run")
        .arg("app:build")
        .current_dir(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[cfg(debug_assertions)]
fn build() {
    println!("Not running the gui's build script, this is a debug build.");
}

fn main() -> Result<()> {
    let mut data = fs::read_to_string(CONFIG_PATH)?;

    data = data.replace("<version>", VERSION);
    data = data.replace("<root>", &ROOT.replace("\\", "/")); // windows sucks

    fs::write(CONFIG_PATH_OUT, data)?;
    build();
    tauri_build::build();

    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");

    Ok(())
}
