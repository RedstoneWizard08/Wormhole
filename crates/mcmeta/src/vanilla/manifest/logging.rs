#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LoggingConfigs {
    pub client: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LoggingConfig {
    pub argument: String,
    pub file: LoggingFile,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}
