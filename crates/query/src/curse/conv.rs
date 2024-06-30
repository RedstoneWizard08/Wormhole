use data::sources::Sources;

use super::furse::structures::{file_structs::File, mod_structs::Mod};
use crate::{
    mod_::{Mod as RealMod, ModVersion},
    IntoAsync,
};

#[async_trait]
impl IntoAsync<RealMod> for Mod {
    async fn into_async(self) -> RealMod {
        RealMod {
            url: Some(self.links.website_url),
            id: self.id.to_string(),
            game_id: Some(self.game_id),
            versions: self
                .latest_files
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<ModVersion>>(),
            name: self.name,
            source: Sources::CurseForge.source_alt().await.unwrap().id,
            icon: self.logo.clone().map(|v| v.url.to_string()),
            banner: self.logo.map(|v| v.url.to_string()),
            author: Some(self.authors.first().unwrap().name.clone()),
            desc: Some(self.summary),
            downloads: self.download_count as u64,
            followers: 0,
        }
    }
}

impl From<File> for ModVersion {
    fn from(val: File) -> Self {
        let hash = val.hashes.first().map(|v| v.value.clone());

        Self {
            id: val.id.to_string(),
            file_name: Some(val.file_name),
            hash,
            url: None,
            name: Some(val.display_name),
            size: Some(val.file_length.to_string()),
        }
    }
}
