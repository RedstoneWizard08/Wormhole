use crate::{
    mod_::{Mod, ModVersion as RealModVersion},
    source::{Paginated, QueryOptions, Resolver, WithToken},
};

use self::schema::{browse::BrowseResult, info::ModInfo, version::ModVersion};
use anyhow::Result;
use data::{
    instance::Instance,
    source::{Source, Sources},
};

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
        _instance: &Instance,
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
                "{}/search/mod?query={}&page={}&count={}&game_id={}",
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
        let url = format!("{}/mod/{}", self.base().await, id);
        let data = self.client().get(url).send().await?;
        let text = data.text().await?;

        Ok(serde_json::from_str::<ModInfo>(&text)?.into())
    }

    async fn get_versions(&self, _instance: &Instance, id: String) -> Result<Vec<RealModVersion>> {
        Ok(self.get_mod(id).await?.versions)
    }

    async fn get_version(
        &self,
        instance: &Instance,
        id: String,
        version: String,
    ) -> Result<RealModVersion> {
        self.get_versions(instance, id)
            .await?
            .iter()
            .find(|v| v.id == version)
            .cloned()
            .ok_or(anyhow!("Could not find the specified version!"))
    }

    async fn get_download_url(
        &self,
        id: String,
        instance: &Instance,
        version: Option<String>,
    ) -> Result<String> {
        if let Some(version) = version {
            Ok(format!(
                "https://spacedock.info{}",
                self.get_version(instance, id, version).await?.url.unwrap()
            ))
        } else {
            let url = format!("{}/mod/{}/latest", self.base().await, id);
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
