use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ferinth::{
    structures::search::{Facet, Sort},
    Ferinth,
};

use crate::models::{
    minecraft::MinecraftInput, pagination::Paginated, project::Project, version::ProjectVersion,
    QueryClient,
};

pub struct ModrinthClient {
    ferinth: Ferinth,
}

#[async_trait]
impl QueryClient<MinecraftInput> for ModrinthClient {
    fn id() -> &'static str {
        "modrinth"
    }

    async fn search(
        &self,
        query: Option<String>,
        per_page: Option<u64>,
        page: u64,
        cx: &MinecraftInput,
    ) -> Result<Paginated<Project>> {
        let per_page = per_page.unwrap_or(Self::DEFAULT_PER_PAGE);

        Ok((
            self.ferinth
                .search_paged(
                    &query.unwrap_or_default(),
                    &Sort::Relevance,
                    per_page as usize,
                    (page * per_page) as usize,
                    vec![vec![
                        Facet::ProjectType(cx.kind.into()),
                        Facet::Versions(cx.mc_version.clone()),
                        Facet::Categories(cx.loader.as_str().into()),
                    ]],
                )
                .await?,
            per_page,
        )
            .into())
    }

    async fn get_project(&self, id: impl Into<String> + Send) -> Result<Project> {
        Ok(self.ferinth.get_project(&id.into()).await?.into())
    }

    async fn get_project_versions(
        &self,
        id: impl Into<String> + Send,
        cx: &MinecraftInput,
    ) -> Result<Vec<ProjectVersion>> {
        Ok(self
            .ferinth
            .list_versions_filtered(
                &id.into(),
                Some(&[cx.loader.as_str()]),
                Some(&[&cx.mc_version]),
                None,
            )
            .await?
            .iter()
            .map(|v| v.clone().into())
            .collect())
    }

    async fn get_project_version(
        &self,
        _project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<ProjectVersion> {
        Ok(self.ferinth.get_version(&version_id.into()).await?.into())
    }

    async fn get_download_url(
        &self,
        _project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<String> {
        Ok(self
            .ferinth
            .get_version(&version_id.into())
            .await?
            .files
            .iter()
            .find(|v| v.primary)
            .ok_or(anyhow!("Modrinth is borked and the universe is bad"))?
            .url
            .to_string())
    }

    async fn get_dependencies(
        &self,
        _project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<Vec<String>> {
        Ok(self
            .ferinth
            .get_version(&version_id.into())
            .await?
            .dependencies
            .iter()
            .filter_map(|v| v.project_id.clone())
            .collect())
    }
}
