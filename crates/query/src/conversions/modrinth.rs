use ferinth::structures::{
    project::ProjectType,
    search::{Response, SearchHit},
    version::{Version, VersionType},
};

use crate::{
    clients::modrinth::ModrinthClient,
    models::{
        pagination::Paginated,
        project::{Project, ProjectKind},
        version::{ProjectVersion, VersionKind},
        QueryClient,
    },
};

impl Into<Paginated<Project>> for (Response, u64) {
    fn into(self) -> Paginated<Project> {
        Paginated {
            current_page: ((self.0.offset / self.1 as usize) as f32).ceil() as u64,
            items_per_page: self.1,
            total_pages: ((self.0.total_hits / self.1 as usize) as f32).ceil() as u64,
            data: self.0.hits.iter().map(|v| v.clone().into()).collect(),
        }
    }
}

impl Into<Project> for SearchHit {
    fn into(self) -> Project {
        Project {
            id: self.project_id,
            author: self.author,
            banner: self.featured_gallery.map(|v| v.to_string()),
            description: None,
            summary: Some(self.description),
            downloads: Some(self.downloads as u64),
            follows: Some(self.follows as u64),
            icon: self.icon_url.map(|v| v.to_string()),
            name: self.title,
            slug: Some(self.slug.clone()),
            source_id: ModrinthClient::id().to_string(),
            url: format!("https://modrinth.com/mod/{}", self.slug),
            versions: None,
        }
    }
}

impl Into<Project> for ferinth::structures::project::Project {
    fn into(self) -> Project {
        Project {
            id: self.id,
            author: self.team,
            banner: self
                .gallery
                .iter()
                .find(|v| v.featured)
                .cloned()
                .map(|v| v.url.to_string()),
            description: Some(self.body),
            summary: Some(self.description),
            downloads: Some(self.downloads as u64),
            follows: Some(self.followers as u64),
            icon: self.icon_url.map(|v| v.to_string()),
            name: self.title,
            slug: Some(self.slug.clone()),
            source_id: ModrinthClient::id().to_string(),
            url: format!("https://modrinth.com/mod/{}", self.slug),
            versions: Some(self.versions),
        }
    }
}

impl Into<ProjectType> for ProjectKind {
    fn into(self) -> ProjectType {
        match self {
            Self::Mod => ProjectType::Mod,
            Self::Datapack => ProjectType::Datapack,
            Self::Modpack => ProjectType::Modpack,
            Self::ResourcePack => ProjectType::ResourcePack,
            Self::Shader => ProjectType::Shader,
        }
    }
}

impl Into<VersionKind> for VersionType {
    fn into(self) -> VersionKind {
        match self {
            Self::Alpha => VersionKind::Alpha,
            Self::Beta => VersionKind::Beta,
            Self::Release => VersionKind::Release,
        }
    }
}

impl Into<ProjectVersion> for Version {
    fn into(self) -> ProjectVersion {
        let file = self.files.iter().find(|v| v.primary).unwrap();
        // ^^^ I'm gonna just unwrap as Modrinth *shouldn't* accept uploading
        // versions without a primary file, and I'm gonna trust that. Will
        // totally not end up having consequences at some point.

        ProjectVersion {
            author: None, // We only have the author's ID.
            changelog: self.changelog,
            download_url: Some(file.url.to_string()),
            downloads: Some(self.downloads as u64),
            file: file.filename.clone(),
            name: self.name,
            source_id: ModrinthClient::id().into(),
            uploaded: Some(self.date_published),
            url: Some(format!(
                "https://modrinth.com/mod/{}/version/{}",
                &self.project_id, &self.id
            )),
            id: self.id,
            project_id: self.project_id,
            version_number: Some(self.version_number),
            kind: self.version_type.into(),
        }
    }
}
