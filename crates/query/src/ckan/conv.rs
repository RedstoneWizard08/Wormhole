use crate::mod_::{Mod as RealMod, ModVersion};
use ckandex::Mod;
use data::{sources::Sources, Source};

use crate::IntoAsync;

#[async_trait]
impl IntoAsync<RealMod> for Mod {
    async fn into_async(mut self) -> RealMod {
        let url = self.resolve_download().await.unwrap();

        RealMod {
            game_id: None,
            id: self.id,
            name: self.name,
            source: Sources::Ckan.source_alt().await.unwrap().id,
            icon: None,
            banner: None,
            author: None,
            desc: self.description,
            downloads: 0,
            followers: 0,
            url: None,
            versions: vec![ModVersion {
                url: Some(url.clone()),
                name: None,
                file_name: Some(url.split('/').last().unwrap().into()),
                id: "-1".into(),
                hash: None,
                size: None,
            }],
        }
    }
}
