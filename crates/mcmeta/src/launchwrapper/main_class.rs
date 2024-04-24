#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainClassConfig {
    pub client: String,
    pub server: String,
    pub server_launcher: String,
}
