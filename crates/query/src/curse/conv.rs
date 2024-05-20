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
use furse::structures::{file_structs::File, mod_structs::Mod};

use crate::mod_::{Mod as RealMod, ModVersion};

impl From<Mod> for RealMod {
    fn from(val: Mod) -> Self {
        Self {
            id: val.id.to_string(),
            game_id: Some(val.game_id),
            versions: val
                .latest_files
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<ModVersion>>(),
            name: val.name,
            source: Sources::CurseForge.id(),
            icon: val.logo.map(|v| v.url.to_string()),
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
