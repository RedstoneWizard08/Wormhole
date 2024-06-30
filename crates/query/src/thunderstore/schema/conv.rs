use data::sources::Sources;

use super::{
    pkg::{Package, PackageListing},
    ver::{NewPackageVersion, PackageVersion},
};
use crate::{
    mod_::{Mod, ModVersion},
    source::Paginated,
    IntoAsync,
};

impl From<PackageVersion> for ModVersion {
    fn from(value: PackageVersion) -> Self {
        Self {
            file_name: Some(format!("{}.zip", value.full_name)),
            hash: None,
            id: value.full_name,
            name: Some(value.name),
            size: Some(value.file_size.to_string()),
            url: Some(value.download_url),
        }
    }
}

impl From<NewPackageVersion> for ModVersion {
    fn from(value: NewPackageVersion) -> Self {
        Self {
            file_name: Some(format!("{}.zip", value.full_name)),
            hash: None,
            id: value.full_name,
            name: Some(value.name),
            size: None,
            url: Some(value.download_url),
        }
    }
}

#[async_trait]
impl IntoAsync<Mod> for PackageListing {
    async fn into_async(self) -> Mod {
        Mod {
            url: Some(self.package_url),
            game_id: None,
            id: self.full_name,
            versions: self
                .versions
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>(),
            name: self.name,
            source: Sources::Thunderstore.source_alt().await.unwrap().id,
            author: Some(self.owner),
            desc: None,
            downloads: 0,
            followers: 0,
            icon: Some(self.versions.first().unwrap().icon.clone()),
            banner: Some(self.versions.first().unwrap().icon.clone()),
        }
    }
}

#[async_trait]
impl IntoAsync<Mod> for Package {
    async fn into_async(self) -> Mod {
        Mod {
            url: Some(self.package_url),
            game_id: None,
            id: self.full_name,
            versions: vec![self.latest.clone().into()],
            name: self.name,
            source: Sources::Thunderstore.source_alt().await.unwrap().id,
            icon: Some(self.latest.icon.clone()),
            banner: Some(self.latest.icon.clone()),
            author: Some(self.owner),
            desc: None,
            downloads: self.total_downloads as u64,
            followers: self.rating_score as u64,
        }
    }
}

#[async_trait]
impl IntoAsync<Paginated<Mod>> for Vec<PackageListing> {
    async fn into_async(self) -> Paginated<Mod> {
        let data: Vec<Mod> = self.into_async().await;

        Paginated {
            per_page: Some(data.len() as i32),
            data,
            page: None,
            pages: None,
        }
    }
}
