#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Downloads {
    pub client: Download,
    pub client_mappings: Option<Download>,
    pub server: Option<Download>, // Missing below 1.2.5
    pub server_mappings: Option<Download>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Download {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}
