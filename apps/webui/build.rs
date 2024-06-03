use std::env::consts::OS;

#[cfg(not(debug_assertions))]
fn build() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");

    std::process::Command::new("pnpm")
        .arg("run")
        .arg("web:build")
        .current_dir(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[cfg(debug_assertions)]
fn build() {
    println!("Not running the frontend's build script, this is a debug build.");
}

fn main() {
    if OS == "windows" {
        return;
    }

    build();
}
