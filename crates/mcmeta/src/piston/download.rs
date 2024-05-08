use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::download::{download_file, DownloadCallbackFn};

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
    game: GameManifest,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let root = root.into();

    for lib in game.libraries {
        if !lib.should_download(&get_features()) {
            continue;
        }

        if let Some(artifact) = lib.downloads.artifact {
            download_file(
                &root,
                artifact.url,
                artifact.path,
                Some(artifact.sha1),
                callback,
            )
            .await?;
        }

        if let Some(classifiers) = lib.downloads.classifiers {
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

        if let Some(artifact) = lib.downloads.artifact {
            download_file(
                &root,
                artifact.url,
                artifact.path,
                Some(artifact.sha1),
                callback,
            )
            .await?;
        }

        if let Some(classifiers) = lib.downloads.classifiers {
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
    }

    Ok(())
}

pub async fn download_assets(
    root: impl Into<PathBuf>,
    index: AssetIndex,
    callback: &Option<DownloadCallbackFn>,
) -> Result<()> {
    let root = root.into();
    let len = index.objects.len() as u64;
    let mut count = 0;

    if let Some(callback) = &callback {
        callback(count, len, vec![], &root, true, false);
    }

    for (path, asset) in index.objects {
        download_file(&root, asset.url(), path, Option::<String>::None, &None).await?;

        count += 1;

        if let Some(callback) = &callback {
            callback(count, len, vec![], &root, false, false);
        }
    }

    if let Some(callback) = &callback {
        callback(count, len, vec![], &root, false, true);
    }

    Ok(())
}
