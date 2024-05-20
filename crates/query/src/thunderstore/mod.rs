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
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken},
};

use anyhow::Result;
use data::source::{Source, Sources};

use self::schema::{
    pkg::{Package, PackageListing},
    ver::NewPackageVersion,
};

pub mod schema;

// Thunderstore's API sucks.
pub struct Thunderstore;

impl WithToken for Thunderstore {
    fn get_token(&self) -> Option<String> {
        None
    }
}

#[async_trait]
impl Resolver for Thunderstore {
    async fn new() -> Self {
        Self
    }

    async fn new_with_token(_token: String) -> Self {
        Self
    }

    async fn base(&self) -> String {
        String::from("https://thunderstore.io")
    }

    async fn search(
        &self,
        game_id: String,
        search: String,
        _opts: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>> {
        if search.is_empty() {
            let url = format!("{}/c/{}/api/v1/package/", self.base().await, game_id,);

            let data = self.client().get(url).send().await?;
            let text = data.text().await?;

            Ok(serde_json::from_str::<Vec<PackageListing>>(&text)?.into())
        } else {
            let url = format!("{}/c/{}/api/v1/package/", self.base().await, game_id,);

            let data = self.client().get(url).send().await?;
            let text = data.text().await?;
            let res = serde_json::from_str::<Vec<PackageListing>>(&text)?;

            let res = res
                .iter()
                .filter(|v| v.full_name.to_lowercase().contains(&search.to_lowercase()))
                .cloned()
                .collect::<Vec<_>>();

            Ok(res.into())
        }
    }

    async fn get_mod(&self, id: String) -> Result<Mod> {
        let mut spl = id.split("-").collect::<Vec<_>>();
        let ns = spl.remove(0);
        let id = spl.remove(0);
        let ver = spl
            .first()
            .cloned()
            .map(|v| format!("{}/", v))
            .unwrap_or_default();

        let url = format!(
            "{}/api/experimental/package/{}/{}/{}",
            self.base().await,
            ns,
            id,
            ver
        );
        let data = self.client().get(url).send().await?;
        let text = data.text().await?;

        Ok(serde_json::from_str::<Package>(&text)?.into())
    }

    async fn get_versions(&self, id: String) -> Result<Vec<ModVersion>> {
        Ok(self.get_mod(id).await?.versions)
    }

    async fn get_version(&self, id: String, version: String) -> Result<ModVersion> {
        let mut spl = id.split("-").collect::<Vec<_>>();
        let ns = spl.remove(0);
        let id = spl.remove(0);

        let url = format!(
            "{}/api/experimental/package/{}/{}/{}/",
            self.base().await,
            ns,
            id,
            version
        );
        let data = self.client().get(url).send().await?;
        let text = data.text().await?;

        Ok(serde_json::from_str::<NewPackageVersion>(&text)?.into())
    }

    async fn get_download_url(&self, id: String, version: Option<String>) -> Result<String> {
        if let Some(ver) = version {
            Ok(self.get_version(id, ver).await?.url.unwrap())
        } else {
            Ok(self
                .get_versions(id)
                .await?
                .first()
                .unwrap()
                .url
                .clone()
                .unwrap())
        }
    }

    fn source(&self) -> Source {
        Sources::Thunderstore.source()
    }
}
