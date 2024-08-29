use std::collections::HashMap;

use anyhow::Result;
use whcore::CoreManager;

use crate::{
    fabric::Fabric,
    forge::Forge,
    installer::libs::LibraryInstaller,
    loader::LoaderData,
    neoforge::NeoForge,
    quilt::Quilt,
    vanilla::{manifest::VersionManifest, meta::PistonMetaManifest, META_API},
};

// Managed to download and parse 767 version manifests in 17.22 seconds!
// Way to go, rust, serde, and reqwest!
#[tokio::test]
pub async fn test_manifest_parsing() -> Result<()> {
    let data = reqwest::get(META_API)
        .await?
        .json::<PistonMetaManifest>()
        .await?
        .versions;

    for (i, ver) in data.iter().enumerate() {
        println!("Testing {} of {}: {}", i + 1, data.len(), ver.id.clone());

        let raw = reqwest::get(ver.url.clone()).await?.text().await?;
        let res = serde_json::from_str::<VersionManifest>(&raw);

        assert!(
            res.is_ok(),
            "Could not parse version manifest for {}: {}",
            ver.id,
            res.unwrap_err()
        );
    }

    Ok(())
}

pub async fn test_loader_manifest_parsing(loader: impl LoaderData) -> Result<()> {
    let data = loader.all_versions().await?;

    for (i, ver) in data[0..15].iter().enumerate() {
        let v = ver.version.clone().unwrap();

        println!("Testing {} of {}: {}", i + 1, data.len(), &v);

        let res = loader.get_version_manifest(ver.clone(), "1.20.1").await;

        assert!(
            res.is_ok(),
            "Could not parse version manifest for {}: {}",
            v,
            res.unwrap_err()
        );
    }

    Ok(())
}

#[tokio::test]
pub async fn test_forge_manifest_parsing() -> Result<()> {
    test_loader_manifest_parsing(Forge).await
}

#[tokio::test]
pub async fn test_neoforge_manifest_parsing() -> Result<()> {
    test_loader_manifest_parsing(NeoForge).await
}

#[tokio::test]
pub async fn test_fabric_manifest_parsing() -> Result<()> {
    test_loader_manifest_parsing(Fabric).await
}

#[tokio::test]
pub async fn test_quilt_manifest_parsing() -> Result<()> {
    test_loader_manifest_parsing(Quilt).await
}

#[tokio::test]
pub async fn test_library_installation() -> Result<()> {
    let mf = NeoForge
        .get_version_manifest(
            NeoForge
                .versions_for("1.21.1")
                .await?
                .first()
                .unwrap()
                .clone(),
            "1.21.1",
        )
        .await?;

    let total = mf.libraries.len();

    let res = LibraryInstaller::new()
        .add_listener(|p, d, t| {
            println!("Downloaded {}: {:.2}%", p.to_str().unwrap(), (d / t) * 100);
        })
        .install_libraries(
            CoreManager::get().dir("libraries"),
            mf.libraries,
            &HashMap::new(),
        )
        .await;

    assert!(
        res.is_ok(),
        "Could not download {} libraries: {}",
        total,
        res.unwrap_err()
    );

    Ok(())
}
