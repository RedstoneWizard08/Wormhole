use crate::source::{QueryOptions, Source, WithToken};
use anyhow::Result;

use self::schema::{browse::BrowseResult, info::ModInfo, version::ModVersion};

pub mod schema;

pub struct SpaceDock;

impl WithToken for SpaceDock {
    fn get_token(&self) -> Option<String> {
        None
    }
}

#[async_trait]
impl Source for SpaceDock {
    type QueryItem = ModInfo;
    type QueryOutput = BrowseResult;
    type VersionItem = ModVersion;
    type VersionOutput = Vec<ModVersion>;

    fn new() -> Self {
        Self
    }

    fn new_with_token(_token: String) -> Self {
        Self
    }

    fn base(&self) -> String {
        String::from("https://spacedock.info/api")
    }

    async fn search(
        &self,
        game_id: i32,
        search: String,
        opts: Option<QueryOptions>,
    ) -> Result<Self::QueryOutput> {
        if search.is_empty() {
            let url = format!(
                "{}/browse?page={}&count={}&game_id={}",
                self.base(),
                opts.unwrap_or_default().page,
                opts.unwrap_or_default().count,
                game_id
            );

            let data = self.client().get(url).send().await?;
            let text = data.text().await?;

            Ok(serde_json::from_str(&text)?)
        } else {
            let url = format!(
                "{}/api/search/mod?query={}&page={}&count={}&game_id={}",
                self.base(),
                search,
                opts.unwrap_or_default().page,
                opts.unwrap_or_default().count,
                game_id
            );

            let data = self.client().get(url).send().await?;
            let text = data.text().await?;

            Ok(serde_json::from_str(&text)?)
        }
    }

    async fn get_mod(&self, id: String) -> Result<Self::QueryItem> {
        let url = format!("{}/api/mod/{}", self.base(), id);
        let data = self.client().get(url).send().await?;
        let text = data.text().await?;

        Ok(serde_json::from_str(&text)?)
    }

    async fn get_versions(&self, id: String) -> Result<Self::VersionOutput> {
        Ok(self.get_mod(id).await?.versions.unwrap_or_default())
    }

    async fn get_version(&self, id: String, version: String) -> Result<Self::VersionItem> {
        self.get_versions(id)
            .await?
            .iter()
            .find(|v| format!("{}", v.id.unwrap()) == version)
            .cloned()
            .ok_or(anyhow!("Could not find the specified version!"))
    }

    async fn get_download_url(&self, id: String, version: Option<String>) -> Result<String> {
        if let Some(version) = version {
            Ok(format!(
                "https://spacedock.info{}",
                self.get_version(id, version)
                    .await?
                    .download_path
                    .ok_or(anyhow!("Version has no download path!"))?
            ))
        } else {
            let url = format!("{}/api/mod/{}/latest", self.base(), id);
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
}
