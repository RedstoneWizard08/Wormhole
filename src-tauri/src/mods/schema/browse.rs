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

impl Default for ModVersion {
    fn default() -> Self {
        return Self {
            friendly_version: None,
            game_version: None,
            id: None,
            created: None,
            download_path: None,
            changelog: None,
            downloads: None,
        };
    }
}

impl ModVersion {
    pub fn finish(&self) -> Self {
        let out = ModVersion {
            friendly_version: Some(self.friendly_version.clone().unwrap_or("".to_string())),
            game_version: Some(self.friendly_version.clone().unwrap_or("".to_string())),
            id: Some(self.id.unwrap_or( 0)),
            created: Some(self.created.clone().unwrap_or("".to_string())),
            download_path: Some(self.download_path.clone().unwrap_or("".to_string())),
            changelog: Some(self.changelog.clone().unwrap_or("".to_string())),
            downloads: Some(self.downloads.unwrap_or( 0)),
        };

        return out;
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

impl Default for ModInfo {
    fn default() -> Self {
        return Self {
            name: None,
            id: None,
            game: None,
            game_id: None,
            short_description: None,
            downloads: None,
            followers: None,
            author: None,
            default_version_id: None,
            shared_authors: None,
            background: None,
            bg_offset_y: None,
            license: None,
            website: None,
            donations: None,
            source_code: None,
            url: None,
            versions: None,
            description: None,
        };
    }
}

impl ModInfo {
    pub fn finish(&self, browse: bool) -> Self {
        let mut out = ModInfo::default();
        let mut _versions = Vec::new();
        let mut bg_img = "https://spacedock.info/static/background.png".to_string();

        if browse {
            bg_img = "https://spacedock.info/static/background-s.png".to_string();
        }

        if let Some(versions) = self.versions.clone() {
            _versions = versions.iter().map(|v| v.finish()).collect();
        }

        out.name = Some(self.name.clone().unwrap_or("".to_string()));
        out.id = Some(self.id.unwrap_or(0));
        out.game = Some(self.game.clone().unwrap_or("".to_string()));
        out.game_id = Some(self.game_id.unwrap_or(0));
        out.short_description = Some(self.short_description.clone().unwrap_or("".to_string()));
        out.downloads = Some(self.downloads.unwrap_or(0));
        out.followers = Some(self.followers.unwrap_or(0));
        out.author = Some(self.author.clone().unwrap_or("".to_string()));
        out.default_version_id = Some(self.default_version_id.unwrap_or(0));
        out.shared_authors = Some(self.shared_authors.clone().unwrap_or(Vec::new()));
        
        out.background = Some(
            self.background.clone()
                .unwrap_or(bg_img),
        );

        out.bg_offset_y = Some(self.bg_offset_y.clone().unwrap_or("".to_string()));
        out.license = Some(self.license.clone().unwrap_or("".to_string()));
        out.website = Some(self.website.clone().unwrap_or("".to_string()));
        out.donations = Some(self.donations.clone().unwrap_or("".to_string()));
        out.source_code = Some(self.source_code.clone().unwrap_or("".to_string()));
        out.url = Some(self.url.clone().unwrap_or("".to_string()));
        out.versions = Some(_versions);
        out.description = Some(self.description.clone().unwrap_or("".to_string()));

        return out;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrowseResult {
    pub result: Option<Vec<ModInfo>>,
    pub count: Option<i32>,
    pub pages: Option<i32>,
    pub page: Option<i32>,
}

impl Default for BrowseResult {
    fn default() -> Self {
        return Self {
            result: None,
            count: None,
            pages: None,
            page: None,
        };
    }
}

impl BrowseResult {
    pub fn finish(&self) -> Self {
        let mut out = BrowseResult::default();
        let mut _res = Vec::new();

        if let Some(result) = self.result.clone() {
            _res = result.iter().map(|v| v.finish(true)).collect();
        }

        out.result = Some(_res);
        out.count = Some(self.count.unwrap_or(0));
        out.pages = Some(self.pages.unwrap_or(0));
        out.page = Some(self.page.unwrap_or(0));

        return out;
    }
}
