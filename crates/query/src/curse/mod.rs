pub mod conv;
pub mod furse;
pub mod query;

use std::env;

use crate::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken, USER_AGENT},
    IntoAsync, CURSEFORGE_KEY,
};

use anyhow::Result;
use data::{
    instance::Instance,
    source::{Source, Sources},
};
use furse::Furse;
use mcmeta::cmd::modded::ModLoader;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

use self::query::QueryResult;

pub struct CurseForge {
    client: Furse,
    token: String,
}

impl WithToken for CurseForge {
    fn get_token(&self) -> Option<String> {
        Some(self.token.clone())
    }

    fn client(&self) -> Client {
        Client::builder()
            .default_headers(HeaderMap::from_iter(vec![(
                HeaderName::from_static("x-api-key"),
                HeaderValue::from_str(&self.token).unwrap(),
            )]))
            .user_agent(USER_AGENT)
            .build()
            .unwrap()
    }
}

#[async_trait]
impl Resolver for CurseForge {
    async fn new() -> Self {
        Self {
            client: Furse::new(CURSEFORGE_KEY.clone()),
            token: CURSEFORGE_KEY.clone().into(),
        }
    }

    async fn new_with_token(token: String) -> Self {
        Self {
            client: Furse::new(&token),
            token,
        }
    }

    async fn base(&self) -> String {
        "https://api.curseforge.com/v1".into()
    }

    async fn search(
        &self,
        loader: &ModLoader,
        game_id: String,
        search: String,
        opts: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>> {
        let opts = opts.unwrap_or_default();

        let res = self
            .client()
            .get(format!("{}/mods/search", self.base().await))
            .header(
                HeaderName::from_static("x-api-key"),
                HeaderValue::from_str(&self.token).unwrap(),
            )
            .query(&[
                ("gameId", game_id),
                ("searchFilter", search),
                ("pageSize", opts.count.to_string()),
                ("index", opts.page.to_string()),
                ("sortOrder", "desc".into()),
                ("sortField", "1".into()),
                ("gameVersion", loader.mc_version()),
                ("modLoaderTypes", format!("[{}]", loader.curse_id())),
            ])
            .send()
            .await?
            .json::<QueryResult>()
            .await?;

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
        let mut it: Mod = self
            .client
            .get_mod(id.parse()?)
            .await
            .map(|v| v.into())
            .map_err(|v| anyhow!(v))?;

        it.desc = self.client.get_mod_description(id.parse()?).await.ok();

        Ok(it)
    }

    async fn get_versions(&self, loader: &ModLoader, id: String) -> Result<Vec<ModVersion>> {
        let mc = loader.mc_version();
        let loader = loader.name();

        self.client
            .get_mod_files(id.parse()?)
            .await
            .map(|v| {
                v.iter()
                    .filter(|v| {
                        v.game_versions.contains(&mc)
                            && loader
                                .map(|loader| v.game_versions.contains(&loader.to_string()))
                                .unwrap_or(true)
                    })
                    .map(|v| v.clone().into())
                    .collect::<Vec<_>>()
            })
            .map_err(|v| anyhow!(v))
    }

    async fn get_version(
        &self,
        _loader: &ModLoader,
        id: String,
        version: String,
    ) -> Result<ModVersion> {
        self.client
            .get_mod_file(id.parse()?, version.parse()?)
            .await
            .map(|v| v.into())
            .map_err(|v| anyhow!(v))
    }

    async fn get_download_url(
        &self,
        loader: &ModLoader,
        id: String,
        version: Option<String>,
    ) -> Result<String> {
        if let Some(version) = version {
            Ok(self.get_version(loader, id, version).await?.url.unwrap())
        } else {
            let ver = self
                .get_versions(loader, id.clone())
                .await?
                .first()
                .unwrap()
                .id
                .clone();

            Ok(self.get_version(loader, id, ver).await?.url.unwrap())
        }
    }

    fn source(&self) -> Source {
        Sources::CurseForge.source()
    }
}
