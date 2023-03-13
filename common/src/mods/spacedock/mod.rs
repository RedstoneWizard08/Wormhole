use std::time::Instant;
use serde::{Deserialize, Serialize};

use crate::models::latest::LatestSchema;

use super::schema::browse::{BrowseResult, ModInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceDockAPI {
    pub base: String,
    pub api_base: String,
}

impl Default for SpaceDockAPI {
    fn default() -> Self {
        return Self::new();
    }
}

impl SpaceDockAPI {
    pub fn new() -> Self {
        return Self {
            base: SpaceDockAPI::get_default_url(),
            api_base: SpaceDockAPI::get_default_api_url(),
        };
    }

    pub fn get_default_api_url() -> String {
        return "https://spacedock.info/api".to_string();
    }

    pub fn get_default_url() -> String {
        return "https://spacedock.info".to_string();
    }

    pub async fn get_mod(&self, id: i32) -> ModInfo {
        let uri = format!("{}/mod/{}", self.api_base, id);
        let resp = reqwest::get(uri).await.unwrap();
        let data = resp.json::<ModInfo>().await.unwrap();

        return data.finish(false);
    }

    pub async fn get_mods(&self, page: i32, count: i32) -> BrowseResult {
        let uri = format!("{}/browse?page={}&count={}", self.api_base, page, count);
        let resp = reqwest::get(uri).await.unwrap();
        let data = resp.json::<BrowseResult>().await.unwrap();

        return data.finish();
    }

    pub async fn get_mods_for_game(&self, game: i32, page: i32, count: i32) -> BrowseResult {
        let start_time = Instant::now();

        let uri = format!("{}/browse?page={}&count={}&game_id={}", self.api_base, page, count, game);
        let resp = reqwest::get(uri).await.unwrap();
        let text = resp.text().await.unwrap();
        let data = serde_json::from_str::<BrowseResult>(&text);

        let elapsed = start_time.elapsed();
        println!("API call took {:?}", elapsed);

        if let Ok(data) = data {
            return data.finish();
        }

        panic!("Found: {}", text);
    }

    pub async fn get_mod_download(&self, id: i32) -> String {
        let uri = format!("{}/mod/{}/latest", self.api_base, id);
        let resp = reqwest::get(uri).await.unwrap();
        let text = resp.text().await.unwrap();
        let data = serde_json::from_str::<LatestSchema>(&text).unwrap();

        return format!("{}{}", self.base, data.download_path);
    }
}
