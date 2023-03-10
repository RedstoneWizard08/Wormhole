use serde::{Serialize, Deserialize};
use super::schema::browse::{BrowseResult, ModInfo};
use crate::models::latest::LatestSchema;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceDockAPI {
    pub base: String,
}

impl Default for SpaceDockAPI {
    fn default() -> Self {
        return Self::new();
    }
}

impl SpaceDockAPI {
    pub fn new() -> Self {
        return Self {
            base: SpaceDockAPI::get_default_api_url(),
        };
    }

    pub fn get_default_api_url() -> String {
        return "https://spacedock.info/api".to_string();
    }

    pub async fn get_mod(&self, id: i32) -> ModInfo {
        let uri = format!("{}/mod/{}", self.base, id);
        let resp = reqwest::get(uri).await.unwrap();
        let data = resp.json::<ModInfo>().await.unwrap();

        return data.finish(false);
    }

    pub async fn get_mods(&self, page: i32, count: i32) -> BrowseResult {
        let uri = format!("{}/browse?page={}&count={}", self.base, page, count);
        let resp = reqwest::get(uri).await.unwrap();
        let data = resp.json::<BrowseResult>().await.unwrap();

        return data.finish();
    }

    pub async fn get_mods_for_game(&self, game: i32, page: i32, count: i32) -> BrowseResult {
        let uri = format!("{}/browse?page={}&count={}&game_id={}", self.base, page, count, game);
        let resp = reqwest::get(uri).await.unwrap();
        let text = resp.text().await.unwrap();
        let data = serde_json::from_str::<BrowseResult>(&text);

        if let Ok(data) = data {
            return data.finish();
        }

        panic!("Found: {}", text);
    }

    pub async fn get_mod_download(&self, id: i32) -> String {
        let uri = format!("{}/mod/${}/latest", self.base.clone(), id);
        let resp = reqwest::get(uri).await.unwrap();
        let text = resp.text().await.unwrap();
        let data = serde_json::from_str::<LatestSchema>(&text).unwrap();

        return format!("{}{}", self.base, data.download_path);
    }
}
