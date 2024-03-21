#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            id: Some(self.id.unwrap_or(0)),
            created: Some(self.created.clone().unwrap_or("".to_string())),
            download_path: Some(self.download_path.clone().unwrap_or("".to_string())),
            changelog: Some(self.changelog.clone().unwrap_or("".to_string())),
            downloads: Some(self.downloads.unwrap_or(0)),
        };

        return out;
    }
}
