use anyhow::Result;
use async_trait::async_trait;
use furse::{structures::search_structs::SearchOptions, Furse};

use crate::{
    models::{
        minecraft::MinecraftInput, pagination::Paginated, project::Project,
        version::ProjectVersion, QueryClient,
    },
    CURSEFORGE_KEY,
};

pub struct CurseForgeClient {
    furse: Furse,
}

impl CurseForgeClient {
    pub fn new(key: Option<String>) -> Self {
        Self {
            furse: Furse::new(&key.unwrap_or(CURSEFORGE_KEY.to_string())),
        }
    }
}

#[async_trait]
impl QueryClient<MinecraftInput> for CurseForgeClient {
    fn id() -> &'static str {
        "curseforge"
    }

    async fn search(
        &self,
        query: Option<String>,
        per_page: Option<u64>,
        page: u64,
        cx: &MinecraftInput,
    ) -> Result<Paginated<Project>> {
        let per_page = per_page.unwrap_or(Self::DEFAULT_PER_PAGE);

        Ok(self
            .furse
            .search_mods(SearchOptions {
                game_id: 432, // 432 is Minecraft
                search_filter: query,
                class_id: Some(cx.kind.class() as i32),
                game_version: Some(cx.mc_version.clone()),
                mod_loader_type: Some(cx.loader.into()),
                page_size: Some(per_page as i32),
                index: Some((page * per_page) as i32),
                ..Default::default()
            })
            .await?
            .into())
    }

    async fn get_project(&self, id: impl Into<String> + Send) -> Result<Project> {
        Ok(self.furse.get_mod(id.into().parse()?).await?.into())
    }

    async fn get_project_versions(
        &self,
        id: impl Into<String> + Send,
        cx: &MinecraftInput,
    ) -> Result<Vec<ProjectVersion>> {
        Ok(self
            .furse
            .get_mod_files_filtered(id.into().parse()?, cx.mc_version.clone(), cx.loader.into())
            .await?
            .iter()
            .map(|v| v.clone().into())
            .collect())
    }

    async fn get_project_version(
        &self,
        project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<ProjectVersion> {
        Ok(self
            .furse
            .get_mod_file(project_id.into().parse()?, version_id.into().parse()?)
            .await?
            .into())
    }

    async fn get_download_url(
        &self,
        project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<String> {
        Ok(self
            .furse
            .file_download_url(project_id.into().parse()?, version_id.into().parse()?)
            .await?
            .to_string())
    }

    async fn get_dependencies(
        &self,
        project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<Vec<String>> {
        Ok(self
            .furse
            .get_mod_file(project_id.into().parse()?, version_id.into().parse()?)
            .await?
            .dependencies
            .iter()
            .map(|v| v.mod_id.to_string())
            .collect())
    }
}
