use data::sources::Sources;
use ferinth::structures::{
    project::Project,
    search::{Response, SearchHit},
    version::Version,
};

use crate::{
    mod_::{Mod, ModVersion},
    source::Paginated,
    IntoAsync,
};

#[async_trait]
impl IntoAsync<Mod> for Project {
    async fn into_async(self) -> Mod {
        Mod {
            url: Some(format!("https://modrinth.com/mod/{}", self.id)),
            id: self.id,
            game_id: None,
            versions: Vec::new(),
            name: self.title,
            source: Sources::Modrinth.source_alt().await.unwrap().id,
            icon: self.icon_url.clone().map(|v| v.to_string()),
            author: Some(self.team),
            desc: Some(self.body),
            downloads: self.downloads as u64,
            followers: self.followers as u64,
            banner: self
                .gallery
                .iter()
                .find(|v| v.featured)
                .map(|v| Some(v.url.clone()))
                .unwrap_or(self.icon_url)
                .map(|v| v.to_string()),
        }
    }
}

#[async_trait]
impl IntoAsync<Mod> for SearchHit {
    async fn into_async(self) -> Mod {
        Mod {
            url: Some(format!("https://modrinth.com/mod/{}", self.project_id)),
            id: self.project_id,
            game_id: None,
            versions: Vec::new(),
            name: self.title,
            source: Sources::Modrinth.source_alt().await.unwrap().id,
            icon: self.icon_url.clone().map(|v| v.to_string()),
            author: Some(self.author),
            desc: Some(self.description),
            downloads: self.downloads as u64,
            followers: self.follows as u64,
            banner: self
                .featured_gallery
                .map(|v| Some(v))
                .unwrap_or(self.icon_url)
                .map(|v| v.to_string()),
        }
    }
}

impl From<Version> for ModVersion {
    fn from(val: Version) -> Self {
        let file = val.files.first().unwrap();

        Self {
            id: val.id,
            name: Some(val.name),
            file_name: Some(file.filename.clone()),
            size: Some(file.size.to_string()),
            hash: Some(file.hashes.sha1.clone()),
            url: Some(file.url.to_string()),
        }
    }
}

#[async_trait]
impl IntoAsync<Paginated<Mod>> for Response {
    async fn into_async(self) -> Paginated<Mod> {
        Paginated {
            page: Some((self.offset / self.limit) as i32),
            per_page: Some(self.limit as i32),
            pages: Some((self.total_hits / self.limit) as i32),
            data: self.hits.into_async().await,
        }
    }
}
