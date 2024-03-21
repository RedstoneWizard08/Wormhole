use std::{
    fs::File,
    io::{self, Cursor},
    path::PathBuf,
};

use tokio_stream::StreamExt;

pub struct Downloader;

impl Downloader {
    pub async fn download<W>(
        url: String,
        target: PathBuf,
        progress_callback: fn(u64, usize, W) -> (),
        finished_callback: fn(u64, W) -> (),
        window: W,
    ) where
        W: Clone,
    {
        let resp = reqwest::get(url).await.unwrap();
        let total = resp.content_length().unwrap();

        let mut received = 0;
        let mut stream = resp.bytes_stream();
        let mut bytes: Vec<u8> = Vec::new();

        progress_callback(total, received, window.clone());

        while let Some(Ok(item)) = stream.next().await {
            received += item.len();

            bytes.append(&mut item.to_vec());

            progress_callback(total, received, window.clone());
        }

        let mut bytes_cursor = Cursor::new(bytes);
        let mut out_file = File::create(target).unwrap();

        io::copy(&mut bytes_cursor, &mut out_file).unwrap();

        finished_callback(total, window);
    }
}
