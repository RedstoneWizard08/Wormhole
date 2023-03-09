use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModVersion {
    pub friendly_version: Option<String>,
    pub game_version: Option<String>,
    pub id: Option<i32>,
    pub created: Option<String>,
    pub download_path: Option<String>,
    pub changelog: Option<String>,
    pub downloads: Option<i32>,
}

impl ModVersion {
    pub fn finish(&mut self) -> &Self {
        self.friendly_version = Some(self.friendly_version .unwrap_or("".to_string()));
        self.game_version = Some(self.friendly_version .unwrap_or("".to_string()));
        self.id = Some(self.id.unwrap_or( 0));
        self.created = Some(self.created.unwrap_or("".to_string()));
        self.download_path = Some(self.download_path.unwrap_or("".to_string()));
        self.changelog = Some(self.changelog.unwrap_or("".to_string()));
        self.downloads = Some(self.downloads.unwrap_or( 0));

        return self;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModInfo {
    pub name: Option<String>,
    pub id: Option<i32>,
    pub game: Option<String>,
    pub game_id: Option<i32>,
    pub short_description: Option<String>,
    pub downloads: Option<i32>,
    pub followers: Option<i32>,
    pub author: Option<String>,
    pub default_version_id: Option<i32>,
    pub shared_authors: Option<Vec<Value>>,
    pub background: Option<String>,
    pub bg_offset_y: Option<String>,
    pub license: Option<String>,
    pub website: Option<String>,
    pub donations: Option<String>,
    pub source_code: Option<String>,
    pub url: Option<String>,
    pub versions: Option<Vec<ModVersion>>,
    pub description: Option<String>,
}

impl ModInfo {
    pub fn finish(&mut self, browse: bool) -> &Self {
        let mut _versions = Vec::new();
        let mut bg_img = "https://spacedock.info/static/background.png".to_string();

        if browse {
            bg_img = "https://spacedock.info/static/background-s.png".to_string();
        }

        if let Some(versions) = self.versions {
            _versions = versions.iter().map(|v| v.finish()).cloned().collect();
        }

        self.name = Some(self.name.unwrap_or("".to_string()));
        self.id = Some(self.id.unwrap_or(0));
        self.game = Some(self.game.unwrap_or("".to_string()));
        self.game_id = Some(self.game_id.unwrap_or(0));
        self.short_description = Some(self.short_description.unwrap_or("".to_string()));
        self.downloads = Some(self.downloads.unwrap_or(0));
        self.followers = Some(self.followers.unwrap_or(0));
        self.author = Some(self.author.unwrap_or("".to_string()));
        self.default_version_id = Some(self.default_version_id.unwrap_or(0));
        self.shared_authors = Some(self.shared_authors.unwrap_or(Vec::new()));
        
        self.background = Some(
            self.background
                .unwrap_or(bg_img),
        );

        self.bg_offset_y = Some(self.bg_offset_y.unwrap_or("".to_string()));
        self.license = Some(self.license.unwrap_or("".to_string()));
        self.website = Some(self.website.unwrap_or("".to_string()));
        self.donations = Some(self.donations.unwrap_or("".to_string()));
        self.source_code = Some(self.source_code.unwrap_or("".to_string()));
        self.url = Some(self.url.unwrap_or("".to_string()));
        self.versions = Some(_versions);
        self.description = Some(self.description.unwrap_or("".to_string()));

        return self;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrowseResult {
    pub result: Option<Vec<ModInfo>>,
    pub count: Option<i32>,
    pub pages: Option<i32>,
    pub page: Option<i32>,
}

impl BrowseResult {
    pub fn finish(&mut self) -> &Self {
        let mut _res = Vec::new();

        if let Some(result) = self.result {
            _res = result.iter().map(|v| v.finish(true)).cloned().collect();
        }

        self.result = Some(_res);
        self.count = Some(self.count.unwrap_or(0));
        self.pages = Some(self.pages.unwrap_or(0));
        self.page = Some(self.page.unwrap_or(0));

        return self;
    }
}
