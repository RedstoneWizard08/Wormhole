// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
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
