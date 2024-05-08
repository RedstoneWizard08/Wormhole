pub mod conv;

use crate::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken},
    IntoAsync,
};

use anyhow::Result;
use ckandex::{kref::KrefResolver, refresh_data, CacheClient, IdFilter, NameFilter, Query, KSP};
use data::source::{Source, Sources};
use whcore::manager::CORE_MANAGER;

pub struct Ckan {
    client: CacheClient,
    kref: KrefResolver,
}

impl WithToken for Ckan {
    fn get_token(&self) -> Option<String> {
        Some(self.kref.token.clone())
    }
}

#[async_trait]
impl Resolver for Ckan {
    async fn new() -> Self {
        let mut client = CacheClient::new(CORE_MANAGER.game_cache_dir("ckandex"));

        client.refresh().await;
        client.update_cache().await.unwrap();

        Self {
            client,
            kref: KrefResolver::new("".into()),
        }
    }

    async fn new_with_token(token: String) -> Self {
        let mut client = CacheClient::new(CORE_MANAGER.game_cache_dir("ckandex"));

        client.refresh().await;
        client.update_cache().await.unwrap();

        Self {
            client,
            kref: KrefResolver::new(token),
        }
    }

    async fn base(&self) -> String {
        String::new()
    }

    async fn search(
        &self,
        _game_id: String,
        search: String,
        _opts: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>> {
        let res = Query::new()
            .filter(NameFilter::new(search).into())
            .execute(&self.client);
        let mut out = Vec::new();

        for item in res.netkans {
            out.push(self.get_mod(item.id).await?);
        }

        for item in res.frozen {
            out.push(self.get_mod(item.id).await?);
        }

        Ok(Paginated {
            data: out.clone(),
            page: Some(0),
            per_page: Some(out.len() as i32),
        })
    }

    async fn get_mod(&self, id: String) -> Result<Mod> {
        let res = Query::new()
            .filter(IdFilter::new(id).into())
            .execute(&self.client);

        if let Ok(it) = res.first().ok_or(anyhow!("Cannot get a mod!")) {
            Ok(it.into_async().await)
        } else {
            Err(anyhow!("Cannot get a mod!"))
        }
    }

    async fn get_versions(&self, id: String) -> Result<Vec<ModVersion>> {
        Ok(self.get_mod(id).await?.versions)
    }

    async fn get_version(&self, id: String, version: String) -> Result<ModVersion> {
        self.get_versions(id)
            .await?
            .iter()
            .find(|v| v.id == version)
            .cloned()
            .ok_or(anyhow!("Could not find the specified version!"))
    }

    async fn get_download_url(&self, id: String, _version: Option<String>) -> Result<String> {
        // I can't get a list of versions with CKAN, so we just use the latest.
        Ok(self
            .get_mod(id)
            .await?
            .versions
            .first()
            .unwrap()
            .url
            .clone()
            .unwrap())
    }

    fn source(&self) -> Source {
        Sources::Ckan.source()
    }
}
