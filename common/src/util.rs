use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_platform_data_dir() -> String {
    return std::env::var("APPDATA").unwrap();
}

#[cfg(target_os = "linux")]
fn get_platform_data_dir() -> String {
    return std::env::var("HOME").unwrap() + "/.local/share";
}

#[cfg(target_os = "macos")]
fn get_platform_data_dir() -> String {
    return std::env::var("HOME").unwrap() + "/Library/Application Support";
}

pub fn get_data_dir() -> PathBuf {
    return PathBuf::from(get_platform_data_dir()).join("Wormhole");
}
