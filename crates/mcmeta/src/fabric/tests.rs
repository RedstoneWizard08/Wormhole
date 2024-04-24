use std::path::PathBuf;

use anyhow::Result;

use crate::{
    fabric::{
        get_fabric_launchwrapper, get_fabric_launchwrapper_artifact, get_fabric_loader,
        get_fabric_versions,
    },
    maven::artifact::download_maven_artifacts,
    test_util::library_download_callback,
};

#[tokio::test]
#[serial_test::serial]
pub async fn test_download_fabric_libs() -> Result<()> {
    let root = PathBuf::from("./test/libraries");
    let latest = get_fabric_versions().await?.versioning.latest;
    let config = get_fabric_launchwrapper(&latest).await?;

    let mut libs = vec![
        get_fabric_loader(&latest),
        get_fabric_launchwrapper_artifact(&latest),
    ];

    libs.extend(config.libraries.all());

    download_maven_artifacts(&root, libs, Some(Box::new(library_download_callback))).await?;

    Ok(())
}
