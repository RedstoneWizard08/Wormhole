use std::{fs::File, io::{self, Cursor}, path::PathBuf};

use serde::{Serialize, Deserialize};
use tauri::Window;
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadProgress {
    pub total: u64,
    pub received: usize,
}

pub struct Downloader;

impl Downloader {
    pub async fn download(url: String, target: PathBuf, window: Window) {
        let resp = reqwest::get(url).await.unwrap();
        let size = resp.content_length().unwrap();

        let mut recv = 0;
        let mut stream = resp.bytes_stream();
        let mut bytes: Vec<u8> = Vec::new();

        window.emit("download_progress", DownloadProgress {
            total: size,
            received: recv,
        }).unwrap();

        while let Some(Ok(item)) = stream.next().await {
            recv += item.len();

            bytes.append(&mut item.to_vec());

            window.emit("download_progress", DownloadProgress {
                total: size,
                received: recv,
            }).unwrap();
        }

        let mut bytes_cursor = Cursor::new(bytes);
        let mut out_file = File::create(target).unwrap();

        io::copy(&mut bytes_cursor, &mut out_file).unwrap();
    }
}
