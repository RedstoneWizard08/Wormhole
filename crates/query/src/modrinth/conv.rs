// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
