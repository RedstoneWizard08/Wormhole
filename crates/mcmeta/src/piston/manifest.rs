use anyhow::Result;

pub const MC_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersionContainer,
    pub versions: Vec<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestVersionContainer {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub url: String,
    pub time: String,
    pub sha1: String,

    #[serde(rename = "type")]
    pub kind: String,

    #[serde(rename = "releaseTime")]
    pub release_time: String,

    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}

pub async fn get_manifest() -> Result<VersionManifest> {
    Ok(reqwest::get(MC_MANIFEST_URL).await?.json().await?)
}

impl VersionManifest {
    pub fn find(&self, id: impl AsRef<str>) -> Option<Version> {
        self.versions
            .iter()
            .find(|v| v.id == id.as_ref())
            .map(|v| v.clone())
    }
}
