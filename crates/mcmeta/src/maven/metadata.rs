use crate::util::unwrap_list;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenMetadata {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: MavenVersioning,
}

impl MavenMetadata {
    pub fn latest(&self) -> String {
        self.versioning.latest.clone()
    }

    pub fn release(&self) -> String {
        self.versioning.release.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenVersioning {
    pub latest: String,
    pub release: String,
    pub last_updated: u64,

    #[serde(deserialize_with = "unwrap_list")]
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionList {
    #[serde(rename = "$value")]
    pub versions: Vec<String>,
}
