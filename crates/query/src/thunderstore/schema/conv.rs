// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
            game_id: None,
            id: value.full_name,
            versions: value
                .versions
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<_>>(),
            name: value.name,
            source: Sources::Thunderstore.id(),
            icon: Some(value.versions.first().unwrap().icon.clone()),
        }
    }
}

impl From<Package> for Mod {
    fn from(value: Package) -> Self {
        Self {
            game_id: None,
            id: value.full_name,
            versions: vec![value.latest.clone().into()],
            name: value.name,
            source: Sources::Thunderstore.id(),
            icon: Some(value.latest.icon.clone()),
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
