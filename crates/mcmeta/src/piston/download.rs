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
    env::consts::{ARCH, OS},
    fs,
    io::{Cursor, Read},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use zip::ZipArchive;

use crate::{
    download::{download_file, download_file_mem, download_file_with_client, DownloadCallbackFn},
    maven::coord::MavenCoordinate,
};

use super::{
    assets::AssetIndex,
    game::{
        get_game_manifest,
        library::{should_download_classifier, LibraryRef},
        manifest::GameManifest,
    },
    get_features,
    manifest::get_manifest,
};

pub async fn download_minecraft_jar(path: PathBuf, version: impl AsRef<str>) -> Result<()> {
    let manifest =
        get_game_manifest(get_manifest().await?.find(version.as_ref()).unwrap().url).await?;

    let bytes = reqwest::get(manifest.downloads.client.url)
        .await?
        .bytes()
        .await?;

    let parent = path.parent().unwrap();

    if !parent.exists() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, bytes)?;

    Ok(())
}

pub async fn download_libs(
    root: impl Into<PathBuf>,
    game: &GameManifest,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let root = root.into();

    for lib in &game.libraries {
        if !lib.should_download(&get_features()) {
            continue;
        }

        if let Some(downloads) = &lib.downloads {
            if let Some(artifact) = &downloads.artifact {
                download_file(
                    &root,
                    &artifact.url,
                    &artifact.path,
                    Some(&artifact.sha1),
                    callback,
                )
                .await?;
            }

            if let Some(classifiers) = &downloads.classifiers {
                for (key, artifact) in classifiers {
                    if should_download_classifier(key) {
                        download_file(
                            &root,
                            &artifact.url,
                            &artifact.path,
                            Some(&artifact.sha1),
                            callback,
                        )
                        .await?;
                    }
                }
            }
        } else {
            if let Some(url) = &lib.url {
                let coord = MavenCoordinate::from(&lib.name);

                download_file(
                    &root,
                    coord.url(url),
                    coord.path(),
                    Option::<String>::None,
                    callback,
                )
                .await?;
            }
        }
    }

    if let Some(config) = &game.logging {
        if let Some(client) = &config.client {
            download_file(
                &root,
                &client.file.url,
                &client.file.id,
                Some(&client.file.sha1),
                callback,
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn extract_natives(
    root: impl Into<PathBuf>,
    game: &GameManifest,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let root: PathBuf = root.into();

    for lib in &game.libraries {
        if lib.is_native() && lib.should_download(&get_features()) {
            if let Some(downloads) = &lib.downloads {
                if let Some(artifact) = &downloads.artifact {
                    let data =
                        download_file_mem(&artifact.url, Some(&artifact.sha1), callback).await?;
                    let reader = Cursor::new(data);
                    let mut jar = ZipArchive::new(reader)?;

                    for idx in 0..jar.len() {
                        let mut file = jar.by_index(idx)?;

                        if !file.is_file() {
                            continue;
                        }

                        let name = file.name().to_lowercase();
                        let mut buf = Vec::new();

                        if name.ends_with(".dll")
                            || name.ends_with(".so")
                            || name.ends_with(".dylib")
                        {
                            file.read_to_end(&mut buf)?;
                        }

                        let new_path = root.join(file.name().split("/").last().unwrap());

                        if !new_path.parent().unwrap().exists() {
                            fs::create_dir_all(new_path.parent().unwrap())?;
                        }

                        fs::write(new_path, buf)?;
                    }
                }
            }
        }
    }

    // This must run AFTER the rest.
    for lib in &game.libraries {
        let coord = MavenCoordinate::from(&lib.name);

        if coord.group == "org.lwjgl" && coord.artifact == "lwjgl" && coord.classifier.is_none() {
            const ARTIFACTS: &[&str] = &[
                "lwjgl",
                "lwjgl-tinyfd",
                "lwjgl-stb",
                "lwjgl-opengl",
                "lwjgl-openal",
                "lwjgl-jemalloc",
                "lwjgl-glfw",
                "lwjgl-freetype",
            ];

            match (OS, ARCH) {
                ("linux", "aarch64") => {
                    let artifacts = ARTIFACTS.iter().map(|v| format!("https://build.lwjgl.org/release/{}/bin/{}/{}-natives-linux-arm64.jar", coord.version.clone().unwrap(), v, v)).collect::<Vec<_>>();

                    for url in artifacts {
                        if reqwest::get(&url).await?.status().is_success() {
                            let data =
                                download_file_mem(&url, Option::<String>::None, callback).await?;
                            let reader = Cursor::new(data);
                            let mut jar = ZipArchive::new(reader)?;

                            for idx in 0..jar.len() {
                                let mut file = jar.by_index(idx)?;

                                if !file.is_file() {
                                    continue;
                                }

                                let name = file.name().to_lowercase();
                                let mut buf = Vec::new();

                                if name.ends_with(".dll")
                                    || name.ends_with(".so")
                                    || name.ends_with(".dylib")
                                {
                                    file.read_to_end(&mut buf)?;
                                }

                                let new_path = root.join(file.name().split("/").last().unwrap());

                                if !new_path.parent().unwrap().exists() {
                                    fs::create_dir_all(new_path.parent().unwrap())?;
                                }

                                fs::write(new_path, buf)?;
                            }
                        }
                    }
                }

                _ => {}
            };

            break;
        }
    }

    Ok(())
}

pub async fn download_lib_refs(
    root: impl Into<PathBuf>,
    libs: Vec<LibraryRef>,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let root = root.into();

    for lib in libs {
        if !lib.should_download(&get_features()) {
            continue;
        }

        if let Some(downloads) = lib.downloads {
            if let Some(artifact) = downloads.artifact {
                download_file(
                    &root,
                    artifact.url,
                    artifact.path,
                    Some(artifact.sha1),
                    callback,
                )
                .await?;
            }

            if let Some(classifiers) = downloads.classifiers {
                for (key, artifact) in classifiers {
                    if should_download_classifier(key) {
                        download_file(
                            &root,
                            artifact.url,
                            artifact.path,
                            Some(artifact.sha1),
                            callback,
                        )
                        .await?;
                    }
                }
            }
        } else {
            if let Some(url) = lib.url {
                let coord = MavenCoordinate::from(lib.name);

                download_file(
                    &root,
                    coord.url(url),
                    coord.path(),
                    Option::<String>::None,
                    callback,
                )
                .await?;
            }
        }
    }

    Ok(())
}

const CONCURRENT_REQUESTS: usize = 8;

pub async fn download_assets(
    root: impl Into<PathBuf>,
    index: AssetIndex,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let root = root.into();
    let len = index.objects.len() as u64;
    let count = AtomicU64::new(0);

    if let Some(callback) = &callback {
        callback(
            count.load(Ordering::Relaxed),
            len,
            vec![],
            &root,
            true,
            false,
        );
    }

    let mut real = Vec::new();

    // This is a DUMB HACK and I HATE it.
    for item in index.objects.clone() {
        real.push((item, root.clone()));
    }

    let client = Client::new();

    // HEY GUYS ITS ME HERE WE STREAMING ON THE VEC AND TODAY WERE GONNA BE PLAYING
    // THIS NEW GAME THAT JUST CAME OUT CALLED FIGHTING TOKIO AND FUTURES FOR CONCURRENCY
    // AND FASTER DOWNLOADS
    // holy sh** that sped up the download by like 100x wtf - shared connections pools are insane
    let futures = tokio_stream::iter(real)
        .map(|((_, asset), rt)| {
            let client = client.clone();

            async move {
                let start = &asset.hash[..2];
                let path = format!("objects/{}/{}", start, asset.hash);

                download_file_with_client(
                    &rt,
                    client,
                    asset.url(),
                    path,
                    Option::<String>::None,
                    &None,
                )
                .await
                .unwrap();
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    futures
        .for_each(|_| async {
            count.fetch_add(1, Ordering::Relaxed);

            if let Some(callback) = &callback {
                callback(
                    count.load(Ordering::Relaxed),
                    len,
                    vec![],
                    &root.clone(),
                    false,
                    false,
                );
            }
        })
        .await;

    if let Some(callback) = &callback {
        callback(
            count.load(Ordering::Relaxed),
            len,
            vec![],
            &root,
            false,
            true,
        );
    }

    Ok(())
}
