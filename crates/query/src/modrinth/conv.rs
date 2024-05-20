use data::source::Sources;
use ferinth::structures::{
    project::Project,
    search::{Response, SearchHit},
    version::Version,
};

use crate::{
    mod_::{Mod, ModVersion},
    source::Paginated,
};

impl From<Project> for Mod {
    fn from(val: Project) -> Self {
        Self {
            id: val.id,
            game_id: None,
            versions: Vec::new(),
            name: val.title,
            source: Sources::Modrinth.id(),
            icon: val.icon_url.map(|v| v.to_string()),
        }
    }
}

impl From<SearchHit> for Mod {
    fn from(val: SearchHit) -> Self {
        Self {
            id: val.project_id,
            game_id: None,
            versions: Vec::new(),
            name: val.title,
            source: Sources::Modrinth.id(),
            icon: val.icon_url.map(|v| v.to_string()),
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

impl From<Response> for Paginated<Mod> {
    fn from(val: Response) -> Self {
        Self {
            page: Some((val.offset / val.limit) as i32),
            per_page: Some(val.limit as i32),
            pages: Some((val.total_hits / val.limit) as i32),
            data: val
                .hits
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>(),
        }
    }
}
