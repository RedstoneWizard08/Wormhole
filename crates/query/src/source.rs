use std::sync::Arc;

use anyhow::Result;
use const_format::formatcp;

use data::{prisma::PrismaClient, Instance, Source};
use mcmeta::cmd::modded::ModLoader;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use crate::mod_::{Mod, ModVersion};

pub const USER_AGENT: &str = formatcp!("Wormhole/{}", env!("CARGO_PKG_VERSION"));

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type,
)]
pub struct QueryOptions {
    pub page: i32,
    pub count: i32,
}

impl Default for QueryOptions {
    fn default() -> Self {
        QueryOptions { page: 1, count: 30 }
    }
}

#[allow(unused)]
pub(crate) trait WithToken {
    fn get_token(&self) -> Option<String>;

    /// Create a client to use for requests.
    fn client(&self) -> Client {
        if let Some(token) = self.get_token() {
            Client::builder()
                .default_headers(HeaderMap::from_iter(vec![(
                    AUTHORIZATION,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Type)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub pages: Option<i32>,
}

unsafe impl<T> Send for Paginated<T> {}
unsafe impl<T> Sync for Paginated<T> {}

/// A mod source.
#[async_trait]
#[allow(private_bounds)]
pub trait Resolver: WithToken + Send + Sync {
    /// Create a new instance of this source with no token.
    async fn new() -> Self
    where
        Self: Sized;

    /// Initialize the resolver.
    /// This has a default implementation that does nothing.
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Create a new instance of this source with a token.
    async fn new_with_token(token: String) -> Self
    where
        Self: Sized;

    /// Get the API base url.
    async fn base(&self) -> String;

    /// Search for mods.
    async fn search(
        &self,
        loader: &ModLoader,
        game_id: String,
        search: String,
        options: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>>;

    /// Get a mod by its ID.
    async fn get_mod(&self, id: String) -> Result<Mod>;

    /// Get a mod's versions.
    async fn get_versions(&self, loader: &ModLoader, id: String) -> Result<Vec<ModVersion>>;

    /// Get a mod version by its ID.
    async fn get_version(
        &self,
        loader: &ModLoader,
        id: String,
        version: String,
    ) -> Result<ModVersion>;

    /// Get a mod version by its ID (option).
    async fn get_version_or_latest(
        &self,
        loader: &ModLoader,
        id: String,
        version: Option<String>,
    ) -> Result<ModVersion> {
        if let Some(ver) = version {
            self.get_version(loader, id, ver).await
        } else {
            self.get_latest_version(loader, id).await
        }
    }

    /// Get a mod's direct download URL.
    /// This will default to using the latest version if it isn't specified.
    async fn get_download_url(
        &self,
        loader: &ModLoader,
        id: String,
        version: Option<String>,
    ) -> Result<String>;

    /// Get the source's id (type).
    async fn source(&self, client: Arc<PrismaClient>) -> Source;

    /// Get the latest version.
    async fn get_latest_version(&self, loader: &ModLoader, id: String) -> Result<ModVersion> {
        self.get_versions(loader, id)
            .await?
            .first()
            .cloned()
            .ok_or(anyhow!("Cannot get a latest version!"))
    }
}
