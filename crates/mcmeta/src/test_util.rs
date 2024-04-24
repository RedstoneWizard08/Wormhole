use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LIBS_PBAR: Arc<Mutex<Option<ProgressBar>>> = Arc::new(Mutex::new(None));
    pub static ref ASSETS_PBAR: Arc<Mutex<Option<ProgressBar>>> = Arc::new(Mutex::new(None));
}

pub fn library_download_callback(
    recv: u64,
    total: u64,
    _chunk: Vec<u8>,
    path: &PathBuf,
    start: bool,
    end: bool,
) {
    let path = path.clone();
    let file = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if start {
        *LIBS_PBAR.lock().unwrap() = Some(
            ProgressBar::new(total).with_style(
                ProgressStyle::with_template(
                    "[{prefix}] {wide_bar:.cyan/blue} {bytes}/{total_bytes} {msg}",
                )
                .unwrap(),
            ),
        );

        LIBS_PBAR.lock().unwrap().as_mut().unwrap().set_prefix(file);
    } else if end {
        LIBS_PBAR.lock().unwrap().as_mut().unwrap().finish();
        *LIBS_PBAR.lock().unwrap() = None;
    } else {
        if let Some(bar) = LIBS_PBAR.lock().unwrap().as_mut() {
            bar.set_position(recv);
        }
    }
}

pub fn asset_download_callback(
    recv: u64,
    total: u64,
    _chunk: Vec<u8>,
    _path: &PathBuf,
    start: bool,
    end: bool,
) {
    if start {
        *ASSETS_PBAR.lock().unwrap() = Some(
            ProgressBar::new(total).with_style(
                ProgressStyle::with_template(
                    "[assets] {wide_bar:.cyan/blue} {pos:>7}/{len:>7} {msg}",
                )
                .unwrap(),
            ),
        );
    } else if end {
        ASSETS_PBAR.lock().unwrap().as_mut().unwrap().finish();
        *ASSETS_PBAR.lock().unwrap() = None;
    } else {
        ASSETS_PBAR
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .set_position(recv);
    }
}
