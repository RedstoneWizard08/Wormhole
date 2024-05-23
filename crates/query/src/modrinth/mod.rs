pub mod conv;

use std::env;

use crate::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken},
    IntoAsync, MODRINTH_KEY,
};

use anyhow::Result;
use data::{
    instance::Instance,
    source::{Source, Sources},
};
use ferinth::{
    structures::{
        project::{Project, ProjectType},
        search::{Facet, Sort},
        version::Version,
    },
    Ferinth,
};

pub struct Modrinth {
    client: Ferinth,
    token: Option<String>,
}

impl WithToken for Modrinth {
    fn get_token(&self) -> Option<String> {
        self.token.clone()
    }
}

#[async_trait]
impl Resolver for Modrinth {
    async fn new() -> Self {
        Self {
            client: Ferinth::new(
                env!("CARGO_CRATE_NAME"),
                Some(env!("CARGO_PKG_VERSION")),
                None,
                MODRINTH_KEY.clone(),
            )
            .unwrap(),
            token: MODRINTH_KEY.map(|v| v.to_string()),
        }
    }

    async fn new_with_token(token: String) -> Self {
        Self {
            client: Ferinth::new(
                env!("CARGO_CRATE_NAME"),
                Some(env!("CARGO_PKG_VERSION")),
                None,
                Some(&token),
            )
            .unwrap(),
            token: Some(token),
        }
    }

    async fn base(&self) -> String {
        "https://api.modrinth.com/v2".into()
    }

    async fn search(
        &self,
        _game_id: String,
        instance: &Instance,
        search: String,
        opts: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>> {
        let opts = opts.unwrap_or_default();

        Ok(self
            .client
            .search_paged(
                &search,
                &Sort::Relevance,
                opts.count as usize,
                (opts.page * opts.count) as usize,
                vec![vec![Facet::ProjectType(ProjectType::Mod)]],
            )
            .await
            .map_err(|v| anyhow!(v))?
            .into())
    }

    async fn get_mod(&self, id: String) -> Result<Mod> {
        self.client
            .get_project(&id)
            .await
            .map(|v| v.into())
            .map_err(|v| anyhow!(v))
    }

    async fn get_versions(&self, instance: &Instance, id: String) -> Result<Vec<ModVersion>> {
        let res = self
            .client()
            .get(format!("{}/project/{}/version", self.base().await, id))
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<Vec<Version>>(&res)?
            .iter()
            .map(|v| v.clone().into())
            .collect::<Vec<_>>())
    }

    async fn get_version(
        &self,
        _instance: &Instance,
        _id: String,
        version: String,
    ) -> Result<ModVersion> {
        self.client
            .get_version(&version)
            .await
            .map(|v| v.into())
            .map_err(|v| anyhow!(v))
    }

    async fn get_download_url(
        &self,
        id: String,
        instance: &Instance,
        version: Option<String>,
    ) -> Result<String> {
        if let Some(version) = version {
            Ok(self.get_version(instance, id, version).await?.url.unwrap())
        } else {
            let ver = self
                .get_versions(instance, id.clone())
                .await?
                .first()
                .unwrap()
                .id
                .clone();

            Ok(self.get_version(instance, id, ver).await?.url.unwrap())
        }
    }

    fn source(&self) -> Source {
        Sources::Modrinth.source()
    }
}
