use std::path::PathBuf;

use anyhow::Result;

use crate::{
    maven::artifact::download_maven_artifacts,
    quilt::{
        get_quilt_launchwrapper, get_quilt_launchwrapper_artifact, get_quilt_loader,
        get_quilt_versions,
    },
    test_util::library_download_callback,
};

#[tokio::test]
#[serial_test::serial]
pub async fn test_download_quilt_libs() -> Result<()> {
    let root = PathBuf::from("./test/libraries");
    let latest = get_quilt_versions().await?.versioning.latest;
    let config = get_quilt_launchwrapper(&latest).await?;

    let mut libs = vec![
        get_quilt_loader(&latest),
        get_quilt_launchwrapper_artifact(&latest),
    ];

    libs.extend(config.libraries.all());

    download_maven_artifacts(&root, libs, Some(Box::new(library_download_callback))).await?;

    Ok(())
}
