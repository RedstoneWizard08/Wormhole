use serde::{Serialize, Deserialize};

use super::schema::browse::{BrowseResult, ModInfo};

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
        let data = resp.json::<BrowseResult>().await.unwrap();

        return data.finish();
    }
}
