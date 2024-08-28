#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: Versioning,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
    pub release: String,
    pub latest: String,
    pub last_updated: String,
    pub versions: Versions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    pub version: Vec<String>,
}
