#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FabricMetaVersion {
    pub separator: String,
    pub build: i32,
    pub maven: String,
    pub version: String,
    pub stable: Option<bool>,
}
