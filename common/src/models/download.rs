use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadProgress {
    pub total: u64,
    pub received: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadComplete {
    pub size: u64,
}
