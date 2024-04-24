use super::download::FileDownloadRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfigs {
    pub client: Option<LogConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub argument: String,
    pub file: FileDownloadRef,

    #[serde(rename = "type")]
    pub kind: String,
}
