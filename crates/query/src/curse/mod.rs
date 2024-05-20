pub mod conv;
pub mod query;

use std::env;

use crate::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken},
    IntoAsync, CURSEFORGE_KEY,
};

use anyhow::Result;
use data::source::{Source, Sources};
use furse::Furse;

use self::query::QueryResult;

pub struct CurseForge {
    client: Furse,
    token: Option<String>,
}

impl WithToken for CurseForge {
    fn get_token(&self) -> Option<String> {
        self.token.clone()
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
