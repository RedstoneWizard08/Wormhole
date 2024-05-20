// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::mod_::ModVersion as RealModVersion;

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
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
    pub fn finish(&self) -> Self {
        ModVersion {
            friendly_version: Some(self.friendly_version.clone().unwrap_or("".to_string())),
            game_version: Some(self.friendly_version.clone().unwrap_or("".to_string())),
            id: Some(self.id.unwrap_or(0)),
            created: Some(self.created.clone().unwrap_or("".to_string())),
            download_path: Some(self.download_path.clone().unwrap_or("".to_string())),
            changelog: Some(self.changelog.clone().unwrap_or("".to_string())),
            downloads: Some(self.downloads.unwrap_or(0)),
        }
    }
}

impl From<ModVersion> for RealModVersion {
    fn from(val: ModVersion) -> Self {
        Self {
            id: format!("{}", val.id.unwrap()),
            name: val.friendly_version,
            file_name: Some(
                val.download_path
                    .clone()
                    .unwrap()
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string(),
            ),
            hash: None,
            size: None,
            url: Some(format!(
                "https://spacedock.info{}",
                val.download_path.unwrap()
            )),
        }
    }
}
