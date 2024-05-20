// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use data::source::Sources;

use super::version::ModVersion;
use crate::mod_::Mod;

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
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
        out.shared_authors = Some(self.shared_authors.clone().unwrap_or_default());

        out.background = Some(self.background.clone().unwrap_or(bg_img));

        out.bg_offset_y = Some(self.bg_offset_y.unwrap_or(0));
        out.license = Some(self.license.clone().unwrap_or("".to_string()));
        out.website = Some(self.website.clone().unwrap_or("".to_string()));
        out.donations = Some(self.donations.clone().unwrap_or("".to_string()));
        out.source_code = Some(self.source_code.clone().unwrap_or("".to_string()));
        out.url = Some(self.url.clone().unwrap_or("".to_string()));
        out.versions = Some(_versions);
        out.description = Some(self.description.clone().unwrap_or("".to_string()));

        out
    }
}

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct SharedAuthor {
    pub mod_id: i32,
    pub user_id: i32,
}

impl From<ModInfo> for Mod {
    fn from(val: ModInfo) -> Self {
        Self {
            id: format!("{}", val.id.unwrap()),
            name: val.name.unwrap(),
            source: Sources::SpaceDock.id(),
            game_id: val.game_id,
            icon: val.background,
            versions: val
                .versions
                .unwrap_or_default()
                .iter()
                .cloned()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
        }
    }
}
