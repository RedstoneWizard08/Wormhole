#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDownloads {
    pub client: DownloadRef,
    pub client_mappings: Option<DownloadRef>,
    pub server: Option<DownloadRef>,
    pub server_mappings: Option<DownloadRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRef {
    pub sha1: String,
    pub size: i32,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibDownloadRef {
    pub path: String,
    pub sha1: String,
    pub size: i32,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadRef {
    pub id: String,
    pub sha1: String,
    pub size: i32,
    pub url: String,
}
