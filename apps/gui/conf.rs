use std::{fs, env};

fn main() {
    let config_path = format!("{}/tauri.conf.json.in", env!("CARGO_MANIFEST_DIR"));
    let config_path_out = format!("{}/tauri.conf.json", env!("CARGO_MANIFEST_DIR"));
    let root = format!("{}/../..", env!("CARGO_MANIFEST_DIR"));
    let version = env!("CARGO_PKG_VERSION");

    let mut data = fs::read_to_string(config_path).unwrap();

    data = data.replace("<version>", version);
    data = data.replace("<root>", &root);

    fs::write(config_path_out, data).unwrap();
}
