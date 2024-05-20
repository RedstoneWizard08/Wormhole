// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub mod conv;
pub mod query;

use std::env;

use crate::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken, USER_AGENT},
    IntoAsync, CURSEFORGE_KEY,
};

use anyhow::Result;
use data::source::{Source, Sources};
use furse::Furse;
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client};

use self::query::QueryResult;

pub struct CurseForge {
    client: Furse,
    token: Option<String>,
}

impl WithToken for CurseForge {
    fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    fn client(&self) -> Client {
        if let Some(token) = self.get_token() {
            Client::builder()
                .default_headers(HeaderMap::from_iter(vec![(
                    HeaderName::from_static("x-api-key"),
                    HeaderValue::from_str(&token).unwrap(),
                )]))
                .user_agent(USER_AGENT)
                .build()
                .unwrap()
        } else {
            Client::builder().user_agent(USER_AGENT).build().unwrap()
        }
    }
}

#[async_trait]
impl Resolver for CurseForge {
    async fn new() -> Self {
        Self {
            client: Furse::new(CURSEFORGE_KEY.clone()),
            token: None,
        }
    }

    async fn new_with_token(token: String) -> Self {
        Self {
            client: Furse::new(&token),
            token: Some(token),
        }
    }

    async fn base(&self) -> String {
        "https://api.curseforge.com/v1".into()
    }

    async fn search(
        &self,
        game_id: String,
        search: String,
        opts: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>> {
        let opts = opts.unwrap_or_default();

        println!("Searching for {} in {}", search, game_id);

        let res = self
            .client()
            .get(format!("{}/mods/search", self.base().await))
            .query(&[
                ("gameId", game_id),
                ("searchFilter", search),
                ("pageSize", opts.count.to_string()),
                ("index", opts.page.to_string()),
            ])
            .send()
            .await?;

        println!("{:?}", res);

        let res = serde_json::from_str::<QueryResult>(&res.text().await?)?;

        Ok(Paginated {
            data: res
                .data
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>(),
            page: Some(res.pagination.index),
            per_page: Some(res.pagination.page_size),
            pages: Some((res.pagination.total_count / res.pagination.page_size) as i32),
        })
    }

    async fn get_mod(&self, id: String) -> Result<Mod> {
        self.client
            .get_mod(id.parse()?)
            .await
            .map(|v| v.into())
            .map_err(|v| anyhow!(v))
    }

    async fn get_versions(&self, id: String) -> Result<Vec<ModVersion>> {
        self.client
            .get_mod_files(id.parse()?)
            .await
            .map(|v| v.iter().map(|v| v.clone().into()).collect::<Vec<_>>())
            .map_err(|v| anyhow!(v))
    }

    async fn get_version(&self, id: String, version: String) -> Result<ModVersion> {
        self.client
            .get_mod_file(id.parse()?, version.parse()?)
            .await
            .map(|v| v.into())
            .map_err(|v| anyhow!(v))
    }

    async fn get_download_url(&self, id: String, version: Option<String>) -> Result<String> {
        if let Some(version) = version {
            Ok(self.get_version(id, version).await?.url.unwrap())
        } else {
            let ver = self
                .get_versions(id.clone())
                .await?
                .first()
                .unwrap()
                .id
                .clone();

            Ok(self.get_version(id, ver).await?.url.unwrap())
        }
    }

    fn source(&self) -> Source {
        Sources::CurseForge.source()
    }
}
