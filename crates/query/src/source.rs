use anyhow::Result;
use const_format::formatcp;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use crate::mod_::{Mod, ModVersion};

pub const USER_AGENT: &str = formatcp!("Wormhole/{}", env!("CARGO_PKG_VERSION"));

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub enum ModSource {
    SpaceDock = 0,
    Ckan = 1,
    Wormhole = 2,
    Local = 3,
    CurseForge = 4,
    Modrinth = 5,
    Thunderstore = 6,
    Nexus = 7,

    #[default]
    Unknown = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
                    HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                )]))
                .user_agent(USER_AGENT)
                .build()
                .unwrap()
        } else {
            Client::builder().user_agent(USER_AGENT).build().unwrap()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// A mod source.
#[async_trait]
#[allow(private_bounds)]
pub trait Source: WithToken {
    /// Create a new instance of this source with no token.
    fn new() -> Self
    where
        Self: Sized;

    /// Create a new instance of this source with a token.
    fn new_with_token(token: String) -> Self
    where
        Self: Sized;

    /// Get the API base url.
    fn base(&self) -> String;

    /// Search for mods.
    async fn search(
        &self,
        game_id: i32,
        search: String,
        options: Option<QueryOptions>,
    ) -> Result<Paginated<Mod>>;

    /// Get a mod by its ID.
    async fn get_mod(&self, id: String) -> Result<Mod>;

    /// Get a mod's versions.
    async fn get_versions(&self, id: String) -> Result<Vec<ModVersion>>;

    /// Get a mod version by its ID.
    async fn get_version(&self, id: String, version: String) -> Result<ModVersion>;

    /// Get a mod's direct download URL.
    /// This will default to using the latest version if it isn't specified.
    async fn get_download_url(&self, id: String, version: Option<String>) -> Result<String>;
}
