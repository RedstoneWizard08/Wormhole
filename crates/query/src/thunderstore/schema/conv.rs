use super::{
    pkg::{Package, PackageListing},
    ver::{NewPackageVersion, PackageVersion},
};
use crate::{
    mod_::{Mod, ModVersion},
    source::Paginated,
};
use data::source::Sources;

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

impl From<PackageListing> for Mod {
    fn from(value: PackageListing) -> Self {
        Self {
            url: Some(value.package_url),
            game_id: None,
            id: value.full_name,
            versions: value
                .versions
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>(),
            name: value.name,
            source: Sources::Thunderstore.id(),
            author: Some(value.owner),
            desc: None,
            downloads: 0,
            followers: 0,
            icon: Some(value.versions.first().unwrap().icon.clone()),
            banner: Some(value.versions.first().unwrap().icon.clone()),
        }
    }
}

impl From<Package> for Mod {
    fn from(value: Package) -> Self {
        Self {
            url: Some(value.package_url),
            game_id: None,
            id: value.full_name,
            versions: vec![value.latest.clone().into()],
            name: value.name,
            source: Sources::Thunderstore.id(),
            icon: Some(value.latest.icon.clone()),
            banner: Some(value.latest.icon.clone()),
            author: Some(value.owner),
            desc: None,
            downloads: value.total_downloads as u64,
            followers: value.rating_score as u64,
        }
    }
}

impl From<Vec<PackageListing>> for Paginated<Mod> {
    fn from(value: Vec<PackageListing>) -> Self {
        let data = value.iter().map(|v| v.clone().into()).collect::<Vec<_>>();

        Self {
            per_page: Some(data.len() as i32),
            data,
            page: None,
            pages: None,
        }
    }
}
