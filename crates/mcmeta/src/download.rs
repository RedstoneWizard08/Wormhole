use std::path::PathBuf;

use anyhow::{anyhow, Result};
use reqwest::Client;
use sha1::{Digest, Sha1};
use tokio::fs;
use tokio_stream::StreamExt;

fn _dummy(_received: u64, _total: u64, _chunk: Vec<u8>, _path: &PathBuf, _start: bool, _end: bool) {
}

pub type DownloadCallbackFn = Box<dyn Fn(u64, u64, Vec<u8>, &PathBuf, bool, bool) + Send + Sync>;

pub async fn download_file(
    root: &PathBuf,
    url: impl AsRef<str>,
    path: impl Into<PathBuf>,
    sha1: Option<impl AsRef<str>>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let url = url.as_ref();
    let hash = sha1.map(|v| v.as_ref().to_string());
    let path = root.join(path.into());

    if path.exists() {
        if let Some(callback) = callback {
            callback(0, 0, vec![], &path, false, false);
        }

        return Ok(());
    }

    let req = reqwest::get(url).await?;
    let len = req.content_length().unwrap_or_default(); // TODO: Use this
    let mut recv = 0;
    let mut stream = req.bytes_stream();
    let mut buf = Vec::new();

    if let Some(callback) = callback {
        callback(recv, len, vec![], &path, true, false);
    }

    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            buf.append(&mut chunk.to_vec());
            recv += chunk.len() as u64;

            if let Some(callback) = callback {
                callback(recv, len, chunk.to_vec(), &path, false, false);
            }
        }
    }

    if let Some(hash) = hash {
        let mut hasher = Sha1::new();

        hasher.update(&buf);

        let found_hash = format!("{:x}", hasher.finalize());

        if found_hash != hash {
            return Err(anyhow!(
                "SHA-1 hashes did not match! '{}' (found) != '{}' (expected)",
                found_hash,
                hash
            ));
        }
    }

    if !path.parent().unwrap().exists() {
        fs::create_dir_all(path.parent().unwrap()).await?;
    }

    fs::write(&path, buf).await?;

    if let Some(callback) = callback {
        callback(recv, len, vec![], &path, false, true);
    }

    Ok(())
}

pub async fn download_file_with_client(
    root: &PathBuf,
    client: Client,
    url: impl AsRef<str>,
    path: impl Into<PathBuf>,
    sha1: Option<impl AsRef<str>>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let url = url.as_ref();
    let hash = sha1.map(|v| v.as_ref().to_string());
    let path = root.join(path.into());

    if path.exists() {
        if let Some(callback) = callback {
            callback(0, 0, vec![], &path, false, false);
        }

        return Ok(());
    }

    let req = client.get(url).send().await?;
    let len = req.content_length().unwrap_or_default(); // TODO: Use this
    let mut recv = 0;
    let mut stream = req.bytes_stream();
    let mut buf = Vec::new();

    if let Some(callback) = callback {
        callback(recv, len, vec![], &path, true, false);
    }

    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            buf.append(&mut chunk.to_vec());
            recv += chunk.len() as u64;

            if let Some(callback) = callback {
                callback(recv, len, chunk.to_vec(), &path, false, false);
            }
        }
    }

    if let Some(hash) = hash {
        let mut hasher = Sha1::new();

        hasher.update(&buf);

        let found_hash = format!("{:x}", hasher.finalize());

        if found_hash != hash {
            return Err(anyhow!(
                "SHA-1 hashes did not match! '{}' (found) != '{}' (expected)",
                found_hash,
                hash
            ));
        }
    }

    if !path.parent().unwrap().exists() {
        fs::create_dir_all(path.parent().unwrap()).await?;
    }

    fs::write(&path, buf).await?;

    if let Some(callback) = callback {
        callback(recv, len, vec![], &path, false, true);
    }

    Ok(())
}

pub async fn download_file_mem(
    url: impl AsRef<str>,
    sha1: Option<impl AsRef<str>>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<Vec<u8>> {
    let url = url.as_ref();
    let hash = sha1.map(|v| v.as_ref().to_string());
    let req = reqwest::get(url).await?;
    let len = req.content_length().unwrap_or_default(); // TODO: Use this
    let mut recv = 0;
    let mut stream = req.bytes_stream();
    let mut buf = Vec::new();

    if let Some(callback) = callback {
        callback(recv, len, vec![], &PathBuf::from("/tmp/mem"), true, false);
    }

    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            buf.append(&mut chunk.to_vec());
            recv += chunk.len() as u64;

            if let Some(callback) = callback {
                callback(
                    recv,
                    len,
                    chunk.to_vec(),
                    &PathBuf::from("/tmp/mem"),
                    false,
                    false,
                );
            }
        }
    }

    if let Some(hash) = hash {
        let mut hasher = Sha1::new();

        hasher.update(&buf);

        let found_hash = format!("{:x}", hasher.finalize());

        if found_hash != hash {
            return Err(anyhow!(
                "SHA-1 hashes did not match! '{}' (found) != '{}' (expected)",
                found_hash,
                hash
            ));
        }
    }

    if let Some(callback) = callback {
        callback(recv, len, vec![], &PathBuf::from("/tmp/mem"), false, true);
    }

    Ok(buf)
}
