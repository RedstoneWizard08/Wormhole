use anyhow::Result;

use crate::vanilla::{manifest::VersionManifest, meta::PistonMetaManifest, META_API};

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
