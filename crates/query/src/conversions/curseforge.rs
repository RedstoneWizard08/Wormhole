use furse::structures::{
    common_structs::ModLoaderType,
    file_structs::{File, FileReleaseType},
    mod_structs::Mod,
    search_structs::SearchResponse,
};

use crate::{
    clients::curseforge::CurseForgeClient,
    models::{
        minecraft::ModLoader,
        pagination::Paginated,
        project::Project,
        version::{ProjectVersion, VersionKind},
        QueryClient,
    },
};

impl Into<ModLoaderType> for ModLoader {
    fn into(self) -> ModLoaderType {
        match self {
            Self::Forge => ModLoaderType::Forge,
            Self::Fabric => ModLoaderType::Fabric,
            Self::Quilt => ModLoaderType::Quilt,
            Self::NeoForge => ModLoaderType::NeoForge,
            Self::Vanilla => ModLoaderType::Any,
        }
    }
}

impl Into<Paginated<Project>> for SearchResponse {
    fn into(self) -> Paginated<Project> {
        Paginated {
            current_page: ((self.pagination.index / self.pagination.page_size) as f32).ceil()
                as u64,
            items_per_page: self.pagination.page_size as u64,
            total_pages: ((self.pagination.total_count / self.pagination.page_size) as f32).ceil()
                as u64,
            data: self.data.iter().map(|v| v.clone().into()).collect(),
        }
    }
}

impl Into<Project> for Mod {
    fn into(self) -> Project {
        Project {
            id: self.id.to_string(),
            author: self.authors.first().unwrap().name.clone(),
            banner: None,
            description: None,
            downloads: Some(self.download_count as u64),
            follows: self.thumbs_up_count.map(|v| v as u64),
            icon: self.logo.map(|v| v.url.to_string()),
            name: self.name,
            slug: Some(self.slug),
            source_id: CurseForgeClient::id().into(),
            summary: Some(self.summary),
            url: self.links.website_url.to_string(),
            versions: None,
        }
    }
}

impl Into<VersionKind> for FileReleaseType {
    fn into(self) -> VersionKind {
        match self {
            Self::Alpha => VersionKind::Alpha,
            Self::Beta => VersionKind::Beta,
            Self::Release => VersionKind::Release,
        }
    }
}

impl Into<ProjectVersion> for File {
    fn into(self) -> ProjectVersion {
        ProjectVersion {
            author: None,
            changelog: None,
            download_url: self.download_url.map(|v| v.to_string()),
            downloads: Some(self.download_count as u64),
            file: self.file_name.clone(),
            name: self.display_name,
            source_id: CurseForgeClient::id().into(),
            uploaded: Some(self.file_date),
            url: None,
            id: self.id.to_string(),
            project_id: self.mod_id.to_string(),
            version_number: None,
            kind: self.release_type.into(),
        }
    }
}
