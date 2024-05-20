// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::{
    mod_::{Mod, ModVersion as RealModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken},
};

use self::schema::{browse::BrowseResult, info::ModInfo, version::ModVersion};
use anyhow::Result;
use data::source::{Source, Sources};

pub mod schema;

pub struct SpaceDock;

impl WithToken for SpaceDock {
    fn get_token(&self) -> Option<String> {
        None
    }
}

#[async_trait]
impl Resolver for SpaceDock {
    async fn new() -> Self {
        Self
    }

    async fn new_with_token(_token: String) -> Self {
        Self
    }

    async fn base(&self) -> String {
        String::from("https://spacedock.info/api")
    }

    async fn search(
        &self,
        game_id: String,
        search: String,
        opts: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>> {
        if search.is_empty() {
            let url = format!(
                "{}/browse?page={}&count={}&game_id={}",
                self.base().await,
                opts.unwrap_or_default().page,
                opts.unwrap_or_default().count,
                game_id
            );

            let data = self.client().get(url).send().await?;
            let text = data.text().await?;

            Ok(serde_json::from_str::<BrowseResult>(&text)?.into())
        } else {
            let url = format!(
                "{}/api/search/mod?query={}&page={}&count={}&game_id={}",
                self.base().await,
                search,
                opts.unwrap_or_default().page,
                opts.unwrap_or_default().count,
                game_id
            );

            let data = self.client().get(url).send().await?;
            let text = data.text().await?;

            Ok(serde_json::from_str::<BrowseResult>(&text)?.into())
        }
    }

    async fn get_mod(&self, id: String) -> Result<Mod> {
        let url = format!("{}/api/mod/{}", self.base().await, id);
        let data = self.client().get(url).send().await?;
        let text = data.text().await?;

        Ok(serde_json::from_str::<ModInfo>(&text)?.into())
    }

    async fn get_versions(&self, id: String) -> Result<Vec<RealModVersion>> {
        Ok(self.get_mod(id).await?.versions)
    }

    async fn get_version(&self, id: String, version: String) -> Result<RealModVersion> {
        self.get_versions(id)
            .await?
            .iter()
            .find(|v| v.id == version)
            .cloned()
            .ok_or(anyhow!("Could not find the specified version!"))
    }

    async fn get_download_url(&self, id: String, version: Option<String>) -> Result<String> {
        if let Some(version) = version {
            Ok(format!(
                "https://spacedock.info{}",
                self.get_version(id, version).await?.url.unwrap()
            ))
        } else {
            let url = format!("{}/api/mod/{}/latest", self.base().await, id);
            let data = self.client().get(url).send().await?;
            let text = data.text().await?;
            let ver = serde_json::from_str::<ModVersion>(&text)?;

            Ok(format!(
                "https://spacedock.info{}",
                ver.download_path
                    .ok_or(anyhow!("Version has no download path!"))?
            ))
        }
    }

    fn source(&self) -> Source {
        Sources::SpaceDock.source()
    }
}
