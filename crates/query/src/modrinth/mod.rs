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
use mcmeta::{cmd::modded::ModLoader, manager::MinecraftManager};

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
        let default_loader = ModLoader::quilt_latest().await?;
        let manager =
            MinecraftManager::load_or_create(instance.data_dir(), &default_loader).await?;
        let loader = manager.loader;
        let ver = loader.mc_version();
        let mut facets = vec![
            vec![Facet::ProjectType(ProjectType::Mod)],
            vec![Facet::Versions(ver)],
        ];

        if let Some(name) = loader.name() {
            facets.push(vec![Facet::Categories(name.to_string())]);
        }

        Ok(self
            .client
            .search_paged(
                &search,
                &Sort::Relevance,
                opts.count as usize,
                (opts.page * opts.count) as usize,
                facets,
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
        let default_loader = ModLoader::quilt_latest().await?;
        let manager =
            MinecraftManager::load_or_create(instance.data_dir(), &default_loader).await?;
        let loader = manager.loader;
        let ver = loader.mc_version();

        if let Some(name) = loader.name() {
            Ok(self
                .client
                .list_versions_filtered(&id, Some(&[name]), Some(&[&ver]), None)
                .await?
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>())
        } else {
            Ok(self
                .client
                .list_versions_filtered(&id, None, Some(&[&ver]), None)
                .await?
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>())
        }
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
