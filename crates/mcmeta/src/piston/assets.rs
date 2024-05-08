use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub hash: String,
    pub size: i32,
}

impl Asset {
    pub fn url(&self) -> String {
        format!(
            "https://resources.download.minecraft.net/{0:.2}/{0}",
            self.hash
        )
    }
}

pub async fn get_asset_index(url: impl AsRef<str>) -> Result<AssetIndex> {
    Ok(reqwest::get(url.as_ref()).await?.json().await?)
}
