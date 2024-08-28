#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ReposiliteVersions {
    pub is_snapshot: bool,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ReposiliteVersion {
    pub is_snapshot: bool,
    pub version: String,
}

