use crate::{mod_::Mod, source::ModSource};

use super::version::ModVersion;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub shared_authors: Option<Vec<SharedAuthor>>,
    pub background: Option<String>,
    pub bg_offset_y: Option<i32>,
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

        out.background = Some(self.background.clone().unwrap_or(bg_img));

        out.bg_offset_y = Some(self.bg_offset_y.unwrap_or(0));
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SharedAuthor {
    pub mod_id: i32,
    pub user_id: i32,
}

impl Into<Mod> for ModInfo {
    fn into(self) -> Mod {
        Mod {
            id: format!("{}", self.id.unwrap()),
            name: self.name.unwrap(),
            source: ModSource::SpaceDock,
            game_id: self.game_id,
            versions: self
                .versions
                .unwrap_or_default()
                .iter()
                .cloned()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
        }
    }
}
