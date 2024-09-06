pub mod minecraft;
pub mod pagination;
pub mod project;
pub mod version;

use anyhow::Result;
use async_trait::async_trait;
use pagination::Paginated;
use project::Project;
use version::ProjectVersion;

/// A query client.
#[async_trait]
pub trait QueryClient<Cx> {
    /// The default items_per_page limit.
    const DEFAULT_PER_PAGE: u64 = 50;

    /// Get the string ID for this source.
    fn id() -> &'static str;

    /// Search for projects in this source.
    ///
    /// Arguments:
    /// - query: The string query to search with.
    /// - per_page: The default items_per_page limit for paginated responses.
    /// - page: The page we are currently on.
    /// - cx: The data sent by the game provider.
    async fn search(
        &self,
        query: Option<String>,
        per_page: Option<u64>,
        page: u64,
        cx: &Cx,
    ) -> Result<Paginated<Project>>;

    /// Get a project.
    async fn get_project(&self, id: impl Into<String> + Send) -> Result<Project>;

    /// Get a project's versions.
    async fn get_project_versions(
        &self,
        id: impl Into<String> + Send,
        cx: &Cx,
    ) -> Result<Vec<ProjectVersion>>;

    /// Get a project's version.
    async fn get_project_version(
        &self,
        project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<ProjectVersion>;

    /// Get the download URL for a project's version.
    async fn get_download_url(
        &self,
        project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<String>;

    /// Get dependencies as a list of project IDs for a project's version.
    async fn get_dependencies(
        &self,
        project_id: impl Into<String> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<Vec<String>>;
}
